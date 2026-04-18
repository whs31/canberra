use crate::Entity;

pub trait Component {
  fn name(&self) -> &'static str;
}

impl std::fmt::Debug for dyn Component {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(self.name()).finish()
  }
}
