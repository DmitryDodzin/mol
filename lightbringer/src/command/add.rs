use std::collections::HashMap;
use std::str::FromStr;

use clap::Clap;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use lightbringer_core::{changeset::Changeset, version::Version, Lightbringer};

use super::Command;

#[derive(Clap, Debug)]
pub struct Add {
  /// Generate empty changeset
  #[clap(long)]
  pub empty: bool,
  #[clap(short, long)]
  pub package: Option<String>,
  #[clap(short, long)]
  pub version: Option<String>,
  #[clap(short, long)]
  pub message: Option<String>,
}

impl Add {
  fn select_version(&self) -> Result<Version, failure::Error> {
    if let Some(version) = &self.version {
      return Version::from_str(version).map_err(|err| err.into());
    }

    let versions = vec![Version::Patch, Version::Minor, Version::Major];
    let version_selection = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("version")
      .items(&versions)
      .default(0)
      .interact_on_opt(&Term::buffered_stderr())?
      .unwrap();

    Ok(versions[version_selection])
  }

  fn select_package(&self, packages: Vec<String>) -> Result<String, failure::Error> {
    if self.package.is_some() {
      return Ok(self.package.clone().unwrap());
    }

    let package_selection = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("package")
      .items(&packages)
      .default(0)
      .interact_on_opt(&Term::buffered_stderr())?
      .unwrap();

    Ok(packages[package_selection].clone())
  }
}

impl Command for Add {
  fn run(&self, context: &Lightbringer) -> Result<(), failure::Error> {
    let changset = if self.empty {
      if let (Some(package), Some(version)) = (
        self.package.as_ref(),
        self
          .version
          .as_ref()
          .map_or(None, |version| Version::from_str(&version).ok()),
      ) {
        Changeset {
          packages: vec![(package.clone(), version)].into_iter().collect(),
          message: "".to_owned(),
        }
      } else {
        Changeset::default()
      }
    } else {
      let mut packages = HashMap::new();
      let package = self.select_package(context.get_packages())?;
      let version = self.select_version()?;

      packages.insert(package, version);

      let message = if self.message.is_some() {
        self.message.clone().unwrap()
      } else {
        Input::<String>::with_theme(&ColorfulTheme::default())
          .with_prompt("message")
          .allow_empty(true)
          .interact_on(&Term::buffered_stderr())?
      };

      Changeset { packages, message }
    };

    println!("{}", changset.to_string());

    Ok(())
  }
}
