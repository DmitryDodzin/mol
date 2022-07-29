use async_trait::async_trait;

use octokit_hyper::{
  api::compare::client::compare_request,
  api::issue::comments::{client::list_issue_comment, IssueListCommentQuery},
  prelude::*,
};
use octokit_webhooks::PullRequestEvent;

use crate::actions::{Action, UnwrapActions, MESSAGE_PREFIX};

#[async_trait]
#[allow(clippy::single_match)]
impl UnwrapActions for PullRequestEvent {
  async fn unwrap_actions(&self, client: &Client) -> anyhow::Result<Vec<Action>> {
    let auth = OAuth::from_env().unwrap_or_default();
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
      | PullRequestEvent::Synchronize {
        number: _,
        pull_request,
        before: _,
        after: _,
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
            &repository.owner.login,
            &repository.name,
            &pull_request.base.sha,
            &pull_request.head.sha,
          )
          .map(|req| req.with_auth(&auth))
          .send(client)
          .await?;

          let comments = list_issue_comment(
            &repository.owner.login,
            &repository.name,
            pull_request.number,
            IssueListCommentQuery {
              ..Default::default()
            },
          )
          .map(|req| req.with_auth(&auth))
          .send(client)
          .await?
          .iter()
          .filter(|comment| comment.body.starts_with(MESSAGE_PREFIX))
          .map(|comment| comment.id)
          .collect::<Vec<u64>>();

          if comparison.files.is_empty()
            || !comparison.files.iter().any(|file| {
              file.filename.starts_with(".changeset/")
                && (file.status == "added" || file.status == "modified")
                && !file.filename.ends_with("README.md")
            })
          {
            if !comments.is_empty() {
              actions.push(Action::RemoveCommentNoChangesets {
                repository: repository.clone(),
                comment_ids: comments,
              });
            }

            actions.push(Action::CommentNoChangesets {
              repository: repository.clone(),
              pull_request: pull_request.clone(),
            });
          } else {
            actions.push(Action::RemoveCommentNoChangesets {
              repository: repository.clone(),
              comment_ids: comments,
            });
          }
        }
      }
      _ => {}
    }

    Ok(actions)
  }
}
