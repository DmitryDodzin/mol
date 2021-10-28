use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use itertools::Itertools;

use crate::error::ChangesetParseError;
use crate::version::Version;

#[derive(Debug, Default)]
pub struct Changeset {
  pub packages: HashMap<String, Version>,
  pub message: String,
}

impl Changeset {
  pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
    Changeset::from_str(value)
  }

  pub fn save(self, output: PathBuf) -> std::io::Result<()> {
    let mut file = File::create(output)?;

    file.write_all(self.to_string().as_bytes())?;

    Ok(())
  }

  fn find_changeset_start(
    lines: &mut dyn Iterator<Item = &str>,
  ) -> Result<(), ChangesetParseError> {
    for line in lines {
      match line {
        "" => {}
        "---" => return Ok(()),
        _ => return Err(ChangesetParseError::HeaderNotFound),
      }
    }

    Err(ChangesetParseError::HeaderNotFound)
  }

  fn parse_package_name(value: &str) -> &str {
    if value.starts_with('\"') {
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
    let mut lines = value.split('\n');

    Changeset::find_changeset_start(&mut lines)?;

    for line in &mut lines {
      match line {
        "---" => break,
        value => {
          let change_value: Vec<&str> = value.split(':').map(|val| val.trim()).collect();

          match change_value.len() {
            2 => {
              let (package, version) = (
                Changeset::parse_package_name(change_value[0]),
                Version::from_str(change_value[1]),
              );

              if version.is_err() {
                return Err(ChangesetParseError::HeaderParsing);
              }

              packages.insert(package.to_string(), version.unwrap());
            }
            _ => return Err(ChangesetParseError::HeaderParsing),
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

impl ToString for Changeset {
  fn to_string(&self) -> String {
    let mut output = vec![];

    output.extend(b"---\n");
    for (package, version) in self.packages.iter().sorted() {
      output.extend(format!("\"{}\": {}\n", package, version.to_string()).as_bytes())
    }
    output.extend(b"---\n");
    output.extend(self.message.as_bytes());

    String::from_utf8(output).unwrap()
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

  #[test]
  fn to_str() {
    let changeset = Changeset {
      packages: vec![("lightbinger".to_owned(), Version::Minor)]
        .into_iter()
        .collect(),
      message: "Do cool stuff".to_string(),
    };

    assert_eq!(
      changeset.to_string(),
      "---
\"lightbinger\": minor
---
Do cool stuff"
    )
  }

  #[test]
  fn to_str_multiple() {
    let changeset = Changeset {
      packages: vec![
        ("lightbinger".to_owned(), Version::Minor),
        ("lightbinger-core".to_owned(), Version::Major),
      ]
      .into_iter()
      .collect(),
      message: "Do cool stuff".to_string(),
    };

    assert_eq!(
      changeset.to_string(),
      "---
\"lightbinger\": minor
\"lightbinger-core\": major
---
Do cool stuff"
    )
  }
}
