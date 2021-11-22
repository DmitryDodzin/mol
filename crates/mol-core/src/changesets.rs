use std::path::PathBuf;
use tokio::fs::create_dir;

pub struct Changesets {
  pub directory: PathBuf,
}

impl Changesets {
  pub fn validate(&self) -> bool {
    self.directory.exists()
  }

  pub async fn initialize(&self) -> Result<(), std::io::Error> {
    create_dir(&self.directory).await?;
    Ok(())
  }
}

impl Default for Changesets {
  fn default() -> Self {
    Changesets {
      directory: [".changeset"].iter().collect(),
    }
  }
}
