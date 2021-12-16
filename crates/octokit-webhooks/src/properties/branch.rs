use serde::Deserialize;

use super::CommitRef;

#[derive(Debug, Deserialize)]
pub struct BranchRef {
  pub name: String,
  pub commit: CommitRef,
  pub protected: bool,
}
