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
  @builtin(position) clip_pos:     vec4<f32>,
  @location(0)       color:        vec4<f32>,
  @location(1)       world_normal: vec3<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
  var out: VertOut;
  out.clip_pos = camera.view_proj * object.model * vec4<f32>(in.position, 1.0);
  out.color    = object.color;
  // Upper-3x3 of the model matrix. Correct for uniform-scale transforms;
  // non-uniform scale would require the inverse-transpose normal matrix.
  let m = mat3x3<f32>(
    object.model[0].xyz,
    object.model[1].xyz,
    object.model[2].xyz,
  );
  out.world_normal = normalize(m * in.normal);
  return out;
}

const AMBIENT: f32 = 0.15;

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
  let light_dir = normalize(vec3<f32>(1.0, 3.0, 2.0));
  let n         = normalize(in.world_normal);
  let diffuse   = max(dot(n, light_dir), 0.0);
  let intensity = AMBIENT + (1.0 - AMBIENT) * diffuse;
  return vec4<f32>(in.color.rgb * intensity, in.color.a);
}
