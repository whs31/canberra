use std::any::Any;

use glam::Mat4;

use crate::Component;

#[derive(Debug, Clone)]
pub struct Camera {
  pub fov_y: f32,
  pub aspect: f32,
  pub near: f32,
  pub far: f32,
}

impl Camera {
  pub fn new(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
    Self {
      fov_y,
      aspect,
      near,
      far,
    }
  }

  pub fn projection_matrix(&self) -> Mat4 {
    Mat4::perspective_rh(self.fov_y, self.aspect, self.near, self.far)
  }
}

impl Component for Camera {
  fn name(&self) -> &'static str {
    "Camera"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
