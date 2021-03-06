use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Publish {
  #[clap(short, long)]
  pub packages: Vec<String>,
  #[clap(long)]
  pub publish_args: Vec<String>,
}

#[async_trait]
impl<T: PackageManager + Send + Sync, V: VersionEditor + Send + Sync + 'static>
  ExecutableCommand<T, V> for Publish
{
  async fn execute(
    &self,
    context: &ExecutableContext<T, V>,
    plugins: Arc<PluginManager>,
  ) -> anyhow::Result<()> {
    plugins.pre_command("publish", &context.as_plugin())?;

    let graph = context.packages.as_package_graph();

    let packages = if self.packages.is_empty() {
      graph.update_order()
    } else {
      graph
        .update_order()
        .into_iter()
        .filter(|package| self.packages.contains(&package.name))
        .collect()
    };

    for package in &packages {
      if let Some(root_path) = package.path.parent() {
        context
          .package_manager
          .run_publish(root_path, self.publish_args.clone(), context.dry_run)
          .await?;

        if !context.dry_run {
          while !context.package_manager.check_version(package).await? {
            println!("Package didn't upate yet");

            tokio::time::sleep(Duration::from_secs(1)).await;
          }
        }
      }
    }

    plugins.post_command("publish", &context.as_plugin())?;

    Ok(())
  }
}
