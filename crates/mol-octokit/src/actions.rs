use async_trait::async_trait;
use serde::Serialize;

use mol_core::prelude::*;
use octokit_hyper::api::issue::comments::{
  client::{create_issue_comment, list_issue_comment},
  IssueCreateCommentBody, IssueListCommentQuery,
};
use octokit_hyper::prelude::*;
use octokit_hyper::properties::{PullRequest, Repository};

#[derive(Debug)]
pub enum Action {
  CommentNoChangesets {
    pull_request: PullRequest,
    repository: Repository,
  },
  RemoveCommentNoChangesets {
    pull_request: PullRequest,
    repository: Repository,
  },
}

#[derive(Debug, Serialize)]
struct FileCreate {
  filename: String,
  value: String,
}

impl Action {
  pub async fn execute(&self, client: &Client) -> anyhow::Result<()> {
    let auth = OAuth::from_env().unwrap_or_default();

    match self {
      Action::CommentNoChangesets {
        pull_request,
        repository,
      } => {
        let changeset = Changeset {
          message: pull_request.title.clone(),
          packages: vec![(repository.name.clone(), VersionMod::new(Semantic::patch()))]
            .into_iter()
            .collect(),
        };

        let create_changeset_url = serde_urlencoded::to_string(FileCreate {
          filename: format!(".changeset/{}", Changeset::<Semantic>::random_file_name()),
          value: changeset.to_string(),
        })
        .map(|params| {
          format!(
            "https://github.com/{}/new/{}?{}",
            repository.full_name, pull_request.head.r#ref, params
          )
        })
        .expect("No Url");

        let body = format!("###  ⚠️  No Changeset found\n\nLatest commit: {}\n\n[Click here if you're a maintainer who wants to add a changeset to this PR]({})\n", pull_request.head.sha, create_changeset_url);

        let update = create_issue_comment(
          &repository.owner.login,
          &repository.name,
          pull_request.number,
          IssueCreateCommentBody { body },
        )
        .map(|req| req.with_auth(&auth))
        .send(client)
        .await?;

        println!("{:?}", update);
      }
      Action::RemoveCommentNoChangesets {
        pull_request,
        repository,
      } => {
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
        .await?;

        println!("{:?}", comments);
      }
    }

    Ok(())
  }
}

#[async_trait]
pub trait UnwrapActions {
  async fn unwrap_actions(&self, client: &Client) -> anyhow::Result<Vec<Action>>;
}
