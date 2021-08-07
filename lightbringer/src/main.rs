use clap::Clap;
use lightbringer_cargo::CargoExplorer;
use lightbringer_core::Lightbringer;

mod cli;
mod command;

use command::Command;

fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  let context = Lightbringer::from(CargoExplorer);

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
