use wgpu::util::DeviceExt;

use crate::components::Mesh;

#[derive(Debug, Clone)]
pub(crate) struct GpuMesh {
  pub(crate) vertex_buffer: wgpu::Buffer,
  pub(crate) index_buffer: wgpu::Buffer,
  pub(crate) index_count: u32,
}

impl GpuMesh {
  pub(crate) fn upload(device: &wgpu::Device, mesh: &Mesh) -> Self {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Mesh Vertex Buffer"),
      contents: bytemuck::cast_slice(&mesh.vertices),
      usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Mesh Index Buffer"),
      contents: bytemuck::cast_slice(&mesh.indices),
      usage: wgpu::BufferUsages::INDEX,
    });

    Self {
      vertex_buffer,
      index_buffer,
      index_count: mesh.indices.len() as u32,
    }
  }
}
