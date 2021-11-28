use std::fmt::Debug;

use async_trait::async_trait;
use clap::Parser;
use tokio::process::Command;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Publish {
  #[clap(long)]
  pub publish_args: Option<Vec<String>>,
}

#[async_trait]
impl<T: PackageManager + Send + Sync, V: Versioned + Send + Sync> ExecutableCommand<T, V>
  for Publish
{
  async fn execute(
    &self,
    _changesets: &Changesets,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<()> {
    let graph = context.packages.as_package_graph();

    for package in &graph.update_order() {
      if let Ok(canon_path) = dunce::canonicalize(&package.path) {
        if let Some(directory) = canon_path.parent() {
          let mut cmd = Command::new("cargo");

          cmd.current_dir(directory).arg("publish");

          if context.dry_run {
            cmd.arg("--dry-run");
          }

          if let Some(args) = &self.publish_args {
            cmd.args(args);
          }

          cmd
            .spawn()
            .expect("cargo command failed to start")
            .wait()
            .await?;
        }
      }
    }

    Ok(())
  }
}
