use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::User;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RealeaseAssetState {
  Uploaded,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseAsset {
  pub url: String,
  pub browser_download_url: String,
  pub id: u64,
  pub node_id: String,
  /// The file name of the asset.
  pub name: String,
  pub label: Option<String>,
  /// State of the release asset.
  pub state: RealeaseAssetState,
  pub content_type: String,
  pub size: u64,
  pub download_count: u64,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  pub uploader: Option<User>,
}
