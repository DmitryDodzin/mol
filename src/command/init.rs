use crate::INIT_EXISTS_PROMPT;
use std::fmt::Debug;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

use super::{ExecutableCommand, ExecutableContext};

#[derive(Parser, Debug)]
pub struct Init;

#[async_trait]
impl<T: PackageManager + Send + Sync, V: Versioned + Default + Send + Sync> ExecutableCommand<T, V>
  for Init
{
  async fn execute(
    &self,
    changesets: &Changesets,
    _context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<()> {
    if !changesets.validate() {
      changesets.initialize().await?;
    } else {
      println!("{}", *INIT_EXISTS_PROMPT);
    }

    Ok(())
  }
}
