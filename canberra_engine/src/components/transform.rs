use std::any::Any;

use glam::{Mat4, Quat, Vec3};

use crate::Component;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
  pub position: Vec3,
  pub rotation: Quat,
  pub scale: Vec3,
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      position: Vec3::ZERO,
      rotation: Quat::IDENTITY,
      scale: Vec3::ONE,
    }
  }
}

impl Transform {
  pub fn from_translation(position: Vec3) -> Self {
    Self { position, ..Default::default() }
  }

  pub fn matrix(&self) -> Mat4 {
    Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
  }
}

impl Component for Transform {
  fn name(&self) -> &'static str {
    "Transform"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
