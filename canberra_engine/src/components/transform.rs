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

  fn inspect(&self, ui: &mut egui::Ui) {
    let euler = self.rotation.to_euler(glam::EulerRot::XYZ);
    egui::Grid::new("transform").num_columns(2).spacing([8.0, 4.0]).show(ui, |ui| {
      ui.label("Position");
      ui.label(format!(
        "{:.3},  {:.3},  {:.3}",
        self.position.x, self.position.y, self.position.z
      ));
      ui.end_row();
      ui.label("Rotation");
      ui.label(format!(
        "{:.1}°,  {:.1}°,  {:.1}°",
        euler.0.to_degrees(),
        euler.1.to_degrees(),
        euler.2.to_degrees()
      ));
      ui.end_row();
      ui.label("Scale");
      ui.label(format!(
        "{:.3},  {:.3},  {:.3}",
        self.scale.x, self.scale.y, self.scale.z
      ));
      ui.end_row();
    });
  }
}
