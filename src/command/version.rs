use async_trait::async_trait;
use clap::Clap;
use git2::Repository;
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
    let mut changeset_files_paths = Vec::new();

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

          if changeset_files_paths.len() != 0 {
            concat_changeset.message.push_str("\n\n");
          }
          concat_changeset.message.push_str(&changeset.message);

          changeset_files_paths.push(changeset_path);
        }
      }
    }

    println!("{}", concat_changeset.to_string());

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

    for (_, name, version, next) in updates {
      println!("bump {}:  {} -> {}", name, version, next);
    }

    println!("delete {:?}", changeset_files_paths);

    let mut repository = Repository::open("")?;

    let signature = repository.signature()?;

    println!("{:?} {:?}", signature.name(), signature.email());

    let (head, next_barnch) = {
      let head = repository.head()?;

      let head_commit = head.peel_to_commit()?;

      let next_branch = repository.branch("lightbringer/version", &head_commit, true)?;

      (
        head.name().map(|val| val.to_owned()),
        next_branch.name()?.map(|val| val.to_owned()),
      )
    };

    let stash = repository.stash_save2(&signature, None, None).ok();

    println!("{:?} -> {:?} ({:?})", head, next_barnch, stash);

    Ok(())
  }
}
