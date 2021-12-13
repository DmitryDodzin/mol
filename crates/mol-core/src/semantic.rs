use std::str::FromStr;

use crate::error::{VersionBumpError, VersionParseError};
use crate::version::Versioned;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum SemanticVersion {
  Patch,
  Minor,
  Major,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

impl Versioned for Semantic {
  // TODO: add mask validation
  fn mask<'a>(mask: &str, version: &'a str) -> &'a str {
    &version[..mask.len()]
  }

  fn r#match(mask: &str, version: &str) -> bool {
    Self::mask(mask, version) == mask
  }

  fn options() -> Vec<Self> {
    vec![Self::patch(), Self::minor(), Self::major()]
  }

  fn apply(&self, current: &str) -> Result<String, VersionBumpError> {
    let mut current = current.split('.');

    let major = current
      .next()
      .map(|val| val.parse::<i32>().ok())
      .flatten()
      .ok_or(VersionBumpError)?;
    let minor = current
      .next()
      .map(|val| val.parse::<i32>().ok())
      .flatten()
      .ok_or(VersionBumpError)?;
    // TODO: Allow dev builds and not
    let patch = current
      .next()
      .map(|val| val.parse::<i32>().ok())
      .flatten()
      .ok_or(VersionBumpError)?;

    Ok(match self.r#type {
      SemanticVersion::Major => format!("{}.{}.{}", major + 1, 0, 0),
      SemanticVersion::Minor => format!("{}.{}.{}", major, minor + 1, 0),
      SemanticVersion::Patch => format!("{}.{}.{}", major, minor, patch + 1),
    })
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
  use crate::version::{VersionMod, Versioned};

  #[test]
  fn from_str() {
    let strings = vec!["patch", "minor", "minor", "major"];

    let versions: Vec<VersionMod<Semantic>> = strings
      .iter()
      .filter_map(|item| VersionMod::from_str(item).ok())
      .collect();

    assert_eq!(
      versions,
      vec![
        VersionMod::new(Semantic::patch()),
        VersionMod::new(Semantic::minor()),
        VersionMod::new(Semantic::minor()),
        VersionMod::new(Semantic::major()),
      ]
    );
  }

  #[test]
  fn to_str() {
    let versions = vec![
      VersionMod::new(Semantic::patch()),
      VersionMod::new(Semantic::minor()),
      VersionMod::new(Semantic::minor()),
      VersionMod::new(Semantic::major()),
    ];

    let strings: Vec<String> = versions.iter().map(|item| item.to_string()).collect();

    assert_eq!(strings, vec!["patch", "minor", "minor", "major"]);
  }

  // TODO: this
  // #[test]
  // fn canary_apply() {
  //   let version = VersionMod::new(Semantic::major());

  //   let bumped = version.apply("0.0.1-alpha.0");

  //   assert!(bumped.is_ok());

  //   assert_eq!(bumped.unwrap(), "0.0.2".to_owned())
  // }

  #[test]
  fn major_apply() {
    let version = VersionMod::new(Semantic::major());

    let bumped = version.apply("0.4.1");

    assert!(bumped.is_ok());

    assert_eq!(bumped.unwrap(), "1.0.0".to_owned())
  }

  #[test]
  fn minor_apply() {
    let version = VersionMod::new(Semantic::minor());

    let bumped = version.apply("4.1.1");

    assert!(bumped.is_ok());

    assert_eq!(bumped.unwrap(), "4.2.0".to_owned())
  }

  #[test]
  fn patch_apply() {
    let version = VersionMod::new(Semantic::patch());

    let bumped = version.apply("0.4.1");

    assert!(bumped.is_ok());

    assert_eq!(bumped.unwrap(), "0.4.2".to_owned())
  }
}
