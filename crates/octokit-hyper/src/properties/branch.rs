use serde::Deserialize;

use super::CommitUrl;

#[derive(Debug, Deserialize)]
pub struct BranchRef {
  pub name: String,
  pub commit: CommitUrl,
  pub protected: bool,
}
