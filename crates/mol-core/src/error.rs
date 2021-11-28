use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ChangesetParseError {
  HeaderNotFound,
  HeaderParsing,
}

impl Display for ChangesetParseError {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
      ChangesetParseError::HeaderNotFound => write!(fmt, "Header Not Found"),
      ChangesetParseError::HeaderParsing => write!(fmt, "Header Parsing Error"),
    }
  }
}

impl Error for ChangesetParseError {}

#[derive(Debug)]
pub struct ChangelogParseError(String);

impl From<String> for ChangelogParseError {
  // TODO: add code here
  fn from(value: String) -> Self {
    ChangelogParseError(value)
  }
}

impl Display for ChangelogParseError {
  // TODO: add code here
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(fmt, "Could not parse changelog: {}", self.0)
  }
}

impl Error for ChangelogParseError {}

#[derive(Debug)]
pub struct ExplorerError;

impl From<std::io::Error> for ExplorerError {
  fn from(_: std::io::Error) -> Self {
    ExplorerError
  }
}

impl Display for ExplorerError {
  // TODO: add code here
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(fmt, "ExplorerError")
  }
}

impl Error for ExplorerError {}

#[derive(Debug)]
pub struct VersionParseError(String);

impl From<String> for VersionParseError {
  // TODO: add code here
  fn from(value: String) -> Self {
    VersionParseError(value)
  }
}

impl Display for VersionParseError {
  // TODO: add code here
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      fmt,
      "\"{}\" isn't a version, should be patch/minor/major",
      self.0
    )
  }
}

impl Error for VersionParseError {}

#[derive(Debug)]
pub struct VersionBumpError;

impl Display for VersionBumpError {
  // TODO: add code here
  fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(fmt, "VersionBumpError")
  }
}

impl Error for VersionBumpError {}
