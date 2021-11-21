use std::path::PathBuf;

use async_trait::async_trait;
use clap::Clap;

use lightbringer_core::prelude::*;

mod add;
mod version;

#[derive(Debug)]
pub struct Context<T: PackageManager> {
  pub packages: Vec<(PathBuf, String, String)>,
  pub package_manager: T,
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

#[derive(Clap, Debug)]
pub struct Init;

#[derive(Clap, Debug)]
pub struct Publish;

#[derive(Clap, Debug)]
pub struct Status;

#[derive(Clap, Debug)]
pub struct Pre;
