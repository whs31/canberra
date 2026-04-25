use std::any::TypeId;

use serde::{Deserialize, Serialize};

use crate::{
  Component,
  components::{Camera, Material, Mesh, Transform},
};

/// Serializable representation of every component variant.
/// `DebugUI` (closure) is intentionally omitted — it is runtime-only and
/// will be silently dropped when an entity is round-tripped through serde.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ComponentData {
  Transform(Transform),
  Camera(Camera),
  Material(Material),
  Mesh(Mesh),
}

impl ComponentData {
  /// Try to snapshot a live component as `ComponentData`.
  /// Returns `None` for types that have no serializable representation.
  pub(crate) fn from_component(c: &dyn Component) -> Option<Self> {
    let any = c.as_any();
    if let Some(t) = any.downcast_ref::<Transform>() {
      return Some(Self::Transform(t.clone()));
    }
    if let Some(t) = any.downcast_ref::<Camera>() {
      return Some(Self::Camera(t.clone()));
    }
    if let Some(t) = any.downcast_ref::<Material>() {
      return Some(Self::Material(t.clone()));
    }
    if let Some(t) = any.downcast_ref::<Mesh>() {
      return Some(Self::Mesh(t.clone()));
    }
    None
  }

  /// Consume into a `(TypeId, Box<dyn Component>)` pair suitable for insertion
  /// into `Entity::components`.
  pub(crate) fn into_keyed_component(self) -> (TypeId, Box<dyn Component>) {
    match self {
      Self::Transform(c) => (TypeId::of::<Transform>(), Box::new(c)),
      Self::Camera(c) => (TypeId::of::<Camera>(), Box::new(c)),
      Self::Material(c) => (TypeId::of::<Material>(), Box::new(c)),
      Self::Mesh(c) => (TypeId::of::<Mesh>(), Box::new(c)),
    }
  }
}
