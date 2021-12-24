use async_trait::async_trait;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use octokit_webhooks::*;

use crate::actions::{Action, UnwrapActions};

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
  url: String,
  html_url: String,
  permalink_url: String,
  diff_url: String,
  patch_url: String,
  status: String, // TODO: Replace
  ahead_by: u64,
  behind_by: u64,
  total_commits: u64,
  files: Vec<File>,
}

async fn fetch_compare(pull_request: &properties::PullRequest) -> anyhow::Result<CompareResult> {
  let head_sha = &pull_request.head.sha;
  let base_sha = &pull_request.base.sha;

  let compare_url = pull_request
    .base
    .repo
    .compare_url
    .replace("{base}", base_sha)
    .replace("{head}", head_sha);

  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  let req = Request::builder()
    .method(Method::GET)
    .uri(compare_url)
    .header(
      "user-agent",
      format!(
        "{}/{} (https://github.com/DmitryDodzin/mol)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
      ),
    )
    .body(Body::empty())?;

  let res = client.request(req).await?;
  let buf = hyper::body::to_bytes(res).await?;

  serde_json::from_slice(&buf).map_err(|err| err.into())
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
      } => match fetch_compare(pull_request).await {
        Ok(comparison) => {
          let changesets: Vec<&File> = comparison
            .files
            .iter()
            .filter(|file| {
              file.filename.starts_with(".changesets") && !file.filename.ends_with("README.md")
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
      },
      _ => {}
    }

    actions
  }
}
