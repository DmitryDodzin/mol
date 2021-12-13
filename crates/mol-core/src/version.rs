use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::changelog::AsChangelogFmt;
use crate::error::VersionBumpError;

pub trait Versioned: AsChangelogFmt + Clone + Default + Hash + FromStr + Ord + ToString {
  fn options() -> Vec<Self>;

  fn mask<'a>(mask: &str, version: &'a str) -> &'a str;

  fn r#match(mask: &str, version: &str) -> bool;

  fn apply(&self, current: &str) -> Result<String, VersionBumpError>;
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct VersionMod<T> {
  pub(crate) version: T,
}

impl<T> VersionMod<T> {
  pub fn new(version: T) -> Self {
    VersionMod { version }
  }
}

impl<T> Versioned for VersionMod<T>
where
  T: Versioned,
{
  fn mask<'a>(mask: &str, version: &'a str) -> &'a str {
    T::mask(mask, version)
  }
  fn r#match(mask: &str, version: &str) -> bool {
    T::r#match(mask, version)
  }
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

impl<T> FromStr for VersionMod<T>
where
  T: FromStr,
{
  type Err = T::Err;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    Ok(VersionMod {
      version: T::from_str(value)?,
    })
  }
}

impl<T> ToString for VersionMod<T>
where
  T: ToString,
{
  fn to_string(&self) -> String {
    self.version.to_string()
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version<T> {
  pub value: String,
  r#type: PhantomData<T>,
}

impl<T, U> From<U> for Version<T>
where
  U: ToString,
{
  fn from(value: U) -> Self {
    Version {
      value: value.to_string(),
      r#type: PhantomData::<T>,
    }
  }
}
