use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Package {
  pub path: PathBuf,
  pub name: String,
  pub version: String,
  pub dependencies: Vec<(String, String)>,
}
