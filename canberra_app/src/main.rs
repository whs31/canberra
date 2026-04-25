mod error;

use canberra_engine::{
  Application, Entity, Scene, Shader, ShaderHandle,
  components::{Camera, Material, Mesh, Transform},
  register_shaders,
};
use glam::Vec3;

pub use self::error::{Error, Result};

fn try_main() -> Result<()> {
  Application::run(|| {
    let mut wobble_shader = ShaderHandle::default();
    let mut bloom_shader = ShaderHandle::default();
    register_shaders(|registry| {
      wobble_shader = registry.register(Shader::new("Wobble", include_str!("wobble.wgsl")));
      bloom_shader = registry.register(Shader::new("Bloom", include_str!("bloom.wgsl")));
    });
    let mut scene = Scene::new();

    // Camera
    let mut cam = Entity::new("Camera");
    cam.add_component(Transform::from_translation(Vec3::new(0.0, 2.0, 10.0)));
    cam.add_component(Camera::new(60_f32.to_radians(), 1.0, 0.1, 100.0));
    scene.add(cam);

    // Group: 3 cubes of different colors
    let mut colored_group = Entity::new("ColoredCubes");
    colored_group.add_component(Transform::default());
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
      colored_group.add_child(e);
    }
    scene.add(colored_group);

    // Group: 3 cubes of the same color (gold)
    let mut same_group = Entity::new("SameCubes");
    same_group.add_component(Transform::default());
    let gold = [1.0f32, 0.75, 0.0, 1.0];
    for i in 0..3usize {
      let mut e = Entity::new(&format!("CubeSame_{i}"));
      e.add_component(Transform::from_translation(Vec3::new(
        (i as f32 - 1.0) * 3.0,
        1.5,
        0.0,
      )));
      e.add_component(Mesh::cube());
      e.add_component(Material::with_color(gold));
      same_group.add_child(e);
    }
    scene.add(same_group);

    // Wobbly cube (center, front)
    let mut wobbly = Entity::new("WobblyCube");
    wobbly.add_component(Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)));
    wobbly.add_component(Mesh::cube());
    wobbly.add_component(Material {
      color: [0.9, 0.5, 0.1, 1.0],
      shader: wobble_shader,
    });
    scene.add(wobbly);

    // Bloom cube
    let mut bloomy = Entity::new("Bloom Cube");
    bloomy.add_component(Transform::from_translation(Vec3::new(3.0, 4.0, 0.0)));
    bloomy.add_component(Mesh::cube());
    bloomy.add_component(Material {
      color: [1.0, 1.0, 0.3, 1.0],
      shader: bloom_shader,
    });
    scene.add(bloomy);

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
