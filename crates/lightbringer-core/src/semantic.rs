use std::str::FromStr;

use crate::error::VersionParseError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum SemanticVersion {
  Patch,
  Minor,
  Major,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Semantic {
  r#type: SemanticVersion,
}

impl Semantic {
  pub fn minor() -> Self {
    Semantic {
      r#type: SemanticVersion::Minor,
    }
  }
  pub fn major() -> Self {
    Semantic {
      r#type: SemanticVersion::Major,
    }
  }
  pub fn patch() -> Self {
    Semantic {
      r#type: SemanticVersion::Patch,
    }
  }
}

impl Default for Semantic {
  fn default() -> Self {
    Semantic::patch()
  }
}

impl From<SemanticVersion> for Semantic {
  fn from(r#type: SemanticVersion) -> Self {
    Semantic { r#type }
  }
}

impl FromStr for Semantic {
  type Err = VersionParseError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value.to_lowercase().as_str() {
      "patch" => Ok(Semantic::patch()),
      "minor" => Ok(Semantic::minor()),
      "major" => Ok(Semantic::major()),
      _ => Err(VersionParseError::from(value.to_string())),
    }
  }
}

impl ToString for Semantic {
  fn to_string(&self) -> String {
    match self.r#type {
      SemanticVersion::Patch => "patch",
      SemanticVersion::Minor => "minor",
      SemanticVersion::Major => "major",
    }
    .to_owned()
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::version::Version;

  #[test]
  fn from_str() {
    let strings = vec!["patch", "minor", "minor", "major"];

    let versions: Vec<Version<Semantic>> = strings
      .iter()
      .filter_map(|item| Version::from_str(item).ok())
      .collect();

    assert_eq!(
      versions,
      vec![
        Version::new(Semantic::patch()),
        Version::new(Semantic::minor()),
        Version::new(Semantic::minor()),
        Version::new(Semantic::major()),
      ]
    );
  }

  #[test]
  fn to_str() {
    let versions: Vec<Version<Semantic>> = vec![
      Version::new(Semantic::patch()),
      Version::new(Semantic::minor()),
      Version::new(Semantic::minor()),
      Version::new(Semantic::major()),
    ];

    let strings: Vec<String> = versions.iter().map(|item| item.to_string()).collect();

    assert_eq!(strings, vec!["patch", "minor", "minor", "major"]);
  }
}
