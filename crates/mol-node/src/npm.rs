use std::ops::Deref;
use std::path::Path;

use async_trait::async_trait;
use tokio::process::Command;

use mol_core::prelude::*;

#[derive(Default)]
pub struct Npm;

impl Npm {
  async fn run_command<T: AsRef<Path> + Send + Sync>(
    &self,
    command: &str,
    crate_path: T,
    args: Vec<&str>,
  ) -> anyhow::Result<()> {
    if let Ok(canon_path) = dunce::canonicalize(crate_path) {
      Command::new("npm")
        .current_dir(canon_path)
        .arg(command)
        .args(args)
        .spawn()
        .expect("Npm command failed to start")
        .wait()
        .await?;
    }

    Ok(())
  }
}

#[async_trait]
impl PackageManager for Npm {
  fn default_path() -> &'static str {
    "package.json"
  }

  async fn read_package<T: AsRef<Path> + Send + Sync, V: Versioned + Send + Sync + 'static>(
    _crate_path: T,
  ) -> anyhow::Result<Vec<Package<V>>> {
    todo!()
  }

  async fn check_version<V: Versioned + Send + Sync + 'static>(
    &self,
    _package: &Package<V>,
  ) -> anyhow::Result<bool> {
    todo!()
  }

  async fn run_build<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    build_args: Vec<String>,
  ) -> anyhow::Result<()> {
    self
      .run_command(
        "build",
        crate_path,
        build_args.iter().map(Deref::deref).collect(),
      )
      .await
  }

  async fn run_publish<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    publish_args: Vec<String>,
    dry_run: bool,
  ) -> anyhow::Result<()> {
    if !dry_run {
      self
        .run_command(
          "publish",
          crate_path,
          publish_args.iter().map(Deref::deref).collect(),
        )
        .await?;
    }

    Ok(())
  }

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    _crate_path: T,
    _version: &str,
  ) -> anyhow::Result<()> {
    todo!()
  }

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    _crate_path: T,
    _name: &str,
    _version: &str,
  ) -> anyhow::Result<()> {
    todo!()
  }
}
