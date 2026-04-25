#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ObjectUniformData {
  pub(crate) model: [[f32; 4]; 4],
  pub(crate) color: [f32; 4],
}

impl ObjectUniformData {
  pub(crate) const fn size() -> u64 {
    size_of::<Self>() as u64
  }
}
