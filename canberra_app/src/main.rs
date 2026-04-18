mod error;

pub use self::error::{Error, Result};

fn try_main() -> Result<()> {
  canberra_engine::Application::run()?;
  Ok(())
}

fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  if let Err(err) = try_main() {
    tracing::error!("Fatal: {}", err);
    std::process::exit(1);
  }
}
