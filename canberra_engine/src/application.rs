mod state;
use std::sync::Arc;

pub use self::state::ApplicationState;
use crate::Result;

#[derive(Debug)]
pub struct Application {
  pub state: Option<ApplicationState>,
}

impl Application {
  pub fn new() -> Self {
    Self { state: None }
  }

  pub fn run() -> Result<()> {
    let event_loop = winit::event_loop::EventLoop::with_user_event().build()?;
    let mut app = Self::new();
    event_loop.run_app(&mut app)?;
    Ok(())
  }
}

impl winit::application::ApplicationHandler for Application {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    #[allow(unused_mut)]
    let mut window_attributes = winit::window::Window::default_attributes();

    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
    self.state = Some(pollster::block_on(ApplicationState::new(window)).unwrap());
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    let state = match &mut self.state {
      Some(canvas) => canvas,
      None => return,
    };

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
