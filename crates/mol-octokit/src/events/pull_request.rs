use async_trait::async_trait;

use octokit_hyper::{
  api::compare::{client::compare_request, CompareReply},
  prelude::{Client, RequestExt},
  properties::{PullRequest, Repository},
};
use octokit_webhooks::PullRequestEvent;

use crate::actions::{Action, UnwrapActions};

async fn fetch_compare(
  client: &Client,
  repository: &Repository,
  pull_request: &PullRequest,
) -> anyhow::Result<CompareReply> {
  compare_request(
    &repository.full_name,
    &pull_request.base.r#ref,
    &pull_request.head.r#ref,
  )
  .send(client)
  .await
}

#[async_trait]
#[allow(clippy::single_match)]
impl UnwrapActions for PullRequestEvent {
  async fn unwrap_actions(&self) -> Vec<Action> {
    let client = Client::new();
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
          match fetch_compare(&client, repository, pull_request).await {
            Ok(comparison) => {
              if !comparison.files.iter().any(|file| {
                file.filename.starts_with(".changesets")
                  && (file.status == "added" || file.status == "modified")
                  && !file.filename.ends_with("README.md")
              }) {
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
