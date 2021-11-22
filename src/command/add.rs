use std::path::PathBuf;
use std::str::FromStr;

use async_trait::async_trait;
use clap::Clap;
use dialoguer::{console::Term, Input, MultiSelect, Select};
use faker_rand::lorem::Word;
use rand::Rng;

use lightbringer_core::prelude::*;

use super::{Context, ExecuteableCommand};
use crate::COLOR_THEME;

#[derive(Clap, Debug)]
pub struct Add {
  /// Generate empty changeset
  #[clap(long)]
  pub empty: bool,
  #[clap(short, long)]
  pub packages: Option<Vec<String>>,
  #[clap(long)]
  pub version: Option<String>,
  #[clap(long)]
  pub patch: bool,
  #[clap(long)]
  pub minor: bool,
  #[clap(long)]
  pub major: bool,
  #[clap(short, long)]
  pub message: Option<String>,
}

impl Add {
  fn select_version(&self) -> Result<Version<Semantic>, failure::Error> {
    if let Some(version) = &self.version {
      return Version::<Semantic>::from_str(version).map_err(|err| err.into());
    }

    if self.major || self.minor || self.patch {
      return Ok(match (self.major, self.minor, self.patch) {
        (true, _, _) => Version::new(Semantic::major()),
        (false, true, _) => Version::new(Semantic::minor()),
        (false, false, true) => Version::new(Semantic::patch()),
        (false, false, false) => unreachable!(),
      });
    }

    let versions = vec![
      Version::new(Semantic::patch()),
      Version::new(Semantic::minor()),
      Version::new(Semantic::major()),
    ];
    let version_selection = Select::with_theme(&*COLOR_THEME)
      .with_prompt("version")
      .items(&versions)
      .default(0)
      .interact_on(&Term::buffered_stderr())?;

    Ok(versions[version_selection].clone())
  }

  fn select_packages<T: PackageManager>(
    &self,
    context: &Context<T>,
  ) -> Result<Vec<(PathBuf, String, String)>, failure::Error> {
    if let Some(packages) = &self.packages {
      let packages = context
        .packages
        .iter()
        .filter(|(_, name, _)| packages.contains(name))
        .cloned()
        .collect();

      return Ok(packages);
    }

    let packages = MultiSelect::with_theme(&*COLOR_THEME)
      .with_prompt("packages")
      .items(
        &context
          .packages
          .iter()
          .map(|(_, name, _)| name)
          .collect::<Vec<&String>>(),
      )
      .interact_on(&Term::buffered_stderr())?
      .into_iter()
      .map(|index| context.packages[index].clone())
      .collect();

    Ok(packages)
  }

  fn get_changeset<T: PackageManager>(
    &mut self,
    context: &Context<T>,
  ) -> Result<Option<Changeset<Semantic>>, failure::Error> {
    let packages = self.select_packages(context)?;

    if packages.is_empty() {
      return Ok(None);
    }

    let version = self.select_version()?;

    let message = if self.empty {
      String::new()
    } else {
      match &self.message {
        Some(message) => message.clone(),
        None => Input::with_theme(&*COLOR_THEME)
          .with_prompt("message")
          .interact_on(&Term::buffered_stderr())?,
      }
    };

    let changeset: Changeset<Semantic> = Changeset {
      packages: packages
        .into_iter()
        .map(|(_, name, _)| (name, version.clone()))
        .collect(),
      message,
    };

    Ok(Some(changeset))
  }
}

#[async_trait]
impl<T: PackageManager + Send + Sync> ExecuteableCommand<T> for Add {
  async fn execute(
    &mut self,
    changesets: &Changesets,
    context: &Context<T>,
  ) -> Result<(), failure::Error> {
    if let Some(changeset) = self.get_changeset(context)? {
      let changeset_path = {
        let mut rng = rand::thread_rng();
        let mut path = changesets.directory.clone();

        path.push(format!("{}-{}.md", rng.gen::<Word>(), rng.gen::<Word>()));

        path
      };

      if context.dry_run {
        println!("{}", changeset.to_string());
      } else {
        changeset.save(changeset_path).await?;
      }
    }

    Ok(())
  }
}
