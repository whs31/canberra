use crate::{
  Component,
  components::{Material, Mesh, Transform},
};

pub trait Entity {
  fn transform(&self) -> &Transform;
  fn transform_mut(&mut self) -> &mut Transform;

  fn mesh(&self) -> Option<&Mesh>;
  fn mesh_mut(&mut self) -> Option<&mut Mesh>;

  fn material(&self) -> Option<&Material>;
  fn material_mut(&mut self) -> Option<&mut Material>;

  fn components(&self) -> &[Box<dyn Component>];
  fn components_mut(&mut self) -> &mut [Box<dyn Component>];
}
