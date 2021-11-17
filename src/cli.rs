use crate::command::Version;
use clap::{AppSettings, Clap};

pub use crate::command::*;

#[derive(Clap, Debug)]
pub enum Command {
  Init(Init),
  /// Add changeset
  Add(Add),
  Version(Version),
  Publish(Publish),
  Status(Status),
}

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Dmitry Dodzin <d.dodzin@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
  /// Command
  #[clap(subcommand)]
  pub cmd: Command,
}
