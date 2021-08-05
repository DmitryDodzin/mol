use clap::Clap;

mod cli;
mod command;

use command::Command;

fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  match opts.cmd {
    cli::Command::Add(add_command) => {
      add_command.run()?;
    }
    _ => {
      println!("{:?}", opts);
    }
  }

  Ok(())
}
