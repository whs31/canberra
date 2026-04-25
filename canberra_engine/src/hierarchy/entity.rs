use std::{any::TypeId, collections::HashMap};

use uuid::Uuid;

use crate::Component;

#[derive(Debug)]
pub struct Entity {
  id: Uuid,
  pub name: String,
  components: HashMap<TypeId, Box<dyn Component>>,
}

impl Entity {
  pub fn new(name: &str) -> Self {
    Self {
      id: Uuid::new_v4(),
      name: name.to_string(),
      components: HashMap::new(),
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn add_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
    self
      .components
      .insert(TypeId::of::<C>(), Box::new(component));
    self
  }

  pub fn get_component<C: 'static>(&self) -> Option<&C> {
    self
      .components
      .get(&TypeId::of::<C>())
      .and_then(|c| c.as_any().downcast_ref::<C>())
  }

  pub fn get_component_mut<C: 'static>(&mut self) -> Option<&mut C> {
    self
      .components
      .get_mut(&TypeId::of::<C>())
      .and_then(|c| c.as_any_mut().downcast_mut::<C>())
  }

  pub fn iter(&self) -> impl Iterator<Item = &dyn Component> {
    self.components.iter().map(|(_, b)| b.as_ref())
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut dyn Component> {
    self.components.iter_mut().map(|(_, b)| b.as_mut())
  }
}
