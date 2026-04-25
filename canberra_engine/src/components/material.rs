use std::any::Any;

use crate::{
  Component,
  renderer::{GLOBAL_SHADER_REGISTRY, ShaderHandle},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
  pub color: [f32; 4],
  pub shader: ShaderHandle,
}

impl Default for Material {
  fn default() -> Self {
    Self {
      color: [1.0, 1.0, 1.0, 1.0],
      shader: ShaderHandle::default(),
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

        let registry = GLOBAL_SHADER_REGISTRY.load();
        let selected_text = registry
          .get(self.shader)
          .map(|s| s.name.clone())
          .unwrap_or_else(|| format!("Unknown ({})", self.shader.0));

        egui::ComboBox::from_id_salt("mat_shader")
          .selected_text(selected_text)
          .show_ui(ui, |ui| {
            let mut handles: Vec<_> = registry.shaders.keys().collect();
            handles.sort_by_key(|h| h.0);

            for (handle, shader) in &registry.shaders {
              ui.selectable_value(&mut self.shader, *handle, &shader.name);
            }
          });
        ui.end_row();
      });
  }
}
