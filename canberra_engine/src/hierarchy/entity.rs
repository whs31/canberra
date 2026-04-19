use uuid::Uuid;

use crate::Component;

#[derive(Debug)]
pub struct Entity {
  id: Uuid,
  pub name: String,
  components: Vec<Box<dyn Component>>,
}

impl Entity {
  pub fn new(name: &str) -> Self {
    Self {
      id: Uuid::new_v4(),
      name: name.to_string(),
      components: Vec::new(),
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
    self.components.push(Box::new(component));
    self
  }

  pub fn get_component<C: 'static>(&self) -> Option<&C> {
    self.components.iter().find_map(|c| c.as_any().downcast_ref::<C>())
  }

  pub fn get_component_mut<C: 'static>(&mut self) -> Option<&mut C> {
    self.components.iter_mut().find_map(|c| c.as_any_mut().downcast_mut::<C>())
  }

  pub fn components(&self) -> &[Box<dyn Component>] {
    &self.components
  }
}
