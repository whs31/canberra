use crate::Component;

#[derive(Debug, Clone)]
pub struct Material {}

impl Component for Material {
  fn name(&self) -> &'static str {
    "Material"
  }
}
