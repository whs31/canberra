#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  WindowEventLoop(#[from] winit::error::EventLoopError),

  #[error(transparent)]
  OsError(#[from] winit::error::OsError),

  #[error(transparent)]
  Renderer(#[from] canberra_renderer::Error),

  #[error(transparent)]
  Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
