mod state;
use std::sync::Arc;

pub use self::state::ApplicationState;
use crate::{Result, Scene};

pub struct Application {
  pub state: Option<ApplicationState>,
  scene_builder: Option<Box<dyn FnOnce(&wgpu::Device) -> Scene>>,
}

impl Application {
  pub fn run<F: FnOnce(&wgpu::Device) -> Scene + 'static>(scene_builder: F) -> Result<()> {
    let event_loop = crate::window::event_loop()?;
    let mut app = Self {
      state: None,
      scene_builder: Some(Box::new(scene_builder)),
    };
    event_loop.run_app(&mut app)?;
    Ok(())
  }
}

impl winit::application::ApplicationHandler for Application {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window_attributes = winit::window::Window::default_attributes();
    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
    let builder = self.scene_builder.take().unwrap();
    self.state = Some(pollster::block_on(ApplicationState::new(window, builder)).unwrap());
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    let state = match &mut self.state {
      Some(s) => s,
      None => return,
    };

    state.on_window_event(&event);

    match event {
      winit::event::WindowEvent::CloseRequested => event_loop.exit(),
      winit::event::WindowEvent::Resized(size) => state.resize(size.width, size.height),
      winit::event::WindowEvent::RedrawRequested => {
        state.update();
        match state.render() {
          Ok(_) => {}
          Err(e) => {
            tracing::error!("fatal: {e}");
            event_loop.exit();
          }
        }
      }
      winit::event::WindowEvent::KeyboardInput {
        event:
          winit::event::KeyEvent {
            physical_key: winit::keyboard::PhysicalKey::Code(code),
            state: key_state,
            ..
          },
        ..
      } => state.handle_key(event_loop, code, key_state.is_pressed()),
      _ => {}
    }
  }
}
