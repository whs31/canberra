mod error;
mod renderer;
mod ui_renderer;

pub use self::{
  error::{Error, Result},
  renderer::{Frame, Renderer},
  ui_renderer::UiRenderer,
};

/// Depth buffer format used by the renderer for the main 3D pass.
///
/// Scenes constructing pipelines with depth testing must use this format.
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
