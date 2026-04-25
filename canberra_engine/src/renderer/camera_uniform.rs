#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
  pub(crate) view_proj: [[f32; 4]; 4],
  pub(crate) time: f32,
  _pad: [f32; 3],
}

impl CameraUniform {
  pub(crate) fn new(view_proj: [[f32; 4]; 4], time: f32) -> Self {
    Self { view_proj, time, _pad: [0.0; 3] }
  }

  pub(crate) const fn size() -> u64 {
    size_of::<Self>() as u64
  }
}
