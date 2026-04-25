#[derive(Debug, Clone)]
pub struct Shader {
  pub name: String,
  pub wgsl: String,
}

impl std::fmt::Display for Shader {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Shader {
  pub fn new(name: &str, wgsl: impl Into<String>) -> Self {
    Self {
      name: name.to_string(),
      wgsl: wgsl.into(),
    }
  }
}
