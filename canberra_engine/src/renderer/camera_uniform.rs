#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
  pub(crate) view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  pub(crate) const fn size() -> u64 {
    size_of::<Self>() as u64
  }
}
