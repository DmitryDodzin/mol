use anyhow::Context;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio::fs;

use crate::bump::Bump;
use crate::changeset::Changeset;
use crate::package::graph::PackageGraph;
use crate::version::VersionEditor;

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

  pub async fn consume<V: VersionEditor>(
    &self,
    package_graph: &PackageGraph<'_, V>,
  ) -> anyhow::Result<(Vec<PathBuf>, Bump<V>)> {
    let mut bump = Bump::default();
    let mut changeset_files_paths = Vec::new();

    let mut changeset_files = fs::read_dir(&self.directory).await.with_context(|| {
      format!(
        "Unable to read the changesets directory at {:?}",
        self.directory
      )
    })?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          if Some(OsStr::new("README.md")) == changeset_path.file_name() {
            continue;
          }

          let raw_changeset = fs::read_to_string(&changeset_path)
            .await
            .with_context(|| format!("Unable to read the changeset at {:?}", changeset_path))?;

          bump.add(
            Changeset::<V>::parse(&raw_changeset)
              .with_context(|| format!("Unable to parse changeset at {:?}", changeset_path))?,
            package_graph,
          );

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    Ok((changeset_files_paths, bump))
  }
}

impl Default for Changesets {
  fn default() -> Self {
    Changesets {
      directory: [".changeset"].iter().collect(),
    }
  }
}
