use async_trait::async_trait;
use serde::Serialize;

use mol_core::prelude::*;
use octokit_hyper::api::issue::comments::{client::create_issue_comment, IssueCreateCommentBody};
use octokit_hyper::prelude::*;
use octokit_hyper::properties::{PullRequest, Repository};

#[derive(Debug)]
pub enum Action {
  CommentNoChangesets {
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
    let auth = OAuth {
      access_token: std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_owned()),
      token_type: "token".to_owned(),
      scope: "".to_owned(),
    };

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
    }

    Ok(())
  }
}

#[async_trait]
pub trait UnwrapActions {
  async fn unwrap_actions(&self, client: &Client) -> anyhow::Result<Vec<Action>>;
}
