use async_trait::async_trait;
use clap::Clap;
use tokio::fs;

use lightbringer_core::prelude::*;

use super::{Context, ExecuteableCommand};

#[derive(Clap, Debug)]
pub struct Version;

#[async_trait]
impl ExecuteableCommand for Version {
  async fn execute(
    &mut self,
    changesets: &Changesets,
    context: &Context,
  ) -> Result<(), failure::Error> {
    let mut concat_changeset = Changeset::<Semantic>::default();

    let mut changeset_files = fs::read_dir(&changesets.directory).await?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          let raw_changeset = fs::read_to_string(changeset_path).await?;

          let changeset = Changeset::<Semantic>::parse(&raw_changeset)?;

          for (name, version) in &changeset.packages {
            if let Some(concat_version) = concat_changeset.packages.get_mut(name) {
              if version > concat_version {
                *concat_version = version.clone();
              }
            } else {
              concat_changeset
                .packages
                .insert(name.to_owned(), version.clone());
            }
          }

          concat_changeset.message.push_str(&changeset.message);
        }
      }
    }

    for (_, name, version) in &context.packages {
      if let Some(patch) = concat_changeset.packages.get(name) {
        let next_version = patch.apply(version)?;
        println!("{}: {}", name, next_version);
      }
    }

    Ok(())
  }
}
