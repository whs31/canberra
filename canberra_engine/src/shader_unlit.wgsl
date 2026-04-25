struct Camera {
  view_proj: mat4x4<f32>,
}

struct Object {
  model: mat4x4<f32>,
  color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var<uniform> object: Object;

struct VertIn {
  @location(0) position: vec3<f32>,
  @location(1) normal:   vec3<f32>,
}

struct VertOut {
  @builtin(position) clip_pos: vec4<f32>,
  @location(0)       color:    vec4<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
  var out: VertOut;
  out.clip_pos = camera.view_proj * object.model * vec4<f32>(in.position, 1.0);
  out.color    = object.color;
  return out;
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
  return in.color;
}
