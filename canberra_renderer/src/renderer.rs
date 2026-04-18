use std::sync::Arc;

use winit::window::Window;

use crate::{Result, scene::DEPTH_FORMAT};

pub struct Renderer {
  pub surface: wgpu::Surface<'static>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub surface_config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  depth_view: wgpu::TextureView,
}

fn create_depth_view(
  device: &wgpu::Device,
  config: &wgpu::SurfaceConfiguration,
) -> wgpu::TextureView {
  let tex = device.create_texture(&wgpu::TextureDescriptor {
    label: Some("depth texture"),
    size: wgpu::Extent3d {
      width: config.width.max(1),
      height: config.height.max(1),
      depth_or_array_layers: 1,
    },
    mip_level_count: 1,
    sample_count: 1,
    dimension: wgpu::TextureDimension::D2,
    format: DEPTH_FORMAT,
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    view_formats: &[],
  });
  tex.create_view(&wgpu::TextureViewDescriptor::default())
}

impl Renderer {
  pub async fn new(window: Arc<Window>) -> Result<Self> {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY, // TODO: make this configurable
      flags: Default::default(),
      memory_budget_thresholds: Default::default(),
      backend_options: Default::default(),
      display: None,
    });

    // SAFETY: surface must not outlive the window
    let surface = instance.create_surface(window.clone())?;
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
      })
      .await?;

    tracing::info!("Selected adapter: {:?}", adapter.get_info());

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("main device"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        experimental_features: Default::default(),
        memory_hints: Default::default(),
        trace: Default::default(),
      })
      .await?;
    let surface_caps = surface.get_capabilities(&adapter);
    let format = surface_caps
      .formats
      .iter()
      .find(|f| f.is_srgb())
      .copied()
      .unwrap_or(surface_caps.formats[0]);
    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format,
      width: size.width.max(1),
      height: size.height.max(1),
      present_mode: wgpu::PresentMode::AutoVsync, // todo: make this configurable
      desired_maximum_frame_latency: 2,
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &surface_config);
    let depth_view = create_depth_view(&device, &surface_config);
    Ok(Self {
      surface,
      device,
      queue,
      surface_config,
      size,
      depth_view,
    })
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width == 0 || new_size.height == 0 {
      return;
    }
    if new_size == self.size {
      return;
    }
    tracing::debug!("Resizing surface from {:?} to {:?}", self.size, new_size);
    self.size = new_size;
    self.surface_config.width = new_size.width.max(1);
    self.surface_config.height = new_size.height.max(1);
    self.surface.configure(&self.device, &self.surface_config);
    self.depth_view = create_depth_view(&self.device, &self.surface_config);
  }

  pub fn render(
    &mut self,
    window: &Window,
    scene: &crate::SceneRenderer,
    ui: &mut crate::UiRenderer,
  ) -> Result<()> {
    let frame = match self.surface.get_current_texture() {
      wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
      wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
        self.surface.configure(&self.device, &self.surface_config);
        return Ok(());
      }
      wgpu::CurrentSurfaceTexture::Timeout
      | wgpu::CurrentSurfaceTexture::Occluded
      | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
    };
    let view = frame
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    scene.update(&self.queue, self.surface_config.width, self.surface_config.height);

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame encoder") });

    scene.render(&mut encoder, &view, &self.depth_view);
    ui.render(
      &self.device,
      &self.queue,
      window,
      &mut encoder,
      &view,
      &self.surface_config,
    );

    self.queue.submit(std::iter::once(encoder.finish()));
    window.pre_present_notify();
    frame.present();
    Ok(())
  }
}
