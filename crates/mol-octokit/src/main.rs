use octokit_ntex::Octokit;

struct MolOctokit;

impl Octokit for MolOctokit {
  fn on_event(&self, event: octokit_webhooks::Events) {
    println!("got event {:?}", event);
  }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  octokit_ntex::listen(MolOctokit).await
}
