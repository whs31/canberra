use std::sync::Arc;

use canberra_renderer::{Renderer, SceneRenderer, UiRenderer};
use winit::{
  event::{DeviceEvent, DeviceId, WindowEvent},
  event_loop::ActiveEventLoop,
  window::WindowId,
};

use crate::Result;

pub enum Application {
  Uninitialized(tokio::runtime::Runtime),
  Ready(ApplicationState),
  Shutdown,
}

pub struct ApplicationState {
  pub rt: tokio::runtime::Runtime,
  pub window: Arc<winit::window::Window>,
  pub renderer: Renderer,
  pub scene: SceneRenderer,
  pub ui: UiRenderer,
}

impl Application {
  pub fn new() -> Result<Self> {
    let rt = tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()?;
    Ok(Self::new_with_runtime(rt))
  }

  pub fn new_with_runtime(rt: tokio::runtime::Runtime) -> Self {
    Self::Uninitialized(rt)
  }

  fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
    let Self::Uninitialized(rt) = self else { return Ok(()) };

    let window_attrs = winit::window::Window::default_attributes()
      .with_title("canberra")
      .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));
    let window = Arc::new(event_loop.create_window(window_attrs)?);

    let renderer = rt.block_on(Renderer::new(window.clone()))?;
    let scene = SceneRenderer::new(&renderer.device, renderer.surface_config.format);
    let ui = UiRenderer::new(&renderer.device, renderer.surface_config.format, &window);

    let Self::Uninitialized(rt) = std::mem::replace(
      self,
      Self::Uninitialized(tokio::runtime::Builder::new_current_thread().build()?),
    ) else {
      unreachable!("checked above");
    };

    *self = Self::Ready(ApplicationState { rt, window, renderer, scene, ui });
    Ok(())
  }
}

impl winit::application::ApplicationHandler for Application {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    if let Err(err) = self.init(event_loop) {
      tracing::error!("Failed to initialize application: {err}");
      event_loop.exit();
    }
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    let Application::Ready(state) = self else { return };

    if state.ui.handle_event(&state.window, &event) {
      state.window.request_redraw();
      return;
    }

    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(new_size) => {
        state.renderer.resize(new_size);
        state.window.request_redraw();
      }
      WindowEvent::RedrawRequested => {
        if let Err(err) = state.renderer.render(&state.window, &state.scene, &mut state.ui) {
          tracing::error!("render error: {err}");
        }
        state.window.request_redraw();
      }
      _ => {}
    }
  }

  fn device_event(
    &mut self,
    _event_loop: &ActiveEventLoop,
    _device_id: DeviceId,
    event: DeviceEvent,
  ) {
    if let DeviceEvent::MouseMotion { delta: _ } = event {
      // todo: handle mouse motion
    }
  }

  fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
    // todo: drop the surface here
  }

  fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
    // Drop the window and wgpu surface while the event loop (and its wayland/X11
    // connection) is still alive — otherwise `wl_proxy_destroy` on the window
    // segfaults when the display connection has already been torn down.
    *self = Self::Shutdown;
  }
}
