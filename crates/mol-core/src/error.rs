use thiserror::Error;

use crate::plugin::{CORE_VERSION, RUSTC_VERSION};

#[derive(Debug, Error)]
pub enum ChangesetParseError {
  #[error("Header not found")]
  HeaderNotFound,
  #[error("Header parsing error")]
  HeaderParsing,
}

#[derive(Debug, Error)]
#[error("Explorer error")]
pub struct ExplorerError;

impl From<std::io::Error> for ExplorerError {
  fn from(_: std::io::Error) -> Self {
    ExplorerError
  }
}

#[derive(Debug, Error)]
#[error("\"{0}\" isn't a version, should be patch/minor/major")]
pub struct VersionParseError(pub(crate) String);

#[derive(Debug, Error)]
#[error("Version bump error")]
pub struct VersionBumpError;

fn pretty_print_version_incompatbility(rustc_ver: &str, core_ver: &str) -> String {
  let mut message = vec![];

  if rustc_ver != RUSTC_VERSION {
    message.push(format!(
      "rustc -> wanted {} but got {}",
      RUSTC_VERSION, rustc_ver
    ));
  }

  if core_ver != CORE_VERSION {
    message.push(format!(
      "mol-core -> wanted {} but got {}",
      CORE_VERSION, core_ver
    ));
  }

  message.join("\n")
}

#[derive(Debug, Error)]
pub enum PluginLoadError<'a> {
  #[error("Incompatible versions detected \n{}", pretty_print_version_incompatbility(.0, .1))]
  IncompatibleVersion(&'a str, &'a str),
}
