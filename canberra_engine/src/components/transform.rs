use glam::Mat4;

use crate::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
  pub matrix: Mat4,
}

impl Component for Transform {
  fn name(&self) -> &'static str {
    "Transform"
  }
}
