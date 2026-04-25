#[repr(C)]
#[derive(
  Debug,
  Default,
  Copy,
  Clone,
  bytemuck::Pod,
  bytemuck::Zeroable,
  serde::Serialize,
  serde::Deserialize,
)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
}
