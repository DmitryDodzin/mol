use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
pub struct Init;

#[derive(Clap, Debug)]
pub struct Add {
  /// Generate empty changeset
  #[clap(long)]
  pub empty: bool,
}

#[derive(Clap, Debug)]
pub struct Version;

#[derive(Clap, Debug)]
pub struct Publish;

#[derive(Clap, Debug)]
pub struct Status;

#[derive(Clap, Debug)]
pub struct Pre;

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
  /// Print debug info
  #[clap(short)]
  pub debug: bool,
  /// Command
  #[clap(subcommand)]
  pub cmd: Command,
}
