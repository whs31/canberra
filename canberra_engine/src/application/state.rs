use std::sync::Arc;

use crate::{Error, Result, Scene, renderer::Renderer};

pub struct ApplicationState {
  surface: wgpu::Surface<'static>,
  pub device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  is_surface_configured: bool,
  window: Arc<winit::window::Window>,
  renderer: Renderer,
  pub scene: Scene,
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
    let renderer = Renderer::new(&device, surface_format, size.width, size.height);

    Ok(Self {
      surface,
      device,
      queue,
      config,
      is_surface_configured: false,
      window,
      renderer,
      scene,
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

    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    let aspect = self.config.width as f32 / self.config.height as f32;
    self.renderer.render(&self.scene, &self.queue, &view, &mut encoder, aspect);

    self.queue.submit(std::iter::once(encoder.finish()));
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
