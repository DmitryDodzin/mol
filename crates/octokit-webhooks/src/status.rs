use serde::Deserialize;

use crate::properties::*;

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
  pub created_at: String,
  pub updated_at: String,
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
  pub tree: CommitRef,
  pub url: String,
  pub comment_count: u64,
  pub verification: Verification,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn payload() {
    let raw = std::fs::read_to_string("./sample/status/payload.json").expect("test case not found");

    let event = serde_json::from_str::<StatusEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_author_committer_null() {
    let raw = std::fs::read_to_string("./sample/status/with-author-committer-null.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<StatusEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_installation() {
    let raw = std::fs::read_to_string("./sample/status/with-installation.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<StatusEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
