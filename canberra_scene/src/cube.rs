use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use canberra_renderer::{DEPTH_FORMAT, Frame};
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
  position: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Uniforms {
  view_proj: [[f32; 4]; 4],
  model: [[f32; 4]; 4],
}

const CUBE_VERTICES: &[Vertex] = &[
  Vertex {
    position: [-1.0, -1.0, -1.0],
  },
  Vertex {
    position: [1.0, -1.0, -1.0],
  },
  Vertex {
    position: [1.0, -1.0, 1.0],
  },
  Vertex {
    position: [-1.0, -1.0, 1.0],
  },
  Vertex {
    position: [-1.0, 1.0, -1.0],
  },
  Vertex {
    position: [1.0, 1.0, -1.0],
  },
  Vertex {
    position: [1.0, 1.0, 1.0],
  },
  Vertex {
    position: [-1.0, 1.0, 1.0],
  },
];

#[rustfmt::skip]
const CUBE_INDICES: &[u16] = &[
  0, 2, 1,  0, 3, 2,
  4, 5, 6,  4, 6, 7,
  3, 6, 2,  3, 7, 6,
  0, 1, 5,  0, 5, 4,
  0, 7, 3,  0, 4, 7,
  1, 2, 6,  1, 6, 5,
];

const SHADER: &str = r#"
struct Uniforms {
  view_proj: mat4x4<f32>,
  model:     mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VsOut {
  @builtin(position) clip: vec4<f32>,
};

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VsOut {
  var out: VsOut;
  out.clip = u.view_proj * u.model * vec4<f32>(position, 1.0);
  return out;
}

@fragment
fn fs_main(_in: VsOut) -> @location(0) vec4<f32> {
  return vec4<f32>(0.9, 0.15, 0.15, 1.0);
}
"#;

pub struct Cube {
  pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
  index_buffer: wgpu::Buffer,
  index_count: u32,
  uniform_buffer: wgpu::Buffer,
  uniform_bind_group: wgpu::BindGroup,
  start: Instant,
}

impl Cube {
  pub fn new(device: &wgpu::Device, color_format: wgpu::TextureFormat) -> Self {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("cube shader"),
      source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("cube vertex buffer"),
      contents: bytemuck::cast_slice(CUBE_VERTICES),
      usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("cube index buffer"),
      contents: bytemuck::cast_slice(CUBE_INDICES),
      usage: wgpu::BufferUsages::INDEX,
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("cube uniforms"),
      size: std::mem::size_of::<Uniforms>() as u64,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("cube bgl"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("cube bg"),
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding(),
      }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("cube pipeline layout"),
      bind_group_layouts: &[Some(&bind_group_layout)],
      immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("cube pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[wgpu::VertexBufferLayout {
          array_stride: std::mem::size_of::<Vertex>() as u64,
          step_mode: wgpu::VertexStepMode::Vertex,
          attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
          }],
        }],
        compilation_options: Default::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: color_format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: Default::default(),
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: Some(wgpu::DepthStencilState {
        format: DEPTH_FORMAT,
        depth_write_enabled: Some(true),
        depth_compare: Some(wgpu::CompareFunction::Less),
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
      }),
      multisample: wgpu::MultisampleState::default(),
      multiview_mask: None,
      cache: None,
    });

    Self {
      pipeline,
      vertex_buffer,
      index_buffer,
      index_count: CUBE_INDICES.len() as u32,
      uniform_buffer,
      uniform_bind_group,
      start: Instant::now(),
    }
  }

  fn update(&self, queue: &wgpu::Queue, width: u32, height: u32) {
    let aspect = width as f32 / height.max(1) as f32;
    let proj = Mat4::perspective_rh(45f32.to_radians(), aspect, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(3.0, 2.5, 4.0), Vec3::ZERO, Vec3::Y);
    let t = self.start.elapsed().as_secs_f32();
    let model = Mat4::from_rotation_y(t * 0.6) * Mat4::from_rotation_x(t * 0.3);
    let uniforms = Uniforms {
      view_proj: (proj * view).to_cols_array_2d(),
      model: model.to_cols_array_2d(),
    };
    queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
  }

  /// Clear the color & depth targets and draw the cube into the current frame.
  pub fn render(&self, frame: &mut Frame<'_>) {
    self.update(
      frame.queue,
      frame.surface_config.width,
      frame.surface_config.height,
    );

    let mut pass = frame
      .encoder
      .begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("cube pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &frame.color_view,
          resolve_target: None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.05,
              g: 0.05,
              b: 0.08,
              a: 1.0,
            }),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: frame.depth_view,
          depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),
            store: wgpu::StoreOp::Store,
          }),
          stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
      });

    pass.set_pipeline(&self.pipeline);
    pass.set_bind_group(0, &self.uniform_bind_group, &[]);
    pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    pass.draw_indexed(0..self.index_count, 0, 0..1);
  }
}
