use anyhow::Result;
use app::api::Application;
pub(crate) use canberra_app as app;

fn main() -> Result<()> {
  let application = app::Application::new();
  application.run()
}
