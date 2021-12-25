use async_trait::async_trait;
use serde::Serialize;

use mol_core::prelude::*;
use octokit_webhooks::properties::{PullRequest, Repository};

#[derive(Debug)]
pub enum Action {
  CommentNoChangesets {
    branch: String,
    latest_commit: String,
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
  pub async fn execute(&self) -> anyhow::Result<()> {
    match self {
      Action::CommentNoChangesets {
        branch,
        repository,
        latest_commit,
        pull_request,
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
            repository.full_name, branch, params
          )
        })
        .expect("No Url");

        println!("###  ⚠️  No Changeset found\n\nLatest commit: {}\n\n[Click here if you're a maintainer who wants to add a changeset to this PR]({})\n", latest_commit, create_changeset_url);
      }
    }

    Ok(())
  }
}

#[async_trait]
pub trait UnwrapActions {
  async fn unwrap_actions(&self) -> Vec<Action>;
}
