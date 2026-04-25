use std::any::Any;

use glam::Mat4;

use crate::Component;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

  fn inspect(&mut self, ui: &mut egui::Ui) {
    egui::Grid::new("camera")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("FOV");
        ui.label(format!("{:.1}°", self.fov_y.to_degrees()));
        ui.end_row();
        ui.label("Near");
        ui.label(format!("{:.4}", self.near));
        ui.end_row();
        ui.label("Far");
        ui.label(format!("{:.1}", self.far));
        ui.end_row();
      });
  }
}
