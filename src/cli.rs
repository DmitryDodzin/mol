use std::str::FromStr;

use crate::command::Publish;
use clap::Parser;

pub use mol_core::prelude::{PackageManager, Versioned};

pub use crate::command::*;

#[derive(Parser, Debug)]
pub enum Command {
  /// Create a new changeset file
  Add(Add),
  /// Initialize the (default .changsets) directory and basic config
  Init(Init),
  /// Consume changesets and update all relevant packages
  Version(Version),
  /// Publish the new versions of all the pacakges that were updated by mol version
  Publish(Publish),
}

impl<T: PackageManager + Send + Sync, V: Versioned + Send + Sync> IntoExecutableCommand<T, V>
  for Command
where
  <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T, V>> {
    match self {
      Self::Add(add) => Some(add as &dyn ExecutableCommand<T, V>),
      Self::Init(init) => Some(init as &dyn ExecutableCommand<T, V>),
      Self::Version(version) => Some(version as &dyn ExecutableCommand<T, V>),
      Self::Publish(publish) => Some(publish as &dyn ExecutableCommand<T, V>),
    }
  }
}

#[derive(Parser, Debug)]
#[clap(name = "cargo-mol", author = "Dmitry Dodzin <d.dodzin@gmail.com>")]
pub struct Opts {
  /// Command
  #[clap(subcommand)]
  pub cmd: Command,

  /// Run with dry_run no files actually change
  #[clap(long)]
  pub dry_run: bool,

  /// Plugin paths
  #[clap(long)]
  pub plugins: Vec<String>,
}
