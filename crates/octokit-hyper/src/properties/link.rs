use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
  pub href: String,
}
