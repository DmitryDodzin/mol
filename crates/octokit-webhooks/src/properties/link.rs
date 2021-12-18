use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
  pub href: String,
}
