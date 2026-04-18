use std::fmt::Debug;

use egui_wgpu::ScreenDescriptor;
use winit::window::Window;

pub struct UiRenderer {
  pub ctx: egui::Context,
  pub renderer: egui_wgpu::Renderer,
  pub state: egui_winit::State,
}

impl Debug for UiRenderer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UiRenderer").field("ctx", &self.ctx).finish()
  }
}

impl UiRenderer {
  pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, window: &Window) -> Self {
    let ctx = egui::Context::default();
    let renderer = egui_wgpu::Renderer::new(
      device,
      surface_format,
      egui_wgpu::RendererOptions {
        msaa_samples: 1,
        depth_stencil_format: None,
        dithering: true,
        predictable_texture_filtering: false,
      },
    );
    let viewport_id = ctx.viewport_id();
    let state = egui_winit::State::new(ctx.clone(), viewport_id, window, None, None, None);

    Self { ctx, renderer, state }
  }

  /// Feed a winit event into egui. Returns true if egui consumed it.
  pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
    self.state.on_window_event(window, event).consumed
  }

  pub fn render(
    &mut self,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    window: &Window,
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    config: &wgpu::SurfaceConfiguration,
  ) {
    let raw_input = self.state.take_egui_input(window);
    let full_output = self.ctx.run_ui(raw_input, |ui| {
      ui.label("map engine v0.1");
      egui::Window::new("hello").show(ui.ctx(), |ui| {
        ui.label("hello world");
      });
    });

    self.state.handle_platform_output(window, full_output.platform_output);

    let tris = self.ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
    for (id, delta) in &full_output.textures_delta.set {
      self.renderer.update_texture(device, queue, *id, delta);
    }

    let screen = ScreenDescriptor {
      size_in_pixels: [config.width, config.height],
      pixels_per_point: full_output.pixels_per_point,
    };
    self.renderer.update_buffers(device, queue, encoder, &tris, &screen);

    {
      let mut pass = encoder
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("egui pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Load,
              store: wgpu::StoreOp::Store,
            },
          })],
          depth_stencil_attachment: None,
          timestamp_writes: None,
          occlusion_query_set: None,
          multiview_mask: None,
        })
        .forget_lifetime();
      self.renderer.render(&mut pass, &tris, &screen);
    }

    for id in &full_output.textures_delta.free {
      self.renderer.free_texture(id);
    }
  }
}
