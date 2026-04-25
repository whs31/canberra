use uuid::Uuid;

use crate::{Entity, Scene};

pub struct Hierarchy {
  pub selected: Option<Uuid>,
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
        for entity in &scene.entities {
          draw_entity(entity, &mut self.selected, ui);
        }
      });
  }
}

fn draw_entity(entity: &Entity, selected: &mut Option<Uuid>, ui: &mut egui::Ui) {
  if entity.children().is_empty() {
    let is_sel = *selected == Some(entity.id());
    if ui.selectable_label(is_sel, &entity.name).clicked() {
      *selected = if is_sel { None } else { Some(entity.id()) };
    }
  } else {
    let coll_id = egui::Id::new(entity.id());
    egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), coll_id, true)
      .show_header(ui, |ui: &mut egui::Ui| {
        let is_sel = *selected == Some(entity.id());
        if ui.selectable_label(is_sel, &entity.name).clicked() {
          *selected = if is_sel { None } else { Some(entity.id()) };
        }
      })
      .body(|ui| {
        for child in entity.children() {
          draw_entity(child, selected, ui);
        }
      });
  }
}
