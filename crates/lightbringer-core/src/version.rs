use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version<T> {
  pub version: T,
}

impl<T> Version<T> {
  pub fn new<Param: Into<T>>(version: Param) -> Self {
    Version {
      version: version.into(),
    }
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
