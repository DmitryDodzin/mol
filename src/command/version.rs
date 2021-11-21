use std::fmt::Debug;
// use std::path::Path;

use async_trait::async_trait;
use clap::Clap;
use tokio::fs;

use lightbringer_core::prelude::*;

use super::{Context, ExecuteableCommand};

#[derive(Clap, Debug)]
pub struct Version;

impl Version {
  async fn consume_changesets<T: PackageManager>(
    changesets: &Changesets,
    context: &Context<T>,
  ) -> Result<Bump<Semantic>, failure::Error> {
    let mut bump = Bump::default();
    let mut changeset_files_paths = Vec::new();

    let mut changeset_files = fs::read_dir(&changesets.directory).await?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          let raw_changeset = fs::read_to_string(&changeset_path).await?;

          bump.add(Changeset::<Semantic>::parse(&raw_changeset)?);

          if context.dry_run {
            println!("dry_run - delete: {:?}", changeset_path);
          } else {
            fs::remove_file(&changeset_path).await?;
          }

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    Ok(bump)
  }
}

#[async_trait]
impl<T: PackageManager + Send + Sync> ExecuteableCommand<T> for Version {
  async fn execute(
    &mut self,
    changesets: &Changesets,
    context: &Context<T>,
  ) -> Result<(), failure::Error> {
    let bump = Self::consume_changesets(changesets, context).await?;

    if bump.is_empty() {
      println!(
        "Sorry but no changesets found in {:?}",
        changesets.directory
      );

      return Ok(());
    }

    for (path, name, version) in &context.packages {
      if let Some(update) = bump.package(name).version() {
        let next_version = update.apply(version)?;

        if context.dry_run {
          println!("dry_run - version bump: {} -> {}", version, next_version);
        } else {
          context
            .package_manager
            .apply_version(path, &next_version)
            .await?;
        }

        if let Some(root_path) = path.parent() {
          let changelog_path = {
            let mut root_path = root_path.to_path_buf();
            root_path.push("CHANGELOG.md");
            root_path
          };

          if context.dry_run {
            println!(
              "dry_run - update changelog {:?} with {}",
              changelog_path,
              bump
                .package(name)
                .version()
                .map(|value| value.to_string())
                .unwrap_or_else(String::new)
            );
          } else {
            Changelog::update_changelog(changelog_path, next_version, &bump.package(name)).await?;
          }
        }
      }
    }

    Ok(())
  }
}
