use clap::Clap;
use lightbringer_core::Lightbringer;

#[cfg(feature = "cargo")]
use lightbringer_cargo::CargoExplorer;

mod cli;
mod command;

use command::Command;

fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  #[cfg(feature = "cargo")]
  let explorer = CargoExplorer;

  #[cfg(not(feature = "cargo"))]
  let explorer = lightbringer_core::explorer::EmptyExplorer;

  let context = Lightbringer::from(explorer);

  match opts.cmd {
    cli::Command::Add(add_command) => {
      add_command.run(&context)?;
    }
    _ => {
      println!("{:?}", opts);
    }
  }

  Ok(())
}
