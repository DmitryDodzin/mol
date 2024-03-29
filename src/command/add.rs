use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use clap::Parser;
use dialoguer::{console::Term, Input, MultiSelect, Select};

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};
use crate::{ADD_NO_PACKAGES, COLOR_THEME};

#[derive(Parser, Debug)]
pub struct Add {
  /// Generate empty changeset
  #[clap(long)]
  pub empty: bool,
  #[clap(short, long)]
  pub packages: Vec<String>,
  #[clap(short, long)]
  pub version: Option<String>,
  #[clap(short, long)]
  pub message: Option<String>,
}

impl Add {
  fn select_version<V>(&self) -> anyhow::Result<VersionMod<V>>
  where
    V: VersionEditor,
    <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
    if let Some(version) = &self.version {
      return Ok(VersionMod::<V>::from_str(version)?);
    }

    let versions = VersionMod::<V>::options();
    let version_selection = Select::with_theme(&*COLOR_THEME)
      .with_prompt("version")
      .items(&versions)
      .default(0)
      .interact_on(&Term::buffered_stderr())?;

    Ok(versions[version_selection].clone())
  }

  fn select_packages<T: PackageManager, V: VersionEditor>(
    &self,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<Vec<Package<V>>> {
    if context.packages.is_empty() {
      return Err(anyhow::Error::msg(
        "No packages were found in current directory",
      ));
    }

    if !self.packages.is_empty() {
      let packages = context
        .packages
        .iter()
        .filter(|package| self.packages.contains(&package.name))
        .cloned()
        .collect();

      return Ok(packages);
    }

    if context.packages.len() == 1 {
      return Ok(context.packages.clone());
    }

    let packages = MultiSelect::with_theme(&*COLOR_THEME)
      .with_prompt("packages")
      .items(
        &context
          .packages
          .iter()
          .map(|package| package.name.clone())
          .collect::<Vec<String>>(),
      )
      .interact_on(&Term::buffered_stderr())?
      .into_iter()
      .map(|index| context.packages[index].clone())
      .collect();

    Ok(packages)
  }

  fn get_changeset<T, V>(
    &self,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<Option<Changeset<V>>>
  where
    T: PackageManager,
    V: VersionEditor,
    <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
  {
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

    let changeset: Changeset<V> = Changeset {
      packages: packages
        .into_iter()
        .map(|package| (package.name, version.clone()))
        .collect(),
      message,
    };

    Ok(Some(changeset))
  }
}

#[async_trait]
impl<T, V> ExecutableCommand<T, V> for Add
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
  <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
  async fn execute(
    &self,
    context: &ExecutableContext<T, V>,
    plugins: Arc<PluginManager>,
  ) -> anyhow::Result<()> {
    plugins.pre_command("add", &context.as_plugin())?;

    if let Some(changeset) = self.get_changeset(context)? {
      let changeset_path = {
        let mut path = context.changesets.directory.clone();

        path.push(Changeset::<V>::random_file_name());

        path
      };

      if context.dry_run {
        println!("{}", changeset.to_string());
      } else {
        changeset
          .save(&changeset_path)
          .await
          .with_context(|| format!("Could not save the changset at {:?}", changeset_path))?;
      }
    } else {
      println!("{}", &*ADD_NO_PACKAGES);
    }

    plugins.post_command("add", &context.as_plugin())?;

    Ok(())
  }
}
