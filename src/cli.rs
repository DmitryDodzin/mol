use crate::command::Version;
use clap::Parser;

pub use crate::command::*;

#[derive(Parser, Debug)]
pub enum Command {
  Init(Init),
  /// Add changeset
  Add(Add),
  Version(Version),
  Publish(Publish),
  Status(Status),
}

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Dmitry Dodzin <d.dodzin@gmail.com>")]
pub struct Opts {
  /// Command
  #[clap(subcommand)]
  pub cmd: Command,

  /// Dry the changes
  #[clap(long)]
  pub dry_run: bool,
}
