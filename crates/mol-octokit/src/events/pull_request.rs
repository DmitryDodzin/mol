use async_trait::async_trait;
use serde::Deserialize;

use octokit_webhooks::*;

use crate::actions::{Action, UnwrapActions};
use crate::octokit::Client;

#[derive(Debug, Deserialize)]
struct File {
  sha: String,
  filename: String,
  status: String,
  additions: u64,
  deletions: u64,
  changes: u64,
  blob_url: String,
  raw_url: String,
  contents_url: String,
  patch: String,
}

#[derive(Debug, Deserialize)]
struct CompareResult {
  files: Vec<File>,
}

async fn fetch_compare(pull_request: &properties::PullRequest) -> anyhow::Result<CompareResult> {
  let compare_url = pull_request
    .base
    .repo
    .compare_url
    .replace("{base}", &pull_request.base.sha)
    .replace("{head}", &pull_request.head.sha);

  Client::new().get(&compare_url).await
}

#[async_trait]
impl UnwrapActions for PullRequestEvent {
  async fn unwrap_actions(&self) -> Vec<Action> {
    let mut actions = Vec::new();

    match self {
      PullRequestEvent::Opened {
        number: _,
        pull_request,
        repository,
        installation: _,
        organization: _,
        sender: _,
      } => {
        if !pull_request.head.r#ref.starts_with("mol/") {
          match fetch_compare(pull_request).await {
            Ok(comparison) => {
              let changesets: Vec<&File> = comparison
                .files
                .iter()
                .filter(|file| {
                  file.filename.starts_with(".changesets")
                    && (file.status == "added" || file.status == "modified")
                    && !file.filename.ends_with("README.md")
                })
                .collect();

              if changesets.is_empty() {
                actions.push(Action::CommentNoChangesets {
                  branch: pull_request.head.r#ref.clone(),
                  latest_commit: pull_request.head.sha.clone(),
                  repository: repository.clone(),
                  pull_request: pull_request.clone(),
                });
              }
            }
            Err(err) => {
              println!("{:?}", err);
            }
          }
        }
      }
      _ => {}
    }

    actions
  }
}
