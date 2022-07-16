use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventsUnwrapError {
  #[cfg(feature = "json")]
  #[error(transparent)]
  SerdeJsonError(#[from] serde_json::Error),
  #[error("Not yet implemented")]
  NotImplemented,
}
