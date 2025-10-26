use anyhow::Result;

pub trait Application {
  fn run(self) -> Result<()>; 
}
