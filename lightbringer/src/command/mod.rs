use clap::Clap;
use lightbringer_core::Lightbringer;

pub trait Command {
  fn run(&self, context: &Lightbringer) -> Result<(), failure::Error>;
}

mod add;

pub use add::Add;

#[derive(Clap, Debug)]
pub struct Init;

#[derive(Clap, Debug)]
pub struct Version;

#[derive(Clap, Debug)]
pub struct Publish;

#[derive(Clap, Debug)]
pub struct Status;

#[derive(Clap, Debug)]
pub struct Pre;
