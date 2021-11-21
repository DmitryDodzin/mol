use std::fmt::Debug;
use std::path::Path;

use async_trait::async_trait;
use clap::Clap;
// use git2::Repository;
use tokio::fs;

use lightbringer_core::prelude::*;

use super::{Context, ExecuteableCommand};

#[derive(Clap, Debug)]
pub struct Version;

impl Version {
  async fn consume_changesets(changesets: &Changesets) -> Result<Bump<Semantic>, failure::Error> {
    let mut bump = Bump::default();
    let mut changeset_files_paths = Vec::new();

    let mut changeset_files = fs::read_dir(&changesets.directory).await?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          let raw_changeset = fs::read_to_string(&changeset_path).await?;

          bump.add(Changeset::<Semantic>::parse(&raw_changeset)?);

          fs::remove_file(&changeset_path).await?;

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    println!("{:?}", changeset_files_paths);

    Ok(bump)
  }

  async fn update_changelog<T: Versioned + Debug, P: AsRef<Path>>(
    changelog: P,
    bump: &Bump<T>,
  ) -> Result<(), failure::Error> {
    let changelog = if changelog.as_ref().exists() {
      fs::read_to_string(changelog).await?
    } else {
      String::new()
    };

    println!("{}", changelog);
    println!("{:#?}", bump);

    Ok(())
  }
}

#[async_trait]
impl ExecuteableCommand for Version {
  async fn execute(
    &mut self,
    changesets: &Changesets,
    context: &Context,
  ) -> Result<(), failure::Error> {
    println!("{:#?}", context);

    // {
    //   let mut repository = Repository::open(".")?;

    //   let signature = repository.signature()?;

    //   let (head, next_barnch) = {
    //     let head = repository.head()?;

    //     let head_commit = head.peel_to_commit()?;

    //     let next_branch = repository.branch("lightbringer/version", &head_commit, true)?;

    //     (
    //       head.name().map(|val| val.to_owned()),
    //       next_branch.name()?.map(|val| val.to_owned()),
    //     )
    //   };

    //   let stash = repository.stash_save2(&signature, None, None).ok();

    //   println!("{:?} -> {:?} ({:?})", head, next_barnch, stash);
    // }

    let bump = Self::consume_changesets(changesets).await?;

    // for (path, name, version) in &context.packages {
    //   if let Some(update) = bump.package(name).version() {
    //     lightbringer_cargo::apply_version(path, &update.apply(version)?).await?;
    //   }
    // }

    // for (name, version) in bump.updates() {
    //   lightbringer_cargo::apply_version(crate_path, &version).await?;
    // }

    // if changeset_files_paths.is_empty() {
    //   println!(
    //     "Sorry but no changesets found in {:?}",
    //     changesets.directory
    //   );

    //   return Ok(());
    // }

    Self::update_changelog("CHANGELOG.md", &bump).await?;

    println!("{:?}", bump.package("lightbringer").changesets());

    // let updates = context.packages.iter().filter_map(|(path, name, version)| {
    //   concat_changeset
    //     .packages
    //     .get(name)
    //     .map(|patch| {
    //       patch
    //         .apply(version)
    //         .ok()
    //         .map(|next| (path, name, version, next))
    //     })
    //     .flatten()
    // });

    // for (crate_path, name, current_version, next_version) in updates {
    //   println!(
    //     "{:?} {:?} {:?} {:?}",
    //     crate_path, name, current_version, next_version
    //   );

    //   lightbringer_cargo::apply_version(crate_path, &next_version).await?;
    // }

    Ok(())
  }
}
