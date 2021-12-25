use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UrlencodedWrapper {
  pub payload: String,
}

#[derive(Debug, Deserialize)]
pub struct WrappedSource<T> {
  pub from: T,
}
