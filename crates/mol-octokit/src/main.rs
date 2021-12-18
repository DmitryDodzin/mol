use async_trait::async_trait;

use octokit_ntex::{Octokit, OctokitConfig};
use octokit_webhooks::*;

struct MolOctokit;

#[async_trait]
impl Octokit for MolOctokit {
  async fn on_event(&self, event: Events) {
    println!("Got event {:?}", event.name());
  }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  octokit_ntex::listen(
    MolOctokit,
    OctokitConfig {
      secret: "foobar2000".to_owned(),
    },
  )
  .await
}
