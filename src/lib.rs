use std::fmt::Debug;
use std::str::FromStr;

use clap::Parser;
use dialoguer::{console, theme::ColorfulTheme};
use lazy_static::lazy_static;

use mol_core::prelude::*;

mod cli;
mod command;

use crate::{
  cli::{Command, Opts},
  command::{ExecutableContext, IntoExecutableCommand},
};

lazy_static! {
  pub(crate) static ref COLOR_THEME: ColorfulTheme = ColorfulTheme {
    unchecked_item_prefix: console::style("✘".to_owned()).for_stderr().red(),
    ..Default::default()
  };
  pub(crate) static ref ADD_NO_PACKAGES: console::StyledObject<&'static str> =
    console::style("Please select a package to create a chageset").yellow();
  static ref INIT_REQ_PROMPT: console::StyledObject<&'static str> =
    console::style("Changesets folder validation failed run 'init'").yellow();
  static ref INIT_EXISTS_PROMPT: console::StyledObject<&'static str> =
    console::style("Changesets folder already initialized").yellow();
}

async fn handle_command<U: PackageManager, V: Versioned, T: IntoExecutableCommand<U, V> + Debug>(
  context: &ExecutableContext<U, V>,
  command: T,
) -> anyhow::Result<()> {
  if let Some(exeutable) = command.as_executable() {
    exeutable.execute(context).await?;
  } else {
    println!("{:?}", command);
  }

  Ok(())
}

pub async fn exec<T: Default + PackageManager + Send + Sync, V: Versioned + Send + Sync + 'static>(
) -> anyhow::Result<()>
where
  <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
  let args: Vec<String> = std::env::args().collect();
  let opts = if args.len() > 1 && args[1] == "mol" {
    Opts::parse_from(args[..1].iter().chain(&args[2..]))
  } else {
    Opts::parse_from(args)
  };

  let package_manager = T::default();

  let mut context: ExecutableContext<T, V> = ExecutableContext {
    changesets: Changesets::default(),
    dry_run: opts.dry_run,
    packages: package_manager.read_package("Cargo.toml").await?,
    package_manager,
    plugin_manager: PluginManager::new(),
  };

  for plugin in &opts.plugins {
    unsafe {
      context.plugin_manager.load_plugin(&plugin)?;
    }
  }

  match opts.cmd {
    Command::Init(_) => handle_command(&context, opts.cmd).await?,
    command => {
      if !context.changesets.validate() {
        println!("{}", *INIT_REQ_PROMPT);
      }

      handle_command(&context, command).await?
    }
  }

  Ok(())
}
