use std::sync::Arc;

use crate::{Error, Result, Scene, components::DebugUI, editor::Inspector, renderer::Renderer};

pub struct ApplicationState {
  surface: wgpu::Surface<'static>,
  pub device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  is_surface_configured: bool,
  window: Arc<winit::window::Window>,
  renderer: Renderer,
  pub scene: Scene,
  egui_ctx: egui::Context,
  egui_state: egui_winit::State,
  egui_renderer: egui_wgpu::Renderer,
  inspector: Inspector,
}

impl ApplicationState {
  pub async fn new(
    window: Arc<winit::window::Window>,
    scene_builder: Box<dyn FnOnce(&wgpu::Device) -> Scene>,
  ) -> Result<Self> {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      flags: Default::default(),
      memory_budget_thresholds: Default::default(),
      backend_options: Default::default(),
      display: None,
    });

    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await?;
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        required_limits: wgpu::Limits::default(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
      })
      .await?;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
      .formats
      .iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };

    let scene = scene_builder(&device);
    let inspector = Inspector::new();
    let renderer = Renderer::new(&device, surface_format, size.width, size.height);

    let egui_ctx = egui::Context::default();
    let egui_state = egui_winit::State::new(
      egui_ctx.clone(),
      egui::ViewportId::ROOT,
      &*window,
      Some(window.scale_factor() as f32),
      None,
      None,
    );
    let egui_renderer = egui_wgpu::Renderer::new(
      &device,
      surface_format,
      egui_wgpu::RendererOptions {
        msaa_samples: 1,
        ..Default::default()
      },
    );

    Ok(Self {
      surface,
      device,
      queue,
      config,
      is_surface_configured: false,
      window,
      renderer,
      scene,
      egui_ctx,
      egui_state,
      egui_renderer,
      inspector,
    })
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
      self.config.width = width;
      self.config.height = height;
      self.surface.configure(&self.device, &self.config);
      self.renderer.resize(&self.device, width, height);
      self.is_surface_configured = true;
    }
  }

  pub fn on_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
    self
      .egui_state
      .on_window_event(&self.window, event)
      .consumed
  }

  pub fn render(&mut self) -> Result<()> {
    self.window.request_redraw();

    if !self.is_surface_configured {
      return Ok(());
    }

    let output = match self.surface.get_current_texture() {
      wgpu::CurrentSurfaceTexture::Success(t) => t,
      wgpu::CurrentSurfaceTexture::Suboptimal(t) => {
        self.surface.configure(&self.device, &self.config);
        t
      }
      wgpu::CurrentSurfaceTexture::Timeout
      | wgpu::CurrentSurfaceTexture::Occluded
      | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
      wgpu::CurrentSurfaceTexture::Outdated => {
        self.surface.configure(&self.device, &self.config);
        return Ok(());
      }
      wgpu::CurrentSurfaceTexture::Lost => return Err(Error::LostDevice),
    };

    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    let aspect = self.config.width as f32 / self.config.height as f32;
    self
      .renderer
      .render(&self.scene, &self.queue, &view, &mut encoder, aspect);

    // --- egui ---
    let raw_input = self.egui_state.take_egui_input(&self.window);
    #[allow(deprecated)]
    let full_output = self.egui_ctx.run(raw_input, |ctx| {
      self.inspector.draw(&self.scene, ctx);
      for entity in &self.scene.entities {
        if let Some(ui) = entity.get_component::<DebugUI>() {
          ui.draw(ctx);
        }
      }
    });
    self
      .egui_state
      .handle_platform_output(&self.window, full_output.platform_output);

    let tris = self
      .egui_ctx
      .tessellate(full_output.shapes, full_output.pixels_per_point);
    for (id, delta) in &full_output.textures_delta.set {
      self
        .egui_renderer
        .update_texture(&self.device, &self.queue, *id, delta);
    }

    let screen_desc = egui_wgpu::ScreenDescriptor {
      size_in_pixels: [self.config.width, self.config.height],
      pixels_per_point: full_output.pixels_per_point,
    };
    let extra_cmds = self.egui_renderer.update_buffers(
      &self.device,
      &self.queue,
      &mut encoder,
      &tris,
      &screen_desc,
    );

    {
      let mut egui_pass = encoder
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("egui Pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],
          depth_stencil_attachment: None,
          occlusion_query_set: None,
          timestamp_writes: None,
          multiview_mask: None,
        })
        .forget_lifetime();
      self
        .egui_renderer
        .render(&mut egui_pass, &tris, &screen_desc);
    }

    for id in &full_output.textures_delta.free {
      self.egui_renderer.free_texture(id);
    }

    self.queue.submit(
      extra_cmds
        .into_iter()
        .chain(std::iter::once(encoder.finish())),
    );
    output.present();
    Ok(())
  }

  pub(crate) fn handle_key(
    &self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    code: winit::keyboard::KeyCode,
    is_pressed: bool,
  ) {
    if let (winit::keyboard::KeyCode::Escape, true) = (code, is_pressed) {
      event_loop.exit();
    }
  }

  pub(crate) fn update(&mut self) {}
}
