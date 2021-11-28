use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use tokio::fs;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Version {
  #[clap(long)]
  pub no_build: bool,
}

impl Version {
  async fn consume_changesets<V: Versioned>(
    changesets: &Changesets,
  ) -> anyhow::Result<(Vec<PathBuf>, Bump<V>)> {
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

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    Ok((changeset_files_paths, bump))
  }
}

#[async_trait]
impl<T: PackageManager + Send + Sync, V: Versioned + Send + Sync> ExecutableCommand<T, V>
  for Version
{
  async fn execute(
    &self,
    changesets: &Changesets,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<()> {
    let (changeset_paths, bump) = Self::consume_changesets::<V>(changesets).await?;

    let package_graph = context.packages.as_package_graph();

    if bump.is_empty() {
      println!(
        "Sorry but no changesets found in {:?}",
        changesets.directory
      );

      return Ok(());
    }

    let mut updated = HashMap::new();

    for package in package_graph.update_order() {
      if let Some(update) = bump.package(&package.name).version() {
        let next_version = update
          .apply(&package.version.value)
          .with_context(|| format!("Failed updating package {}", package.name))?;

        updated.insert(package.name.as_str(), next_version.clone());

        if context.dry_run {
          println!(
            "dry_run - version bump: {} -> {}",
            package.version.value, next_version
          );
        } else {
          context
            .package_manager
            .apply_version(&package.path, &next_version)
            .await?;
        }

        for (name, version) in package
          .dependencies
          .iter()
          .filter(|(name, _)| updated.contains_key(name.as_str()))
        {
          if context.dry_run {
            println!(
              "dry_run - dependecy version bump: {} {} -> {}",
              name,
              version,
              V::mask(version, &updated[name.as_str()])
            );
          } else {
            context
              .package_manager
              .apply_dependency_version(
                &package.path,
                name,
                V::mask(version, &updated[name.as_str()]),
              )
              .await?;
          }
        }

        if let Some(root_path) = package.path.parent() {
          let changelog_path = {
            let mut root_path = root_path.to_path_buf();
            root_path.push("CHANGELOG.md");
            root_path
          };

          Changelog::update_changelog(
            &changelog_path,
            next_version.into(),
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

    if !context.dry_run && !self.no_build {
      context.package_manager.run_build(".").await?;
    }

    for changeset_path in changeset_paths {
      if context.dry_run {
        println!("dry_run - delete: {:?}", changeset_path);
      } else {
        fs::remove_file(&changeset_path)
          .await
          .with_context(|| format!("Unable to remove the changeset at {:?}", changeset_path))?;
      }
    }

    Ok(())
  }
}
