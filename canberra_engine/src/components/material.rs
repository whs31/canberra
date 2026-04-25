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

#[typetag::serde]
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
    egui::Grid::new("material")
      .num_columns(2)
      .spacing([8.0, 4.0])
      .show(ui, |ui| {
        ui.label("Color");
        ui.color_edit_button_rgba_unmultiplied(&mut self.color);
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
