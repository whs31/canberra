use std::any::Any;

use crate::{Component, renderer::ShaderHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ShaderKind {
  #[default]
  DefaultLit,
  DefaultUnlit,
  Custom(ShaderHandle),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
  pub color: [f32; 4],
  pub shader: ShaderKind,
}

impl Default for Material {
  fn default() -> Self {
    Self {
      color: [1.0, 1.0, 1.0, 1.0],
      shader: ShaderKind::default(),
    }
  }
}

impl Material {
  pub fn with_color(color: [f32; 4]) -> Self {
    Self {
      color,
      ..Default::default()
    }
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
    egui::Grid::new("material")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("Color");
        ui.horizontal(|ui| {
          let (rect, _) = ui.allocate_exact_size(egui::vec2(32.0, 16.0), egui::Sense::hover());
          ui.painter().rect_filled(rect, 3.0, swatch);
          ui.label(format!("({r:.2}, {g:.2}, {b:.2}, {a:.2})"));
        });
        ui.end_row();

        ui.label("Shader");
        let selected_text = match self.shader {
          ShaderKind::DefaultLit => "Default Lit".to_string(),
          ShaderKind::DefaultUnlit => "Default Unlit".to_string(),
          ShaderKind::Custom(h) => format!("Custom ({})", h.0),
        };
        egui::ComboBox::from_id_salt("mat_shader")
          .selected_text(selected_text)
          .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.shader, ShaderKind::DefaultLit, "Default Lit");
            ui.selectable_value(&mut self.shader, ShaderKind::DefaultUnlit, "Default Unlit");
          });
        ui.end_row();
      });
  }
}
