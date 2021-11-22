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

impl<T: PackageManager + Send + Sync> IntoExecuteableCommand<T> for Command {
  fn as_executable(&self) -> Option<&dyn ExecuteableCommand<T>> {
    match self {
      Self::Add(add) => Some(add as &dyn ExecuteableCommand<T>),
      Self::Version(version) => Some(version as &dyn ExecuteableCommand<T>),
      _ => None,
    }
  }
}

#[derive(Parser, Debug)]
pub struct CommandTarget {
  #[clap(subcommand)]
  pub target: Command,
}

#[derive(Parser, Debug)]
pub enum Root {
  Mol(CommandTarget),
  Init(Init),
  Add(Add),
  Version(Version),
  Publish(Publish),
  Status(Status),
}

impl<T: PackageManager + Send + Sync> IntoExecuteableCommand<T> for Root {
  fn as_executable(&self) -> Option<&dyn ExecuteableCommand<T>> {
    match self {
      Self::Add(add) => Some(add as &dyn ExecuteableCommand<T>),
      Self::Version(version) => Some(version as &dyn ExecuteableCommand<T>),
      _ => None,
    }
  }
}

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Dmitry Dodzin <d.dodzin@gmail.com>")]
pub struct Opts {
  /// Command
  #[clap(subcommand)]
  pub cmd: Root,

  /// Dry the changes
  #[clap(long)]
  pub dry_run: bool,
}
