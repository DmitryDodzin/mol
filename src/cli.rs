use crate::command::Version;
use clap::Parser;

pub use mol_core::prelude::*;

pub use crate::command::*;

#[derive(Parser, Debug)]
pub enum Command {
  Add(Add),
  Init(Init),
  Version(Version),
}

impl<T: PackageManager + Send + Sync> IntoExecutableCommand<T> for Command {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T>> {
    match self {
      Self::Add(add) => Some(add as &dyn ExecutableCommand<T>),
      Self::Init(init) => Some(init as &dyn ExecutableCommand<T>),
      Self::Version(version) => Some(version as &dyn ExecutableCommand<T>),
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
}
