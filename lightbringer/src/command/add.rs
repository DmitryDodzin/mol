use std::collections::HashMap;

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
}

fn select_version() -> Result<Version, failure::Error> {
  let versions = vec![Version::Patch, Version::Minor, Version::Major];
  let version_selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("version")
    .items(&versions)
    .default(0)
    .interact_on_opt(&Term::buffered_stderr())?
    .unwrap();

  Ok(versions[version_selection])
}

fn select_package(packages: Vec<String>) -> Result<String, failure::Error> {
  let package_selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("package")
    .items(&packages)
    .default(0)
    .interact_on_opt(&Term::buffered_stderr())?
    .unwrap();

  Ok(packages[package_selection].clone())
}

impl Command for Add {
  fn run(&self, context: &Lightbringer) -> Result<(), failure::Error> {
    let changset = if self.empty {
      Changeset::default()
    } else {
      let mut packages = HashMap::new();
      let package = select_package(context.get_packages())?;
      let version = select_version()?;

      packages.insert(package, version);

      let message = Input::<String>::with_theme(&ColorfulTheme::default())
        .allow_empty(true)
        .interact_on(&Term::buffered_stderr())?;

      Changeset { packages, message }
    };

    println!("{}", changset.to_string());

    Ok(())
  }
}
