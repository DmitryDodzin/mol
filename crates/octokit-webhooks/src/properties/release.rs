use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp_option;

use super::{Reactions, ReleaseAsset, User};

#[derive(Debug, Deserialize)]
pub struct Release {
  pub url: String,
  pub assets_url: String,
  pub upload_url: String,
  pub html_url: String,
  pub id: u32,
  pub node_id: String,
  /// The name of the tag.
  pub tag_name: String,
  /// Specifies the commitish value that determines where the Git tag is created from.
  pub target_commitish: String,
  pub name: String,
  /// Wether the release is a draft or published
  pub draft: bool,
  pub author: User,
  /// Whether the release is identified as a prerelease or a full release.
  pub prerelease: bool,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub created_at: Option<DateTime<Utc>>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub published_at: Option<DateTime<Utc>>,
  pub assets: Vec<ReleaseAsset>,
  pub tarball_url: Option<String>,
  pub zipball_url: Option<String>,
  pub body: String,
  pub reactions: Option<Reactions>,
  pub discussion_url: Option<String>,
}
