use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

#[derive(Debug, Deserialize)]
pub struct CheckRunDeployment {
  pub url: String,
  pub id: u64,
  pub node_id: String,
  pub task: String,
  pub original_environment: String,
  pub environment: String,
  pub description: Option<String>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  pub statuses_url: String,
  pub repository_url: String,
}
