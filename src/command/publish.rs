use std::fmt::Debug;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Publish {
  #[clap(long)]
  pub publish_args: Vec<String>,
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
      if let Some(root_path) = package.path.parent() {
        context
          .package_manager
          .run_publish(root_path, self.publish_args.clone(), context.dry_run)
          .await?;
      }
    }

    Ok(())
  }
}
