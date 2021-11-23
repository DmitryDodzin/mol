use std::fmt::Debug;

use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use tokio::fs;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Version;

impl Version {
  async fn consume_changesets<T: PackageManager, V: Versioned + Default>(
    changesets: &Changesets,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<Bump<V>> {
    let mut bump = Bump::default();
    let mut changeset_files_paths = Vec::new();

    let mut changeset_files = fs::read_dir(&changesets.directory).await.with_context(|| {
      format!(
        "Unable to read the changesets directory at {:?}",
        changesets.directory
      )
    })?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          let raw_changeset = fs::read_to_string(&changeset_path)
            .await
            .with_context(|| format!("Unable to read the changeset at {:?}", changeset_path))?;

          bump.add(
            Changeset::<V>::parse(&raw_changeset)
              .with_context(|| format!("Unable to parse changeset at {:?}", changeset_path))?,
          );

          if context.dry_run {
            println!("dry_run - delete: {:?}", changeset_path);
          } else {
            fs::remove_file(&changeset_path)
              .await
              .with_context(|| format!("Unable to remove the changeset at {:?}", changeset_path))?;
          }

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    Ok(bump)
  }
}

#[async_trait]
impl<T: PackageManager + Send + Sync, V: Versioned + Default + Send + Sync> ExecutableCommand<T, V>
  for Version
{
  async fn execute(
    &self,
    changesets: &Changesets,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<()> {
    let bump = Self::consume_changesets(changesets, context).await?;

    if bump.is_empty() {
      println!(
        "Sorry but no changesets found in {:?}",
        changesets.directory
      );

      return Ok(());
    }

    for package in &context.packages {
      if let Some(update) = bump.package(&package.name).version() {
        let next_version = update
          .apply(&package.version)
          .with_context(|| format!("Failed updating package {}", package.name))?;

        if context.dry_run {
          println!(
            "dry_run - version bump: {} -> {}",
            package.version, next_version
          );
        } else {
          context
            .package_manager
            .apply_version(&package.path, &next_version)
            .await?;
        }

        if let Some(root_path) = package.path.parent() {
          let changelog_path = {
            let mut root_path = root_path.to_path_buf();
            root_path.push("CHANGELOG.md");
            root_path
          };

          Changelog::update_changelog(
            &changelog_path,
            next_version,
            &bump.package(&package.name),
            context.dry_run,
          )
          .await
          .with_context(|| {
            format!(
              "Could not update the changelog for {} at {:?}",
              package.name, changelog_path
            )
          })?;
        }
      }
    }

    Ok(())
  }
}
