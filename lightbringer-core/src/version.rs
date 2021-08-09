use crate::error::VersionParseError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Version {
  Patch,
  Minor,
  Major,
}

impl FromStr for Version {
  type Err = VersionParseError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value.to_lowercase().as_str() {
      "patch" => Ok(Version::Patch),
      "minor" => Ok(Version::Minor),
      "major" => Ok(Version::Major),
      _ => Err(VersionParseError::from(value.to_string())),
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

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn from_str() {
    let strings = vec!["patch", "minor", "minor", "major"];

    let versions: Vec<Version> = strings
      .iter()
      .filter_map(|item| Version::from_str(item).ok())
      .collect();

    assert_eq!(
      versions,
      vec![
        Version::Patch,
        Version::Minor,
        Version::Minor,
        Version::Major
      ]
    );
  }

  #[test]
  fn to_str() {
    let versions = vec![
      Version::Patch,
      Version::Minor,
      Version::Minor,
      Version::Major,
    ];

    let strings: Vec<String> = versions.iter().map(|item| item.to_string()).collect();

    assert_eq!(strings, vec!["patch", "minor", "minor", "major"]);
  }
}
