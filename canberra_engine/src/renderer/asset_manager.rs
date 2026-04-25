use std::{
  collections::{
    HashMap,
    hash_map::{DefaultHasher, Entry},
  },
  hash::{Hash, Hasher},
};

use super::GpuMesh;
use crate::components::Mesh;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(u64);

impl MeshHandle {
  fn from_mesh(mesh: &Mesh) -> Self {
    let mut h = DefaultHasher::new();
    bytemuck::cast_slice::<_, u8>(&mesh.vertices).hash(&mut h);
    bytemuck::cast_slice::<_, u8>(&mesh.indices).hash(&mut h);
    Self(h.finish())
  }
}

pub struct AssetManager {
  meshes: HashMap<MeshHandle, GpuMesh>,
}

impl AssetManager {
  pub fn new() -> Self {
    Self {
      meshes: HashMap::with_capacity(64),
    }
  }

  /// Returns the handle for `mesh`, uploading it to the GPU exactly once.
  pub(crate) fn get_or_upload(
    &mut self,
    device: &wgpu::Device,
    mesh: &Mesh,
  ) -> (MeshHandle, &GpuMesh) {
    let handle = MeshHandle::from_mesh(mesh);
    let gpu_mesh = match self.meshes.entry(handle) {
      Entry::Occupied(e) => e.into_mut(),
      Entry::Vacant(e) => e.insert(GpuMesh::upload(device, mesh)),
    };
    (handle, gpu_mesh)
  }

  #[allow(dead_code)]
  pub(crate) fn get(&self, handle: MeshHandle) -> Option<&GpuMesh> {
    self.meshes.get(&handle)
  }

  pub fn remove(&mut self, handle: MeshHandle) -> bool {
    self.meshes.remove(&handle).is_some()
  }

  pub fn len(&self) -> usize {
    self.meshes.len()
  }

  pub fn is_empty(&self) -> bool {
    self.meshes.is_empty()
  }
}

impl Default for AssetManager {
  fn default() -> Self {
    Self::new()
  }
}
