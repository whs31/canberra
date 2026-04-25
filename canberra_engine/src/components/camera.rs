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

#[typetag::serde]
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
    const DRAG_WIDTH: f32 = 60.0; 

    egui::Grid::new("camera")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("FOV");
        let mut fov_deg = self.fov_y.to_degrees();
        if ui
          .add_sized(
            [DRAG_WIDTH, ui.available_height()],
            egui::DragValue::new(&mut fov_deg)
              .suffix("°")
              .speed(0.1)
              .max_decimals(1)
              .range(1.0..=179.0),
          )
          .changed()
        {
          self.fov_y = fov_deg.to_radians();
        }
        ui.end_row();

        ui.label("Near");
        ui.add_sized(
          [DRAG_WIDTH, ui.available_height()],
          egui::DragValue::new(&mut self.near)
            .speed(0.01)
            .max_decimals(3)
            .range(0.001..=self.far),
        );
        ui.end_row();

        ui.label("Far");
        ui.add_sized(
          [DRAG_WIDTH, ui.available_height()],
          egui::DragValue::new(&mut self.far)
            .speed(1.0)
            .max_decimals(1)
            .range(self.near..=10000.0),
        );
        ui.end_row();
      });
  }
}
