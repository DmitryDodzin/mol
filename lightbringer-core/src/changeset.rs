use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseVersionError(String);

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub enum ChangesetParseError {
  NoPackageVersionsFound,
  InvalidPackageVerionSyntax,
}

#[derive(Debug)]
pub struct Changeset {
  pub packages: HashMap<String, Version>,
  pub message: String,
}

impl Changeset {
  pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
    Changeset::from_str(value)
  }

  fn find_changeset_start(
    lines: &mut dyn Iterator<Item = &str>,
  ) -> Result<(), ChangesetParseError> {
    for line in lines {
      match line {
        "" => {}
        "---" => return Ok(()),
        _ => return Err(ChangesetParseError::NoPackageVersionsFound),
      }
    }

    Err(ChangesetParseError::NoPackageVersionsFound)
  }

  fn parse_package_name(value: &str) -> &str {
    if value.starts_with("\"") {
      let mut chars = value.chars();
      chars.next();
      chars.next_back();
      chars.as_str()
    } else {
      value
    }
  }
}

impl FromStr for Changeset {
  type Err = ChangesetParseError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let mut packages = HashMap::new();
    let mut lines = value.split("\n");

    Changeset::find_changeset_start(&mut lines)?;

    for line in &mut lines {
      match line {
        "---" => break,
        value => {
          let change_value: Vec<&str> = value.split(":").map(|val| val.trim()).collect();

          match change_value.len() {
            2 => {
              let (package, version) = (
                Changeset::parse_package_name(change_value[0]),
                Version::from_str(change_value[1]),
              );

              if version.is_err() {
                return Err(ChangesetParseError::InvalidPackageVerionSyntax);
              }

              packages.insert(package.to_string(), version.unwrap());
            }
            _ => return Err(ChangesetParseError::InvalidPackageVerionSyntax),
          }
        }
      }
    }

    Ok(Changeset {
      packages,
      message: lines.collect::<Vec<&str>>().join("\n"),
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn from_str() {
    let changeset = Changeset::from_str(
      "
---
\"lightbinger\": minor
---

Do cool stuff
      ",
    );

    assert!(changeset.is_ok());

    let changeset = changeset.unwrap();

    assert_eq!(
      changeset.packages,
      vec![("lightbinger".to_string(), Version::Minor)]
        .into_iter()
        .collect()
    );
    assert_eq!(
      changeset.message,
      "
Do cool stuff
      "
    );
  }

  #[test]
  fn from_str_multiple() {
    let changeset = Changeset::from_str(
      "
---
\"lightbinger\": minor
\"lightbinger-core\": major
---

Do cool stuff
      ",
    )
    .unwrap();

    assert_eq!(
      changeset.packages,
      vec![
        ("lightbinger".to_string(), Version::Minor),
        ("lightbinger-core".to_string(), Version::Major)
      ]
      .into_iter()
      .collect()
    );
  }
}
