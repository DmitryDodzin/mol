pub use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CheckRunPullRequest {
  pub url: String,
  pub id: u64,
  pub number: u64,
  pub head: CheckRunCommitRef,
  pub base: CheckRunCommitRef,
}

#[derive(Debug, Deserialize)]
pub struct CheckRunCommitRef {
  pub r#ref: String,
  pub sha: String,
  pub repo: RepoRef,
}

#[derive(Debug, Deserialize)]
pub struct RepoRef {
  pub id: u64,
  pub url: String,
  pub name: String,
}
