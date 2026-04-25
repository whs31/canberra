use crate::Scene;

pub struct Hierarchy {
  pub selected: Option<usize>,
}

impl Hierarchy {
  pub fn new() -> Self {
    Self { selected: None }
  }

  pub fn draw(&mut self, scene: &Scene, ctx: &egui::Context) {
    egui::Window::new("Hierarchy")
      .resizable(true)
      .min_size([200.0, 200.0])
      .show(ctx, |ui| {
        ui.label(format!("{} entities", scene.entities.len()));
        ui.separator();

        for (i, entity) in scene.entities.iter().enumerate() {
          let is_selected = self.selected == Some(i);
          let response = ui.selectable_label(is_selected, &entity.name);
          if response.clicked() {
            self.selected = if is_selected { None } else { Some(i) };
          }
        }
      });
  }
}
