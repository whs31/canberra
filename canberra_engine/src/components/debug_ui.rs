use std::any::Any;

use crate::Component;

pub struct DebugUI {
  draw_fn: Box<dyn Fn(&egui::Context) + 'static>,
}

impl DebugUI {
  pub fn new<F: Fn(&egui::Context) + 'static>(f: F) -> Self {
    Self {
      draw_fn: Box::new(f),
    }
  }

  pub fn draw(&self, ctx: &egui::Context) {
    (self.draw_fn)(ctx);
  }
}

impl std::fmt::Debug for DebugUI {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "DebugUI")
  }
}

impl Component for DebugUI {
  fn name(&self) -> &'static str {
    "DebugUI"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
