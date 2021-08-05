use std::str::FromStr;

#[derive(Debug)]
pub struct ParseVersionError(String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Version {
  Patch,
  Minor,
  Major,
}

impl FromStr for Version {
  type Err = ParseVersionError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value.to_lowercase().as_str() {
      "patch" => Ok(Version::Patch),
      "minor" => Ok(Version::Minor),
      "major" => Ok(Version::Major),
      _ => Err(ParseVersionError(value.to_string())),
    }
  }
}

impl ToString for Version {
  fn to_string(&self) -> String {
    match self {
      Version::Patch => "patch".to_owned(),
      Version::Minor => "minor".to_owned(),
      Version::Major => "major".to_owned(),
    }
  }
}
