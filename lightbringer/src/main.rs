use clap::Clap;
use lightbringer_core::Lightbringer;

mod command;

use command::Opts;

fn main() {
  let opts: Opts = Opts::parse();

  println!("Hello, {:?} {:?}!", Lightbringer, opts);
}
