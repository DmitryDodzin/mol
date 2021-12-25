use async_trait::async_trait;

use octokit_ntex::{Octokit, OctokitConfig};
use octokit_webhooks::*;

mod actions;
mod events;
mod octokit;

use actions::UnwrapActions;

struct MolOctokit;

#[async_trait]
impl Octokit for MolOctokit {
  async fn on_event(&self, event: Events) {
    println!("Got: {:?}", event.name());

    let actions = match event {
      Events::PullRequest(event) => event.unwrap_actions().await,
      _ => vec![],
    };

    if actions.is_empty() {
      println!("Doing: Nothing");
    } else {
      for action in actions {
        println!("Doing: {:#?}", action);

        if let Err(err) = action.execute().await {
          println!("{:?}", err);
        }
      }
    }
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
