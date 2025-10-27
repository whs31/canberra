use anyhow::Result;
use app::api::Application;
pub(crate) use canberra_app as app;

fn main() -> Result<()> {
  app::logger::Logger::try_with_env_or_str("trace")
    .unwrap_or_else(|_e| app::logger::Logger::with(app::logger::LogSpecification::trace()))
    .log_to_stderr()
    .start()
    .ok();

  let application = app::Application::new();
  application.run()
}
