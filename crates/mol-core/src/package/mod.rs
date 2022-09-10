use std::path::PathBuf;

use crate::version::{Version, Versioned};

pub mod command;
pub mod graph;
pub mod loader;
pub mod manager;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Package<T: Versioned> {
  pub path: PathBuf,
  pub name: String,
  pub version: Version<T>,
  pub dependencies: Vec<(String, String)>,
}
