use std::{any::TypeId, collections::HashMap};

use serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};
use uuid::Uuid;

use super::component_data::ComponentData;
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

// ── serde ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct SerializedEntity {
  id: Uuid,
  name: String,
  components: Vec<ComponentData>,
}

impl Serialize for Entity {
  fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
    let components: Vec<ComponentData> = self
      .components
      .values()
      .filter_map(|c| ComponentData::from_component(c.as_ref()))
      .collect();
    SerializedEntity {
      id: self.id,
      name: self.name.clone(),
      components,
    }
    .serialize(s)
  }
}

impl<'de> Deserialize<'de> for Entity {
  fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
    let raw = SerializedEntity::deserialize(d)?;
    let mut components: HashMap<TypeId, Box<dyn Component>> =
      HashMap::with_capacity(raw.components.len());
    for cd in raw.components {
      let (tid, c) = cd.into_keyed_component();
      components.insert(tid, c);
    }
    Ok(Self {
      id: raw.id,
      name: raw.name,
      components,
    })
  }
}
