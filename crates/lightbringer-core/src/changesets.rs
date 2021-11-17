use std::path::PathBuf;

pub struct Changesets {
  pub directory: PathBuf,
}

impl Changesets {
  pub fn validate(&self) -> bool {
    self.directory.exists()
  }
}

impl Default for Changesets {
  fn default() -> Self {
    Changesets {
      directory: [".changeset"].iter().collect(),
    }
  }
}
