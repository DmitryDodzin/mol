use async_trait::async_trait;
use clap::Parser;

use octokit_hyper::prelude::*;
use octokit_ntex::{Octokit, OctokitConfig};
use octokit_webhooks::Events;

mod actions;
mod events;

use actions::UnwrapActions;

#[derive(Parser)]
struct MolOctokit {
  #[clap(long, env = "SERVICE_ADDRESS", default_value = "0.0.0.0:8081")]
  address: String,
  #[clap(long, env = "SERVICE_SECRET", default_value = "secret")]
  secret: String,
}

impl MolOctokit {
  fn config(&self) -> (String, OctokitConfig) {
    (
      self.address.clone(),
      OctokitConfig {
        secret: self.secret.clone(),
      },
    )
  }
}

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
    }

    for action in actions {
      action.execute(&client).await?;
    }

    Ok(())
  }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  let octokit = MolOctokit::parse();

  let (address, config) = octokit.config();

  octokit_ntex::listen(address, octokit, config).await
}
