use crate::command::Version;
use clap::Parser;

pub use mol_core::prelude::*;

pub use crate::command::*;

#[derive(Parser, Debug)]
pub enum Command {
  Init(Init),
  Add(Add),
  Version(Version),
  Publish(Publish),
  Status(Status),
}

impl<T: PackageManager + Send + Sync> IntoExecutableCommand<T> for Command {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T>> {
    match self {
      Self::Add(add) => Some(add as &dyn ExecutableCommand<T>),
      Self::Version(version) => Some(version as &dyn ExecutableCommand<T>),
      _ => None,
    }
  }
}

#[derive(Parser, Debug)]
#[clap(name = "cargo-mol", author = "Dmitry Dodzin <d.dodzin@gmail.com>")]
pub struct Opts {
  /// Command
  #[clap(subcommand)]
  pub cmd: Command,

  /// Dry the changes
  #[clap(long)]
  pub dry_run: bool,
}
