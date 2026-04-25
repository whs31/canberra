use std::any::Any;

use crate::Component;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
  pub color: [f32; 4],
}

impl Default for Material {
  fn default() -> Self {
    Self {
      color: [1.0, 1.0, 1.0, 1.0],
    }
  }
}

impl Material {
  pub fn with_color(color: [f32; 4]) -> Self {
    Self { color }
  }
}

impl Component for Material {
  fn name(&self) -> &'static str {
    "Material"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn inspect(&mut self, ui: &mut egui::Ui) {
    let [r, g, b, a] = self.color;
    let swatch = egui::Color32::from_rgba_unmultiplied(
      (r * 255.0) as u8,
      (g * 255.0) as u8,
      (b * 255.0) as u8,
      (a * 255.0) as u8,
    );
    ui.horizontal(|ui| {
      let (rect, _) = ui.allocate_exact_size(egui::vec2(32.0, 16.0), egui::Sense::hover());
      ui.painter().rect_filled(rect, 3.0, swatch);
      ui.label(format!("({r:.2}, {g:.2}, {b:.2}, {a:.2})"));
    });
  }
}
