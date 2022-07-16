use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::{Committer, RepoRef};

#[derive(Debug, Deserialize)]
pub struct Commit {
  pub id: String,
  pub tree_id: String,
  /// Whether this commit is distinct from any that have been pushed before.
  pub distinct: bool,
  /// The commit message.
  pub message: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
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

#[derive(Debug, Deserialize)]
pub struct SimpleCommit {
  pub id: String,
  pub tree_id: String,
  pub message: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub timestamp: DateTime<Utc>,
  pub author: Committer,
  pub committer: Committer,
}

#[derive(Debug, Deserialize)]
pub struct CommitParent {
  pub sha: String,
  pub url: String,
  pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitRef {
  pub r#ref: String,
  pub sha: String,
  pub repo: RepoRef,
}

#[derive(Debug, Deserialize)]
pub struct CommitUrl {
  pub sha: String,
  pub url: String,
}
