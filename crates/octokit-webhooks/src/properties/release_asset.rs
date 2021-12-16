use serde::Deserialize;

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
  pub id: u32,
  pub node_id: String,
  /// The file name of the asset.
  pub name: String,
  pub label: Option<String>,
  /// State of the release asset.
  pub state: RealeaseAssetState,
  pub content_type: String,
  pub size: u32,
  pub download_count: u32,
  pub created_at: String,
  pub updated_at: String,
  pub uploader: Option<User>,
}
