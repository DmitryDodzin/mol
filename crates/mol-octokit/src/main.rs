use async_trait::async_trait;

use octokit_hyper::prelude::*;
use octokit_ntex::{Octokit, OctokitConfig};
use octokit_webhooks::Events;

mod actions;
mod events;

use actions::UnwrapActions;

struct MolOctokit;

#[async_trait]
impl Octokit for MolOctokit {
  async fn on_event(&self, event: Events) -> anyhow::Result<()> {
    println!("Got: {:?}", event.name());
    let client = Client::new();

    let actions = match event {
      Events::PullRequest(event) => event.unwrap_actions(&client).await?,
      _ => vec![],
    };

    if actions.is_empty() {
      println!("Doing: Nothing");
    } else {
      for action in actions {
        action.execute(&client).await?;
      }
    }

    Ok(())
  }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  octokit_ntex::listen(
    "0.0.0.0:8081",
    MolOctokit,
    OctokitConfig {
      secret: "foobar2000".to_owned(),
    },
  )
  .await
}
