use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventsUnwrapError {
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
  #[error("Not yet implemented")]
  NotImplemented,
}
