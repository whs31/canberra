use std::any::Any;

#[typetag::serde(tag = "type")]
pub trait Component: Any + std::fmt::Debug {
  fn name(&self) -> &'static str;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn inspect(&mut self, ui: &mut egui::Ui) {
    ui.label(self.name());
  }
}
