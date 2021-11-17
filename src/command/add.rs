use std::path::PathBuf;
use std::str::FromStr;

use clap::Clap;
use dialoguer::{Input, MultiSelect, Select};

use lightbringer_core::prelude::*;

use super::Context;
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
      .interact()?;

    Ok(versions[version_selection].clone())
  }

  fn select_packages(
    &self,
    context: &Context,
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
      .interact()?
      .into_iter()
      .map(|index| context.packages[index].clone())
      .collect();

    Ok(packages)
  }

  pub fn get_changeset(
    &mut self,
    context: &Context,
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
          .interact_text()?,
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
