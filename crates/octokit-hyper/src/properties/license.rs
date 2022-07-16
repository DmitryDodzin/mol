use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct License {
  pub key: String,
  pub name: String,
  pub spdx_id: String,
  pub url: Option<String>,
  pub node_id: String,
}
