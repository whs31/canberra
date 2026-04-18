mod app;
mod error;
pub mod window;

pub use self::{
  app::{Application, ApplicationState},
  error::{Error, Result},
};

fn try_main() -> Result<()> {
  let event_loop = window::event_loop()?;
  let mut app = Application::new()?;
  event_loop.run_app(&mut app)?;

  Ok(())
}

fn main() {
  canberra_core::logging::init();

  if let Err(err) = try_main() {
    tracing::error!("Fatal: {}", err);
    std::process::exit(1);
  }
}
