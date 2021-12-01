use std::fmt::Debug;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Init;

#[async_trait]
impl<T: PackageManager + Send + Sync, V: Versioned + Send + Sync> ExecutableCommand<T, V> for Init {
  async fn execute(&self, context: &ExecutableContext<T, V>) -> anyhow::Result<()> {
    if !context.dry_run {
      context.changesets.initialize().await?;
    }

    Ok(())
  }
}
