use std::{
  collections::HashMap,
  sync::{LazyLock, RwLock},
};

use crate::Shader;

/// Opaque handle to a custom compiled shader pipeline.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ShaderHandle(pub(crate) u32);

pub static GLOBAL_SHADER_REGISTRY: LazyLock<RwLock<ShaderRegistry>> =
  LazyLock::new(|| RwLock::new(ShaderRegistry::new()));

impl std::fmt::Display for ShaderHandle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ShaderHandle({})", self.0)
  }
}

/// Pre-device registry: stores WGSL source strings and assigns handles.
/// Pass `&mut ShaderRegistry` to the scene builder, register your WGSL,
/// then the renderer compiles everything during initialisation.
pub struct ShaderRegistry {
  pub(crate) shaders: HashMap<ShaderHandle, Shader>,
  next_id: u32,
}

impl ShaderRegistry {
  fn new() -> Self {
    Self {
      shaders: HashMap::new(),
      next_id: 0,
    }
    .register_default()
  }

  /// Register a WGSL shader source and return its handle.
  /// The shader must expose `vs_main` and `fs_main` entry points and
  /// declare the same bind groups as the built-in shaders:
  ///   group(0) binding(0) — camera uniform  (view_proj: mat4x4<f32>)
  ///   group(1) binding(0) — object uniform  (model: mat4x4<f32>, color: vec4<f32>)
  /// Vertex input: @location(0) position: vec3<f32>, @location(1) normal: vec3<f32>
  pub fn register(&mut self, shader: Shader) -> ShaderHandle {
    let handle = ShaderHandle(self.next_id);
    self.next_id += 1;
    self.shaders.insert(handle, shader);
    handle
  }

  #[inline]
  pub fn get(&self, handle: ShaderHandle) -> Option<&Shader> {
    self.shaders.get(&handle)
  }

  fn register_default(mut self) -> Self {
    self.register(Shader::new(
      "Default Lit",
      include_str!("../shader_lit.wgsl"),
    ));
    self.register(Shader::new(
      "Default Unlit",
      include_str!("../shader_unlit.wgsl"),
    ));
    self
  }
}

impl Default for ShaderRegistry {
  fn default() -> Self {
    Self::new()
  }
}
