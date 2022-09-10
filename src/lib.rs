#![doc = include_str!("../README.md")]

use std::alloc::System;
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use dialoguer::{console, theme::ColorfulTheme};
use lazy_static::lazy_static;

use mol_core::prelude::*;

mod cli;
mod command;

#[global_allocator]
static ALLOCATOR: System = System;

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

async fn handle_command<
  T: PackageManager,
  V: VersionEditor + 'static,
  U: IntoExecutableCommand<T, V>,
>(
  context: &ExecutableContext<T, V>,
  plugin_manager: &PluginManager,
  command: U,
) -> anyhow::Result<()> {
  if let Some(exeutable) = command.as_executable() {
    exeutable.execute(context, plugin_manager).await?;
  }

  Ok(())
}

pub async fn exec<T, V>() -> anyhow::Result<()>
where
  T: PackageManager + Default + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
  T::Metadata: Send + Sync,
  <V as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
  let args: Vec<String> = std::env::args().collect();
  let opts = if args.len() > 1 && args[1] == "mol" {
    Opts::parse_from(args[..1].iter().chain(&args[2..]))
  } else {
    Opts::parse_from(args)
  };

  let context = ExecutableContext::<T, V>::new(DEFAULT_PACKAGE_DIR.clone(), opts.dry_run).await?;

  let mut plugin_manager = PluginManager::default();

  for plugin in &opts.plugins {
    unsafe {
      plugin_manager
        .load(&plugin, &context.as_plugin())
        .with_context(|| format!("Could not load plugin at path {}", plugin))?;
    }
  }

  match opts.cmd {
    Command::Init(_) => handle_command(&context, &plugin_manager, opts.cmd).await?,
    command => {
      if !context.changesets.validate() {
        println!("{}", *INIT_REQ_PROMPT);
      }

      handle_command(&context, &plugin_manager, command).await?
    }
  }

  Ok(())
}
