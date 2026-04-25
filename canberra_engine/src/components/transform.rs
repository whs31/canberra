use std::any::Any;

use glam::{Mat4, Quat, Vec3};

use crate::Component;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    Self {
      position,
      ..Default::default()
    }
  }

  pub fn matrix(&self) -> Mat4 {
    Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
  }
}

#[typetag::serde]
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

  fn inspect(&mut self, ui: &mut egui::Ui) {
    const X: egui::Color32 = egui::Color32::from_rgb(210, 70, 70);
    const Y: egui::Color32 = egui::Color32::from_rgb(70, 190, 70);
    const Z: egui::Color32 = egui::Color32::from_rgb(70, 110, 210);
    const DRAG_WIDTH: f32 = 50.0;

    egui::Grid::new("transform")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("Position");
        ui.horizontal(|ui| {
          for (val, color) in [
            (&mut self.position.x, X),
            (&mut self.position.y, Y),
            (&mut self.position.z, Z),
          ] {
            ui.colored_label(
              color,
              if color == X {
                "X"
              } else if color == Y {
                "Y"
              } else {
                "Z"
              },
            );
            ui.add_sized(
              [DRAG_WIDTH, ui.available_height()],
              egui::DragValue::new(val).max_decimals(1).speed(0.1),
            );
          }
        });
        ui.end_row();

        ui.label("Rotation");
        let (mut ex, mut ey, mut ez) = self.rotation.to_euler(glam::EulerRot::XYZ);
        ex = ex.to_degrees();
        ey = ey.to_degrees();
        ez = ez.to_degrees();

        ui.horizontal(|ui| {
          let axes = [(&mut ex, X, "X"), (&mut ey, Y, "Y"), (&mut ez, Z, "Z")];
          for (val, color, label) in axes {
            ui.colored_label(color, label);
            ui.add_sized(
              [DRAG_WIDTH, ui.available_height()],
              egui::DragValue::new(val)
                .max_decimals(1)
                .speed(0.5)
                .suffix("°"),
            );
          }
        });
        self.rotation = Quat::from_euler(
          glam::EulerRot::XYZ,
          ex.to_radians(),
          ey.to_radians(),
          ez.to_radians(),
        );
        ui.end_row();

        ui.label("Scale");
        ui.horizontal(|ui| {
          for (val, color) in [
            (&mut self.scale.x, X),
            (&mut self.scale.y, Y),
            (&mut self.scale.z, Z),
          ] {
            ui.colored_label(
              color,
              if color == X {
                "X"
              } else if color == Y {
                "Y"
              } else {
                "Z"
              },
            );
            ui.add_sized(
              [DRAG_WIDTH, ui.available_height()],
              egui::DragValue::new(val).max_decimals(1).speed(0.01),
            );
          }
        });
        ui.end_row();
      });
  }
}
