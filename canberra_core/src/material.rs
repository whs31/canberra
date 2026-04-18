#[derive(Debug, Clone)]
pub struct Material {
  pub pipeline: wgpu::RenderPipeline,
  pub bind_group_layout: wgpu::BindGroupLayout,
}
