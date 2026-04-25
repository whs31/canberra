use uuid::Uuid;

use crate::{Entity, Scene};

pub struct Inspector;

impl Inspector {
  pub fn new() -> Self {
    Self
  }

  pub fn draw(&self, selected: Option<Uuid>, scene: &mut Scene, ctx: &egui::Context) {
    let Some(id) = selected else {
      return;
    };
    let Some(entity) = find_entity_mut(&mut scene.entities, id) else {
      return;
    };

    let name = entity.name.clone();
    egui::Window::new("Inspector")
      .resizable(true)
      .min_size([260.0, 100.0])
      .show(ctx, |ui| {
        ui.heading(&name);
        ui.separator();
        for component in entity.iter_mut() {
          let cname = component.name();
          egui::CollapsingHeader::new(cname)
            .default_open(true)
            .show(ui, |ui| {
              component.inspect(ui);
            });
        }
      });
  }
}

fn find_entity_mut<'a>(entities: &'a mut [Entity], id: Uuid) -> Option<&'a mut Entity> {
  // Two-pass: check this level first (immutable scan for index), then recurse into children.
  if let Some(idx) = entities.iter().position(|e| e.id() == id) {
    return Some(&mut entities[idx]);
  }
  for entity in entities.iter_mut() {
    if let Some(found) = find_entity_mut(entity.children_mut(), id) {
      return Some(found);
    }
  }
  None
}
