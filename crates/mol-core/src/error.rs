use std::marker::PhantomData;

use itertools::Itertools;
use thiserror::Error;

use crate::plugin::{CORE_VERSION, RUSTC_VERSION};
use crate::version::VersionEditor;

#[derive(Debug, Error)]
pub enum ChangesetParseError {
  #[error("Header not found")]
  HeaderNotFound,
  #[error("Header parsing error")]
  HeaderParsing,
}

#[derive(Debug, Error)]
#[error("\"{0}\" isn't a version, should be {}", V::options().iter().map(|val| val.to_string()).join("/"))]
pub struct VersionParseError<V: VersionEditor>(String, PhantomData<V>);

impl<V: VersionEditor> From<&str> for VersionParseError<V> {
  fn from(value: &str) -> Self {
    VersionParseError(value.to_owned(), PhantomData::<V>)
  }
}

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
pub enum PluginLoadError {
  #[error("Incompatible versions detected \n{}", pretty_print_version_incompatbility(.0, .1))]
  IncompatibleVersion(String, String),
  #[error(transparent)]
  LibloadingError(#[from] libloading::Error),
  #[error(transparent)]
  GeneralError(#[from] anyhow::Error),
}
