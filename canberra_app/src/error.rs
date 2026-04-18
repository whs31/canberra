#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Engine error: {0}")]
  Engine(#[from] canberra_engine::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
