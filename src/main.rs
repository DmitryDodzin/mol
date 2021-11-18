use clap::Clap;
use dialoguer::{console, theme::ColorfulTheme};
use lazy_static::lazy_static;

use lightbringer_cargo::read_package;
use lightbringer_core::prelude::*;

mod cli;
mod command;

lazy_static! {
  pub(crate) static ref COLOR_THEME: ColorfulTheme = ColorfulTheme {
    unchecked_item_prefix: console::style("✘".to_owned()).for_stderr().red(),
    ..Default::default()
  };
}

use cli::Command;
use command::Context;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  let changesets = Changesets::default();

  let packages = read_package("Cargo.toml").await?;

  let context = Context { packages };

  if !changesets.validate() {
    println!("Changesets folder validation failed run 'init'");

    return Ok(());
  }

  match opts.cmd {
    Command::Add(mut add) => add.run(&changesets, &context).await?,
    _ => {
      println!("{:?}", opts.cmd);
    }
  }

  Ok(())
}
