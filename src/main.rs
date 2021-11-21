use clap::Clap;
use dialoguer::{console, theme::ColorfulTheme};
use lazy_static::lazy_static;

use lightbringer_cargo::Cargo;
use lightbringer_core::prelude::*;

mod cli;
mod command;

use cli::{Command, Opts};
use command::{Context, ExecuteableCommand};

lazy_static! {
  pub(crate) static ref COLOR_THEME: ColorfulTheme = ColorfulTheme {
    unchecked_item_prefix: console::style("✘".to_owned()).for_stderr().red(),
    ..Default::default()
  };
  static ref INIT_REQ_PROMPT: console::StyledObject<&'static str> =
    console::style("Changesets folder validation failed run 'init'").yellow();
  static ref INIT_EXISTS_PROMPT: console::StyledObject<&'static str> =
    console::style("Changesets folder already initialized").yellow();
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
  let opts = Opts::parse();

  let changesets = Changesets::default();

  let package_manager = Cargo::default();

  let context = Context {
    dry_run: opts.dry_run,
    packages: package_manager.read_package("Cargo.toml").await?,
    package_manager,
  };

  match opts.cmd {
    Command::Init(_) => {
      if !changesets.validate() {
        changesets.initialize().await?;
      } else {
        println!("{}", *INIT_EXISTS_PROMPT);
      }
    }
    command => {
      if !changesets.validate() {
        println!("{}", *INIT_REQ_PROMPT);
      }

      match command {
        Command::Add(mut add) => add.execute(&changesets, &context).await?,
        Command::Version(mut version) => version.execute(&changesets, &context).await?,
        command => {
          println!("{:?}", command);
        }
      }
    }
  }

  Ok(())
}
