use crate::Result;

pub fn event_loop() -> Result<winit::event_loop::EventLoop<()>> {
  let event_loop = winit::event_loop::EventLoop::new()?;
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
  Ok(event_loop)
}
