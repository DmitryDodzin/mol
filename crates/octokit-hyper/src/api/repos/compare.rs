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
  use crate::octokit_request;
  use crate::request::{middleware::Unauthorized, proxy::RequestProxy};

  use super::CompareReply;

  pub fn compare_request(
    owner: &str,
    repo: &str,
    base: &str,
    head: &str,
  ) -> RequestProxy<CompareReply, Unauthorized> {
    RequestProxy::new(octokit_request!(
      GET,
      "/repos/{}/{}/compare/{}...{}",
      owner,
      repo,
      base,
      head
    ))
  }
}
