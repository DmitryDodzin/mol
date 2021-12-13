use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Init;

#[async_trait]
impl<T: PackageManager + Send + Sync, V: VersionEditor + Send + Sync> ExecutableCommand<T, V>
  for Init
{
  async fn execute(
    &self,
    context: &ExecutableContext<T, V>,
    plugins: Arc<PluginManager>,
  ) -> anyhow::Result<()> {
    plugins.pre_command("init", &context.as_plugin())?;

    if !context.dry_run {
      context.changesets.initialize().await?;
    }

    plugins.post_command("init", &context.as_plugin())?;

    Ok(())
  }
}
