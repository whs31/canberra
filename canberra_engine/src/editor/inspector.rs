use uuid::Uuid;

use crate::{Entity, Scene};

pub struct Inspector;

impl Inspector {
  pub fn new() -> Self {
    Self
  }

  pub fn draw(&self, selected: Option<Uuid>, scene: &Scene, ctx: &egui::Context) {
    let Some(id) = selected else {
      return;
    };
    let Some(entity) = find_entity(&scene.entities, id) else {
      return;
    };

    egui::Window::new("Inspector")
      .resizable(true)
      .min_size([260.0, 100.0])
      .show(ctx, |ui| {
        ui.heading(&entity.name);
        ui.separator();
        for component in entity.iter() {
          egui::CollapsingHeader::new(component.name())
            .default_open(true)
            .show(ui, |ui| {
              component.inspect(ui);
            });
        }
      });
  }
}

fn find_entity<'a>(entities: &'a [Entity], id: Uuid) -> Option<&'a Entity> {
  for entity in entities {
    if entity.id() == id {
      return Some(entity);
    }
    if let Some(found) = find_entity(entity.children(), id) {
      return Some(found);
    }
  }
  None
}
