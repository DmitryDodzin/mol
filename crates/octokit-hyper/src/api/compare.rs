use serde::Deserialize;

use crate::properties::FilePatch;

#[derive(Debug, Deserialize)]
pub struct CompareReply {
  pub url: String,
  pub html_url: String,
  pub permalink_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub status: String, // TODO: Replace with enum
  pub ahead_by: u64,
  pub behind_by: u64,
  pub total_commits: u64,
  pub files: Vec<FilePatch>,
}

#[cfg(feature = "client")]
pub mod client {
  use crate::api::GITHUB_API;
  use crate::request::builder::RequestBuilder;

  #[cfg(feature = "client")]
  pub fn compare_request(repo: &str, base: &str, head: &str) -> RequestBuilder {
    RequestBuilder::new(format!("{}/repos/{}/{}..{}", *GITHUB_API, repo, base, head))
      .method(hyper::Method::GET)
  }
}
