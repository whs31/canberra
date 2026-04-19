use glam::Mat4;

use crate::{
  Entity,
  components::{Camera, Transform},
};

pub struct Scene {
  pub entities: Vec<Entity>,
}

impl Scene {
  pub fn new() -> Self {
    Self { entities: Vec::new() }
  }

  pub fn add(&mut self, entity: Entity) {
    self.entities.push(entity);
  }

  pub fn camera_view_proj(&self, aspect: f32) -> Mat4 {
    for entity in &self.entities {
      if let Some(camera) = entity.get_component::<Camera>() {
        let mut cam = camera.clone();
        cam.aspect = aspect;
        let view = entity
          .get_component::<Transform>()
          .cloned()
          .unwrap_or_default()
          .matrix()
          .inverse();
        return cam.projection_matrix() * view;
      }
    }
    Mat4::IDENTITY
  }
}

impl Default for Scene {
  fn default() -> Self {
    Self::new()
  }
}
