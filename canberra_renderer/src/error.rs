#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  SurfaceCreation(#[from] wgpu::CreateSurfaceError),

  #[error(transparent)]
  AdapterRequest(#[from] wgpu::RequestAdapterError),

  #[error(transparent)]
  DeviceRequest(#[from] wgpu::RequestDeviceError),
}

pub type Result<T> = std::result::Result<T, Error>;
