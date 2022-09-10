use std::collections::HashMap;

use std::sync::Arc;

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
  #[clap(long)]
  pub build_args: Vec<String>,
}

#[async_trait]
impl<T, V> ExecutableCommand<T, V> for Version
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
  T::Metadata: Send + Sync,
{
  async fn execute(
    &self,
    context: &ExecutableContext<T, V>,
    plugins: Arc<PluginManager>,
  ) -> anyhow::Result<()> {
    plugins.pre_command("version", &context.as_plugin())?;

    let package_graph = context.packages.as_package_graph();
    let (changeset_paths, bump) = context.changesets.consume::<V>(&package_graph).await?;

    if bump.is_empty() {
      println!(
        "Sorry but no changesets found in {:?}",
        context.changesets.directory
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
            .apply_version(&package.path, &next_version, &context.metadata)
            .await?;
        }

        for (name, version, updated_version) in package
          .dependencies
          .iter()
          .filter(|(name, _)| updated.contains_key(name.as_str()))
          .filter(|(name, version)| !V::r#match(version, &updated[name.as_str()]))
          .map(|(name, version)| (name, version, &updated[name.as_str()]))
        {
          if context.dry_run {
            println!(
              "dry_run - dependecy version bump: {} {} -> {}",
              name,
              version,
              V::mask(version, updated_version)
            );
          } else {
            context
              .package_manager
              .apply_dependency_version(
                &package.path,
                name,
                V::mask(version, updated_version),
                &context.metadata,
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
      context
        .package_manager
        .run_build(
          &context.root_dir,
          self.build_args.clone(),
          &context.metadata,
        )
        .await?;
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

    plugins.post_command("version", &context.as_plugin())?;

    Ok(())
  }
}
