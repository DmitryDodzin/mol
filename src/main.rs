use clap::Clap;

mod cli;
mod command;

fn main() -> Result<(), failure::Error> {
  let opts: cli::Opts = cli::Opts::parse();

  println!("{:?}", opts.cmd);

  Ok(())
}
