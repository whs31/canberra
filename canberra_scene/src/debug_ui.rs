#[derive(Debug, Default)]
pub struct DebugUi;

impl DebugUi {
  pub fn new() -> Self {
    Self
  }

  pub fn show(&mut self, ctx: &egui::Context) {
    egui::Window::new("hello").show(ctx, |ui| {
      ui.label("hello world");
      ui.label("map engine v0.1");
    });
  }
}
