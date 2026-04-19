use std::any::Any;

use wgpu::util::DeviceExt;

use crate::{Component, Vertex};

#[derive(Debug, Clone)]
pub struct Mesh {
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub index_count: u32,
}

impl Component for Mesh {
  fn name(&self) -> &'static str {
    "Mesh"
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Mesh {
  pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
    Self {
      vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mesh vertex buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
      }),
      index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("mesh index buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
      }),
      index_count: indices.len() as u32,
    }
  }

  pub fn default_cube(device: &wgpu::Device) -> Self {
    Self::new(device, &CUBE_VERTICES, &CUBE_INDICES)
  }
}

#[rustfmt::skip]
const CUBE_VERTICES: &[Vertex] = &[
  Vertex { position: [-1.0, -1.0, -1.0] },
  Vertex { position: [ 1.0, -1.0, -1.0] },
  Vertex { position: [ 1.0, -1.0,  1.0] },
  Vertex { position: [-1.0, -1.0,  1.0] },
  Vertex { position: [-1.0,  1.0, -1.0] },
  Vertex { position: [ 1.0,  1.0, -1.0] },
  Vertex { position: [ 1.0,  1.0,  1.0] },
  Vertex { position: [-1.0,  1.0,  1.0] },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
  0, 2, 1,  0, 3, 2,
  4, 5, 6,  4, 6, 7,
  3, 6, 2,  3, 7, 6,
  0, 1, 5,  0, 5, 4,
  0, 7, 3,  0, 4, 7,
  1, 2, 6,  1, 6, 5,
];
