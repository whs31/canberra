#[derive(Debug, Clone)]
pub struct ObjectUniform {
  pub buffer: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
}
