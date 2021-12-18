pub use serde::Deserialize;

use super::CommitRef;

#[derive(Debug, Deserialize)]
pub struct RunPullRequest {
  pub url: String,
  pub id: u64,
  pub number: u64,
  pub head: CommitRef,
  pub base: CommitRef,
}
