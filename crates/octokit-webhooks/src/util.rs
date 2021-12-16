use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WrappedSource<T> {
  from: T,
}
