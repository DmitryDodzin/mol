use clap::Clap;
use lightbringer_core::{
  explorer::{Explorer, ExplorerError},
  Lightbringer,
};

mod cli;
mod command;

use command::Command;

struct CrateExplorer;

impl Explorer for CrateExplorer {
  fn list_packages(&self) -> Result<Vec<String>, ExplorerError> {
    if let Ok(dirs) = self.list_dir() {
      println!("{:?}", dirs);
    }

    todo!()
  }
}

fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  let context = Lightbringer::from(CrateExplorer);

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
