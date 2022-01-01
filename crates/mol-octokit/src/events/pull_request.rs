use async_trait::async_trait;

use octokit_hyper::{api::compare::client::compare_request, prelude::Client};
use octokit_webhooks::PullRequestEvent;

use crate::actions::{Action, UnwrapActions};

#[async_trait]
#[allow(clippy::single_match)]
impl UnwrapActions for PullRequestEvent {
  async fn unwrap_actions(&self, client: &Client) -> anyhow::Result<Vec<Action>> {
    let mut actions = Vec::new();

    match self {
      PullRequestEvent::Opened {
        number: _,
        pull_request,
        repository,
        installation: _,
        organization: _,
        sender: _,
      }
      | PullRequestEvent::Reopened {
        number: _,
        pull_request,
        repository,
        installation: _,
        organization: _,
        sender: _,
      }
      | PullRequestEvent::Edited {
        number: _,
        pull_request,
        changes: _,
        repository,
        installation: _,
        organization: _,
        sender: _,
      } => {
        if !pull_request.head.r#ref.starts_with("mol/") {
          let comparison = compare_request(
            &repository.full_name,
            &pull_request.base.r#ref,
            &pull_request.head.r#ref,
          )
          .send(client)
          .await?;

          if !comparison.files.iter().any(|file| {
            file.filename.starts_with(".changesets")
              && (file.status == "added" || file.status == "modified")
              && !file.filename.ends_with("README.md")
          }) {
            actions.push(Action::CommentNoChangesets {
              repository: repository.clone(),
              pull_request: pull_request.clone(),
            });
          }
        }
      }
      _ => {}
    }

    Ok(actions)
  }
}
