use std::ops::Deref;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use thiserror::Error;
use tokio::{fs, process::Command};
use toml_edit::{value, Document};

use mol_core::prelude::*;

mod crates_api;
mod loader;

use crates_api::CratesResult;
use loader::CrateLoader;

#[derive(Error, Debug)]
pub enum CargoValidationError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("cargo workspace-inheritance is currently isn't supported")]
  WorkspaceInheritanceIsntSupported,
}

pub struct CargoBuild;

#[async_trait]
impl PackageManagerCommandWithArgs<CrateMetadata> for CargoBuild {
  type Error = std::io::Error;

  async fn execute_with_args(
    context: &CommandContext<'_, CrateMetadata>,
    args: Vec<String>,
  ) -> Result<(), Self::Error> {
    Cargo::run_command(
      "build",
      context.path,
      args.iter().map(Deref::deref).collect(),
    )
    .await
  }
}

pub struct CargoPublish;

#[async_trait]
impl PackageManagerCommandWithArgs<CrateMetadata> for CargoPublish {
  type Error = std::io::Error;

  async fn execute_with_args(
    context: &CommandContext<'_, CrateMetadata>,
    args: Vec<String>,
  ) -> Result<(), Self::Error> {
    let args = if context.dry_run {
      vec!["--dry-run"]
        .into_iter()
        .chain(args.iter().map(Deref::deref))
        .collect()
    } else {
      args.iter().map(Deref::deref).collect()
    };

    Cargo::run_command("update", &context.path, vec![]).await?;

    Cargo::run_command("publish", context.path, args).await?;

    Ok(())
  }
}

pub struct CargoValidate;

#[async_trait]
impl PackageManagerCommand<CrateMetadata> for CargoValidate {
  type Error = CargoValidationError;

  async fn execute(context: &CommandContext<'_, CrateMetadata>) -> Result<(), Self::Error> {
    let (_, document) = Cargo::load_document(context.path).await?;

    if document.contains_key("workspace") {
      if let Some(workspace) = document["workspace"].as_table() {
        if workspace.contains_key("package") || workspace.contains_key("dependencies") {
          return Err(CargoValidationError::WorkspaceInheritanceIsntSupported);
        }
      }
    }

    Ok(())
  }
}

#[derive(Clone)]
pub struct CrateMetadata {}

#[derive(Default)]
pub struct Cargo;

impl Cargo {
  async fn run_command<T: AsRef<Path> + Send + Sync>(
    command: &str,
    crate_path: T,
    args: Vec<&str>,
  ) -> Result<(), std::io::Error> {
    if let Ok(canon_path) = dunce::canonicalize(crate_path) {
      Command::new("cargo")
        .current_dir(canon_path)
        .arg(command)
        .args(args)
        .spawn()
        .expect("Cargo command failed to start")
        .wait()
        .await?;
    }

    Ok(())
  }
  async fn load_document<T: AsRef<Path>>(
    crate_path: T,
  ) -> Result<(PathBuf, Document), std::io::Error> {
    let document = fs::read_to_string(&crate_path)
      .await?
      .parse::<Document>()
      .expect("Invalid Cargo.toml");

    Ok((crate_path.as_ref().to_path_buf(), document))
  }
}

#[async_trait]
impl PackageManager for Cargo {
  type Loader = CrateLoader;
  type Metadata = CrateMetadata;
  type Build = CargoBuild;
  type Publish = CargoPublish;
  type Validate = CargoValidate;

  fn default_path() -> &'static str {
    "Cargo.toml"
  }

  async fn load_metadata<T: AsRef<Path> + Send + Sync>(
    _crate_path: T,
  ) -> anyhow::Result<Self::Metadata> {
    Ok(CrateMetadata {})
  }

  async fn check_version<V: Versioned + Send + Sync + 'static>(
    &self,
    package: &Package<V>,
    _: &Self::Metadata,
  ) -> anyhow::Result<bool> {
    let crates_result = crates_api::fetch_version(package).await?;

    match crates_result {
      CratesResult::Ok(val) => Ok(val.version.name == package.name),
      CratesResult::Err { errors } => {
        for error in errors {
          println!("crates-error:\n{}", error.detail);
        }
        Ok(false)
      }
    }
  }

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
    _: &Self::Metadata,
  ) -> anyhow::Result<()> {
    let (crate_path, mut document) = Self::load_document(crate_path).await?;

    if document.contains_key("package") {
      document["package"]["version"] = value(version);
    }

    fs::write(&crate_path, document.to_string()).await?;

    Ok(())
  }

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    name: &str,
    version: &str,
    _: &Self::Metadata,
  ) -> anyhow::Result<()> {
    let (crate_path, mut document) = Self::load_document(crate_path).await?;

    if document.contains_key("dependencies") {
      let dep = &document["dependencies"][name];

      if dep.is_inline_table() {
        document["dependencies"][name]["version"] = value(version);
      } else if dep.is_str() {
        document["dependencies"][name] = value(version);
      }
    }

    fs::write(&crate_path, document.to_string()).await?;

    Ok(())
  }
}
