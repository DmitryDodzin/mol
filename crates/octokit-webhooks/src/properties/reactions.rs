use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reactions {
  pub url: String,
  pub total_count: u64,
  #[serde(rename = "+1")]
  pub positive: u64,
  #[serde(rename = "-1")]
  pub negative: u64,
  pub laugh: u64,
  pub hooray: u64,
  pub confused: u64,
  pub heart: u64,
  pub rocket: u64,
  pub eyes: u64,
}
