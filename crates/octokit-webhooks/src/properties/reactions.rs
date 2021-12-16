use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reactions {
  pub url: String,
  pub total_count: u32,
  #[serde(rename = "+1")]
  pub positive: u32,
  #[serde(rename = "-1")]
  pub negative: u32,
  pub laugh: u32,
  pub hooray: u32,
  pub confused: u32,
  pub heart: u32,
  pub rocket: u32,
  pub eyes: u32,
}
