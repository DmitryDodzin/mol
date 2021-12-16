use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::Committer;

#[derive(Debug, Deserialize)]
pub struct Commit {
  pub id: String,
  pub tree_id: String,
  /// Whether this commit is distinct from any that have been pushed before.
  pub distinct: bool,
  /// The commit message.
  pub message: String,
  pub timestamp: DateTime<Utc>,
  /// URL that points to the commit API resource.
  pub url: String,
  pub author: Committer,
  pub committer: Committer,
  /// An array of files added in the commit.
  pub added: Vec<String>,
  /// An array of files modified by the commit.
  pub modified: Vec<String>,
  /// An array of files removed in the commit.
  pub removed: Vec<String>,
}
