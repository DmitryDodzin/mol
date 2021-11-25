use std::hash::Hash;
use std::str::FromStr;

use crate::changelog::AsChangelogFmt;
use crate::error::VersionBumpError;

pub trait Versioned: AsChangelogFmt + Clone + Default + Hash + FromStr + Ord + ToString {
  fn options() -> Vec<Self>;

  fn apply(&self, current: &str) -> Result<String, VersionBumpError>;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Version<T> {
  version: T,
}

impl<T> Version<T> {
  pub fn new(version: T) -> Self {
    Version { version }
  }
}

impl<T: AsChangelogFmt> AsChangelogFmt for Version<T> {
  fn as_changelog_fmt(&self) -> String {
    self.version.as_changelog_fmt()
  }
}

impl<T> Default for Version<T>
where
  T: Versioned,
{
  fn default() -> Self {
    Version {
      version: Default::default(),
    }
  }
}

impl<T> Versioned for Version<T>
where
  T: Versioned,
{
  fn options() -> Vec<Self> {
    T::options()
      .into_iter()
      .map(|version| Self { version })
      .collect()
  }
  fn apply(&self, current: &str) -> Result<String, VersionBumpError> {
    self.version.apply(current)
  }
}

impl<T> FromStr for Version<T>
where
  T: FromStr,
{
  type Err = T::Err;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Ok(Version {
      version: T::from_str(value)?,
    })
  }
}

impl<T> ToString for Version<T>
where
  T: ToString,
{
  fn to_string(&self) -> String {
    self.version.to_string()
  }
}
