use crate::Scene;

pub struct Inspector {
  selected: Option<usize>,
}

impl Inspector {
  pub fn new() -> Self {
    Self { selected: None }
  }

  pub fn draw(&mut self, scene: &Scene, ctx: &egui::Context) {
    #[allow(deprecated)]
    egui::Panel::left("inspector")
      .resizable(true)
      .min_size(200.0)
      .show(ctx, |ui| {
        ui.heading("Inspector");
        ui.separator();

        ui.label(format!("{} entities", scene.entities.len()));
        ui.separator();

        for (i, entity) in scene.entities.iter().enumerate() {
          let is_selected = self.selected == Some(i);
          let response = ui.selectable_label(is_selected, &entity.name);
          if response.clicked() {
            self.selected = if is_selected { None } else { Some(i) };
          }

          if is_selected {
            ui.indent(i, |ui| {
              for component in entity.components() {
                ui.label(format!("  \u{2022} {}", component.name()));
              }
            });
          }
        }
      });
  }
}
