use glam::Mat4;

#[derive(Debug, Clone)]
pub struct Transform {
  pub matrix: Mat4,
}

impl Transform {
  pub fn new() -> Self {
    Self {
      matrix: Mat4::IDENTITY,
    }
  }
}
