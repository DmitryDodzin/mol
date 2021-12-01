use std::path::PathBuf;
use tokio::fs;

pub struct Changesets {
  pub directory: PathBuf,
}

impl Changesets {
  fn readme_path(&self) -> PathBuf {
    let mut readme_path = self.directory.clone();
    readme_path.push("README.md");
    readme_path
  }

  pub fn validate(&self) -> bool {
    self.directory.exists() && self.readme_path().exists()
  }

  pub async fn initialize(&self) -> Result<(), std::io::Error> {
    if !self.directory.exists() {
      fs::create_dir(&self.directory).await?;
    }

    fs::write(self.readme_path(), b"# Changesets directory\n\nThis directory is for changeset files, can be createted with `mol add`\n\n").await?;

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
