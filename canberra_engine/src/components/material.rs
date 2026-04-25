use std::any::Any;

use crate::Component;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
  pub color: [f32; 4],
}

impl Default for Material {
  fn default() -> Self {
    Self {
      color: [1.0, 1.0, 1.0, 1.0],
    }
  }
}

impl Material {
  pub fn with_color(color: [f32; 4]) -> Self {
    Self { color }
  }
}

impl Component for Material {
  fn name(&self) -> &'static str {
    "Material"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
