use std::fmt::Debug;

use egui_wgpu::ScreenDescriptor;
use winit::window::Window;

use crate::Frame;

pub struct UiRenderer {
  pub ctx: egui::Context,
  pub renderer: egui_wgpu::Renderer,
  pub state: egui_winit::State,
}

impl Debug for UiRenderer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UiRenderer")
      .field("ctx", &self.ctx)
      .finish()
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

    Self {
      ctx,
      renderer,
      state,
    }
  }

  /// Feed a winit event into egui. The returned [`egui_winit::EventResponse`]
  /// tells the caller whether egui consumed the event and whether a redraw is
  /// needed (e.g. to update hover state, animate, or finish a drag).
  pub fn handle_event(
    &mut self,
    window: &Window,
    event: &winit::event::WindowEvent,
  ) -> egui_winit::EventResponse {
    self.state.on_window_event(window, event)
  }

  /// Tessellate and draw the UI for this frame on top of the current color target.
  ///
  /// `build_ui` is called with the active [`egui::Context`] and is the only place
  /// where concrete UI content belongs.
  pub fn render<F: FnMut(&egui::Context)>(&mut self, frame: &mut Frame<'_>, mut build_ui: F) {
    let raw_input = self.state.take_egui_input(frame.window);
    let full_output = self.ctx.run_ui(raw_input, |ui| build_ui(ui.ctx()));

    self
      .state
      .handle_platform_output(frame.window, full_output.platform_output);

    let tris = self
      .ctx
      .tessellate(full_output.shapes, full_output.pixels_per_point);
    for (id, delta) in &full_output.textures_delta.set {
      self
        .renderer
        .update_texture(frame.device, frame.queue, *id, delta);
    }

    let screen = ScreenDescriptor {
      size_in_pixels: [frame.surface_config.width, frame.surface_config.height],
      pixels_per_point: full_output.pixels_per_point,
    };
    self.renderer.update_buffers(
      frame.device,
      frame.queue,
      &mut frame.encoder,
      &tris,
      &screen,
    );

    {
      let mut pass = frame
        .encoder
        .begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("egui pass"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame.color_view,
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
