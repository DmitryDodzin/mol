use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::properties::*;
use crate::util::parse_flexible_timestamp;

#[derive(Debug, Deserialize)]
pub struct StatusEvent {
  /// The unique identifier of the status.
  pub id: u64,
  /// The Commit SHA.
  pub sha: String,
  pub name: String,
  pub avatar_url: Option<String>,
  /// The optional link added to the status.
  pub target_url: Option<String>,
  pub context: String,
  /// The optional human-readable description added to the status.
  pub description: Option<String>,
  /// The new state. Can be `pending`, `success`, `failure`, or `error`.
  pub state: StatusEventState,
  pub commit: StatusCommit,
  /// An array of branch objects containing the status' SHA. Each branch contains the given SHA, but the SHA may or may not be the head of the branch. The array includes a maximum of 10 branches.
  pub branches: Vec<BranchRef>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  pub repository: Repository,
  pub sender: User,
  pub installation: Option<InstallationLite>,
  pub organization: Option<Organization>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusEventState {
  Pending,
  Success,
  Failure,
  Error,
}

#[derive(Debug, Deserialize)]
pub struct StatusCommit {
  pub sha: String,
  pub node_id: String,
  pub commit: StatusCommitMeta,
  pub url: String,
  pub html_url: String,
  pub comments_url: String,
  pub author: Option<User>,
  pub committer: Option<User>,
  pub parents: Vec<CommitParent>,
}

#[derive(Debug, Deserialize)]
pub struct StatusCommitMeta {
  pub author: Committer,
  pub committer: Committer,
  pub message: String,
  pub tree: CommitUrl,
  pub url: String,
  pub comment_count: u64,
  pub verification: Verification,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(payload, StatusEvent, "./sample/status/payload.json");
  test_from_sample!(
    with_author_committer_null,
    StatusEvent,
    "./sample/status/with-author-committer-null.payload.json"
  );
  test_from_sample!(
    with_installation,
    StatusEvent,
    "./sample/status/with-installation.payload.json"
  );
}
