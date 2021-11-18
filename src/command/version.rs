use std::path::{Path, PathBuf};

use async_trait::async_trait;
use clap::Clap;
// use git2::Repository;
use tokio::fs;

use lightbringer_core::prelude::*;

use super::{Context, ExecuteableCommand};

#[derive(Clap, Debug)]
pub struct Version;

impl Version {
  async fn consume_changesets(
    changesets: &Changesets,
  ) -> Result<(Changeset<Semantic>, Vec<PathBuf>), failure::Error> {
    let mut concat_changeset = Changeset::default();
    let mut changeset_files_paths = Vec::new();

    let mut changeset_files = fs::read_dir(&changesets.directory).await?;

    while let Some(changeset) = changeset_files.next_entry().await? {
      let changeset_path = changeset.path();

      if let Some(ext) = changeset_path.extension() {
        if ext == "md" {
          let raw_changeset = fs::read_to_string(&changeset_path).await?;

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

          if !changeset_files_paths.is_empty() {
            concat_changeset.message.push_str("\n\n");
          }
          concat_changeset.message.push_str(&changeset.message);

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    for path in &changeset_files_paths {
      fs::remove_file(&path).await?
    }

    Ok((concat_changeset, changeset_files_paths))
  }

  async fn update_changelog<T: Versioned, P: AsRef<Path>>(
    changelog: P,
    changeset: &Changeset<T>,
  ) -> Result<(), failure::Error> {
    let changelog = if changelog.as_ref().exists() {
      fs::read_to_string(changelog).await?
    } else {
      String::new()
    };

    println!("{}", changelog);

    println!("{}", changeset.to_string());

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

    let (concat_changeset, changeset_files_paths) = Self::consume_changesets(changesets).await?;

    if changeset_files_paths.is_empty() {
      println!(
        "Sorry but no changesets found in {:?}",
        changesets.directory
      );

      return Ok(());
    }

    Self::update_changelog("CHANGELOG.md", &concat_changeset).await?;

    let updates = context.packages.iter().filter_map(|(path, name, version)| {
      concat_changeset
        .packages
        .get(name)
        .map(|patch| {
          patch
            .apply(version)
            .ok()
            .map(|next| (path, name, version, next))
        })
        .flatten()
    });

    for (crate_path, name, current_version, next_version) in updates {
      println!(
        "{:?} {:?} {:?} {:?}",
        crate_path, name, current_version, next_version
      );

      lightbringer_cargo::apply_version(crate_path, &next_version).await?;
    }

    Ok(())
  }
}
