use std::fmt::Debug;

use clap::Parser;
use dialoguer::{console, theme::ColorfulTheme};
use lazy_static::lazy_static;

use mol_core::prelude::*;

mod cli;
mod command;

use crate::{
  cli::Opts,
  command::{ExecutableContext, IntoExecutableCommand},
};

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

pub async fn handle_command<U: PackageManager, T: IntoExecutableCommand<U> + Debug>(
  changesets: &Changesets,
  context: &ExecutableContext<U>,
  command: T,
) -> anyhow::Result<()> {
  if !changesets.validate() {
    println!("{}", *INIT_REQ_PROMPT);
  }

  if let Some(exeutable) = command.as_executable() {
    exeutable.execute(changesets, context).await?;
  } else {
    println!("{:?}", command);
  }

  Ok(())
}

pub async fn exec<T: Default + PackageManager + Send + Sync>() -> anyhow::Result<()> {
  let args: Vec<String> = std::env::args().collect();
  let opts = if args.len() > 1 && args[1] == "mol" {
    Opts::parse_from(args[..1].iter().chain(&args[2..]))
  } else {
    Opts::parse_from(args)
  };

  let changesets = Changesets::default();

  let package_manager = T::default();

  let context = ExecutableContext {
    dry_run: opts.dry_run,
    packages: package_manager.read_package("Cargo.toml").await?,
    package_manager,
  };

  handle_command(&changesets, &context, opts.cmd).await?;

  Ok(())
}
