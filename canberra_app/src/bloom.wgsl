struct Camera {
  view_proj: mat4x4<f32>,
  time: f32,
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
  @location(2)       view_dir:     vec3<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
  var out: VertOut;

  let world_pos = object.model * vec4<f32>(in.position, 1.0);
  out.clip_pos = camera.view_proj * world_pos;
  out.color    = object.color;

  let m = mat3x3<f32>(
    object.model[0].xyz,
    object.model[1].xyz,
    object.model[2].xyz,
  );

  out.world_normal = normalize(m * in.normal);

  // Basic view direction (assuming camera is at origin or using simplified vector)
  // For a real camera position, you'd need to pass camera_pos in the uniform.
  out.view_dir = normalize(-world_pos.xyz);

  return out;
}

const BLOOM_INTENSITY: f32 = 2.5;
const BLOOM_THRESHOLD: f32 = 0.5;

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
  let n = normalize(in.world_normal);
  let v = normalize(in.view_dir);

  // 1. Standard Diffuse Lighting
  let light_dir = normalize(vec3<f32>(1.0, 3.0, 2.0));
  let diffuse = max(dot(n, light_dir), 0.0);

  // 2. Fresnel Effect (Glow at edges)
  var fresnel = 1.0 - max(dot(n, v), 0.0);
  fresnel = pow(fresnel, 3.0); // Sharpen the glow edge

  // 3. Brightness "Bloom" extraction
  // Boost colors that are already bright
  let luminance = dot(in.color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
  let bloom_mask = smoothstep(BLOOM_THRESHOLD, 1.0, luminance);

  // Combine
  let base_light = in.color.rgb * (diffuse + 0.15);
  let glow = in.color.rgb * fresnel * BLOOM_INTENSITY;
  let bright_pass = in.color.rgb * bloom_mask * BLOOM_INTENSITY;

  let final_color = base_light + glow + bright_pass;

  return vec4<f32>(final_color, in.color.a);
}