use std::collections::HashMap;

use glam::Mat4;

use crate::{
  Entity, Scene, Vertex,
  components::{Material, Mesh, Transform},
};

mod asset_manager;
mod camera_uniform;
mod gpu_mesh;
mod object_uniform_data;
mod shader_registry;

pub use self::{
  asset_manager::{AssetManager, MeshHandle},
  shader_registry::{GLOBAL_SHADER_REGISTRY, ShaderHandle, ShaderRegistry},
};
pub(crate) use self::{
  camera_uniform::CameraUniform, gpu_mesh::GpuMesh, object_uniform_data::ObjectUniformData,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const OBJECT_STRIDE: u64 = 256;
const MAX_OBJECTS: u64 = 256;

pub struct Renderer {
  pipelines: HashMap<ShaderHandle, wgpu::RenderPipeline>,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  object_buffer: wgpu::Buffer,
  object_bind_group: wgpu::BindGroup,
  depth_texture: wgpu::Texture,
  depth_view: wgpu::TextureView,
  asset_manager: AssetManager,
}

impl Renderer {
  pub fn new(
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
    width: u32,
    height: u32,
  ) -> Self {
    let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("camera_bgl"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: wgpu::BufferSize::new(CameraUniform::size()),
        },
        count: None,
      }],
    });

    let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("camera_buffer"),
      size: CameraUniform::size(),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("camera_bg"),
      layout: &camera_bgl,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding(),
      }],
    });

    let object_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("object_bgl"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: true,
          min_binding_size: wgpu::BufferSize::new(ObjectUniformData::size()),
        },
        count: None,
      }],
    });

    let object_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("object_buffer"),
      size: MAX_OBJECTS * OBJECT_STRIDE,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("object_bg"),
      layout: &object_bgl,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
          buffer: &object_buffer,
          offset: 0,
          size: wgpu::BufferSize::new(ObjectUniformData::size()),
        }),
      }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("pipeline_layout"),
      bind_group_layouts: &[Some(&camera_bgl), Some(&object_bgl)],
      immediate_size: 0,
    });

    let registry = GLOBAL_SHADER_REGISTRY.read().expect("lock poisoned");
    let pipelines = registry
      .shaders
      .iter()
      .map(|(handle, shader)| {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
          label: Some(&shader.name),
          source: wgpu::ShaderSource::Wgsl(shader.wgsl.clone().into()),
        });
        (
          handle.clone(),
          make_pipeline(device, &module, &pipeline_layout, surface_format),
        )
      })
      .collect();

    let (depth_texture, depth_view) = Self::make_depth_texture(device, width.max(1), height.max(1));

    Self {
      pipelines,
      camera_buffer,
      camera_bind_group,
      object_buffer,
      object_bind_group,
      depth_texture,
      depth_view,
      asset_manager: AssetManager::new(),
    }
  }

  pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
    let (t, v) = Self::make_depth_texture(device, width.max(1), height.max(1));
    self.depth_texture = t;
    self.depth_view = v;
  }

  pub fn render(
    &mut self,
    device: &wgpu::Device,
    scene: &Scene,
    queue: &wgpu::Queue,
    view: &wgpu::TextureView,
    encoder: &mut wgpu::CommandEncoder,
    aspect: f32,
    time: f32,
  ) {
    let view_proj = scene.camera_view_proj(aspect);
    queue.write_buffer(
      &self.camera_buffer,
      0,
      bytemuck::cast_slice(&[CameraUniform::new(view_proj.to_cols_array_2d(), time)]),
    );

    let mut renderables: Vec<(Mat4, &Entity)> = Vec::new();
    for root in &scene.entities {
      collect_renderables(root, Mat4::IDENTITY, &mut renderables);
    }

    let mut object_data = vec![0u8; renderables.len() * OBJECT_STRIDE as usize];
    for (i, (world_mat, entity)) in renderables.iter().enumerate() {
      let color = entity
        .get_component::<Material>()
        .map(|m| m.color)
        .unwrap_or([1.0, 1.0, 1.0, 1.0]);
      let uniform = ObjectUniformData {
        model: world_mat.to_cols_array_2d(),
        color,
      };
      let offset = i * OBJECT_STRIDE as usize;
      let bytes = bytemuck::bytes_of(&uniform);
      object_data[offset..offset + bytes.len()].copy_from_slice(bytes);
    }
    if !object_data.is_empty() {
      queue.write_buffer(&self.object_buffer, 0, &object_data);
    }

    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Render Pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        depth_slice: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
          }),
          store: wgpu::StoreOp::Store,
        },
      })],
      depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: &self.depth_view,
        depth_ops: Some(wgpu::Operations {
          load: wgpu::LoadOp::Clear(1.0),
          store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
      }),
      occlusion_query_set: None,
      timestamp_writes: None,
      multiview_mask: None,
    });

    for (i, (_, entity)) in renderables.iter().enumerate() {
      let shader = entity
        .get_component::<Material>()
        .map(|m| m.shader)
        .unwrap_or_default();
      let pipeline = self.pipelines.get(&shader).unwrap_or_else(|| {
        self
          .pipelines
          .get(&ShaderHandle::default())
          .expect("No default pipeline")
      });
      pass.set_pipeline(pipeline);
      pass.set_bind_group(0, &self.camera_bind_group, &[]);

      let mesh = entity.get_component::<Mesh>().unwrap();
      let (_, gpu_mesh) = self.asset_manager.get_or_upload(device, mesh);

      let dynamic_offset = (i as u64 * OBJECT_STRIDE) as u32;
      pass.set_bind_group(1, &self.object_bind_group, &[dynamic_offset]);
      pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
      pass.set_index_buffer(gpu_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
      pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
    }
  }

  fn make_depth_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
  ) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some("depth_texture"),
      size: wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: DEPTH_FORMAT,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
  }
}

fn make_pipeline(
  device: &wgpu::Device,
  shader: &wgpu::ShaderModule,
  pipeline_layout: &wgpu::PipelineLayout,
  surface_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("render_pipeline"),
    layout: Some(pipeline_layout),
    vertex: wgpu::VertexState {
      module: shader,
      entry_point: Some("vs_main"),
      buffers: &[wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
          wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
          },
          wgpu::VertexAttribute {
            offset: 12,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x3,
          },
        ],
      }],
      compilation_options: Default::default(),
    },
    fragment: Some(wgpu::FragmentState {
      module: shader,
      entry_point: Some("fs_main"),
      targets: &[Some(wgpu::ColorTargetState {
        format: surface_format,
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
  })
}

fn collect_renderables<'a>(
  entity: &'a Entity,
  parent_world: Mat4,
  out: &mut Vec<(Mat4, &'a Entity)>,
) {
  let local = entity
    .get_component::<Transform>()
    .map(|t| t.matrix())
    .unwrap_or(Mat4::IDENTITY);
  let world = parent_world * local;
  if entity.get_component::<Mesh>().is_some() {
    out.push((world, entity));
  }
  for child in entity.children() {
    collect_renderables(child, world, out);
  }
}
