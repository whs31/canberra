#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Device lost")]
  LostDevice,

  #[error(transparent)]
  WindowEventLoop(#[from] winit::error::EventLoopError),

  #[error(transparent)]
  OsError(#[from] winit::error::OsError),

  #[error(transparent)]
  SurfaceCreation(#[from] wgpu::CreateSurfaceError),

  #[error(transparent)]
  AdapterRequest(#[from] wgpu::RequestAdapterError),

  #[error(transparent)]
  DeviceRequest(#[from] wgpu::RequestDeviceError),

  #[error(transparent)]
  Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
