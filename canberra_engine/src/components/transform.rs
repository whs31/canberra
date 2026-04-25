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

    egui::Grid::new("transform")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("Position");
        ui.horizontal(|ui| {
          ui.colored_label(X, "X");
          ui.add(egui::DragValue::new(&mut self.position.x).speed(0.1));
          ui.colored_label(Y, "Y");
          ui.add(egui::DragValue::new(&mut self.position.y).speed(0.1));
          ui.colored_label(Z, "Z");
          ui.add(egui::DragValue::new(&mut self.position.z).speed(0.1));
        });
        ui.end_row();

        ui.label("Rotation");
        let euler = self.rotation.to_euler(glam::EulerRot::XYZ);
        let mut ex = euler.0.to_degrees();
        let mut ey = euler.1.to_degrees();
        let mut ez = euler.2.to_degrees();
        ui.horizontal(|ui| {
          ui.colored_label(X, "X");
          ui.add(egui::DragValue::new(&mut ex).speed(0.5).suffix("°"));
          ui.colored_label(Y, "Y");
          ui.add(egui::DragValue::new(&mut ey).speed(0.5).suffix("°"));
          ui.colored_label(Z, "Z");
          ui.add(egui::DragValue::new(&mut ez).speed(0.5).suffix("°"));
        });
        self.rotation = glam::Quat::from_euler(
          glam::EulerRot::XYZ,
          ex.to_radians(),
          ey.to_radians(),
          ez.to_radians(),
        );
        ui.end_row();

        ui.label("Scale");
        ui.horizontal(|ui| {
          ui.colored_label(X, "X");
          ui.add(egui::DragValue::new(&mut self.scale.x).speed(0.01));
          ui.colored_label(Y, "Y");
          ui.add(egui::DragValue::new(&mut self.scale.y).speed(0.01));
          ui.colored_label(Z, "Z");
          ui.add(egui::DragValue::new(&mut self.scale.z).speed(0.01));
        });
        ui.end_row();
      });
  }
}
