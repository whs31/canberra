use std::sync::Arc;

use crate::{Material, Mesh, ObjectUniform, Transform};

#[derive(Debug, Clone)]
pub struct Object {
  pub transform: Transform,
  pub uniform: ObjectUniform,
  pub mesh: Mesh,
  pub material: Arc<Material>,
}
