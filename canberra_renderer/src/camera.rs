use glam::{Mat4, Vec3};

#[derive(Debug)]
pub struct Camera {
  pub view_proj: Mat4,
}

impl Camera {
  pub fn new(width: u32, height: u32) -> Self {
    let aspect = width as f32 / height.max(1) as f32;
    let proj = Mat4::perspective_rh(45.0_f32.to_radians(), aspect, 0.1, 100.0); // todo: near/far controls
    let view = Mat4::look_at_rh(Vec3::new(3.0, 2.5, 4.0), Vec3::ZERO, Vec3::Y); // todo: camera controls

    Self {
      view_proj: proj * view,
    }
  }
}
