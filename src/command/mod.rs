use std::path::PathBuf;

use clap::Clap;

mod add;

#[derive(Debug)]
pub struct Context {
  pub packages: Vec<(PathBuf, String, String)>,
}

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
