use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;

use mol_core::prelude::*;

mod add;
mod version;

#[derive(Debug)]
pub struct Context<T: PackageManager> {
  pub dry_run: bool,
  pub package_manager: T,
  pub packages: Vec<(PathBuf, String, String)>,
}

#[async_trait]
pub trait ExecuteableCommand<T: PackageManager> {
  async fn execute(
    &mut self,
    changesets: &Changesets,
    context: &Context<T>,
  ) -> Result<(), failure::Error>;
}

pub use add::Add;
pub use version::Version;

#[derive(Parser, Debug)]
pub struct Init;

#[derive(Parser, Debug)]
pub struct Publish;

#[derive(Parser, Debug)]
pub struct Status;

#[derive(Parser, Debug)]
pub struct Pre;
