mod error;

use canberra_engine::{
  Application, Entity, Scene,
  components::{Camera, Material, Mesh, Transform},
};
use glam::Vec3;

pub use self::error::{Error, Result};

fn try_main() -> Result<()> {
  Application::run(|| {
    let mut scene = Scene::new();

    // Camera
    let mut cam = Entity::new("Camera");
    cam.add_component(Transform::from_translation(Vec3::new(0.0, 2.0, 10.0)));
    cam.add_component(Camera::new(60_f32.to_radians(), 1.0, 0.1, 100.0));
    scene.add(cam);

    // 3 cubes of different colors
    let unique: &[([f32; 4], &str)] = &[
      ([0.9, 0.2, 0.2, 1.0], "Cube_Red"),
      ([0.2, 0.9, 0.2, 1.0], "Cube_Green"),
      ([0.2, 0.4, 0.9, 1.0], "Cube_Blue"),
    ];
    for (i, &(color, name)) in unique.iter().enumerate() {
      let mut e = Entity::new(name);
      e.add_component(Transform::from_translation(Vec3::new(
        (i as f32 - 1.0) * 3.0,
        -1.5,
        0.0,
      )));
      e.add_component(Mesh::cube());
      e.add_component(Material::with_color(color));
      scene.add(e);
    }

    // 3 cubes of the same color (gold)
    let gold = [1.0f32, 0.75, 0.0, 1.0];
    for i in 0..3usize {
      let name = format!("CubeSame_{i}");
      let mut e = Entity::new(&name);
      e.add_component(Transform::from_translation(Vec3::new(
        (i as f32 - 1.0) * 3.0,
        1.5,
        0.0,
      )));
      e.add_component(Mesh::cube());
      e.add_component(Material::with_color(gold));
      scene.add(e);
    }

    scene
  })?;
  Ok(())
}

fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

  if let Err(err) = try_main() {
    tracing::error!("Fatal: {}", err);
    std::process::exit(1);
  }
}
