use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use faker_rand::lorem::Word;
use itertools::Itertools;
use rand::Rng;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::error::ChangesetParseError;
use crate::version::{VersionMod, Versioned};

#[derive(Debug, Default)]
pub struct Changeset<T> {
  pub packages: HashMap<String, VersionMod<T>>,
  pub message: String,
}

impl<T> Changeset<T> {
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

  pub fn random_file_name() -> String {
    let mut rng = rand::thread_rng();
    format!("{}-{}.md", rng.gen::<Word>(), rng.gen::<Word>())
  }
}

impl<T> Changeset<T>
where
  T: FromStr + Ord + Versioned,
{
  pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
    Changeset::from_str(value)
  }

  pub async fn save<P: AsRef<Path>>(self, output: P) -> std::io::Result<()> {
    let mut file = File::create(output).await?;

    file.write_all(self.to_string().as_bytes()).await?;

    Ok(())
  }
}

impl<T> FromStr for Changeset<T>
where
  T: FromStr,
{
  type Err = ChangesetParseError;
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let mut packages = HashMap::new();
    let mut lines = value.split('\n').map(|line| line.trim_end());

    Self::find_changeset_start(&mut lines)?;

    for line in &mut lines {
      match line {
        "---" => break,
        value => {
          let change_value: Vec<&str> = value.split(':').map(|val| val.trim()).collect();

          match change_value.len() {
            2 => {
              let (package, version) = (
                Self::parse_package_name(change_value[0]),
                VersionMod::from_str(change_value[1]),
              );

              if let Ok(version) = version {
                packages.insert(package.to_string(), version);
              } else {
                return Err(ChangesetParseError::HeaderParsing);
              }
            }
            _ => return Err(ChangesetParseError::HeaderParsing),
          }
        }
      }
    }

    Ok(Self {
      packages,
      message: lines.collect::<Vec<&str>>().join("\n").trim().to_owned(),
    })
  }
}

impl<T> ToString for Changeset<T>
where
  T: Versioned + Ord + ToString,
{
  fn to_string(&self) -> String {
    let mut output = vec![];

    output.extend(b"---\n");
    for (package, version) in self.packages.iter().sorted() {
      output.extend(format!("\"{}\": {}\n", package, version.to_string()).as_bytes())
    }
    output.extend(b"---\n\n");
    output.extend(self.message.as_bytes());
    output.push(b'\n');

    String::from_utf8(output).unwrap()
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::semantic::Semantic;

  #[test]
  fn from_str() {
    let changeset = Changeset::from_str(
      "
---
\"mol\": minor
---

Do cool stuff
",
    );

    assert!(changeset.is_ok());

    let changeset = changeset.unwrap();

    assert_eq!(
      changeset.packages,
      vec![("mol".to_string(), VersionMod::new(Semantic::minor()))]
        .into_iter()
        .collect()
    );
    assert_eq!(changeset.message, "Do cool stuff");
  }

  #[test]
  fn from_str_multiple() {
    let changeset = Changeset::from_str(
      "
---
\"mol\": minor
\"mol-core\": major
---

Do cool stuff
",
    )
    .unwrap();

    assert_eq!(
      changeset.packages,
      vec![
        ("mol".to_string(), VersionMod::new(Semantic::minor())),
        ("mol-core".to_string(), VersionMod::new(Semantic::major()))
      ]
      .into_iter()
      .collect()
    );
  }

  #[test]
  fn to_str() {
    let changeset = Changeset {
      packages: vec![("mol".to_owned(), VersionMod::new(Semantic::minor()))]
        .into_iter()
        .collect(),
      message: "Do cool stuff".to_string(),
    };

    assert_eq!(
      changeset.to_string(),
      "---
\"mol\": minor
---

Do cool stuff
"
    )
  }

  #[test]
  fn to_str_multiple() {
    let changeset = Changeset {
      packages: vec![
        ("mol".to_owned(), VersionMod::new(Semantic::minor())),
        ("mol-core".to_owned(), VersionMod::new(Semantic::major())),
      ]
      .into_iter()
      .collect(),
      message: "Do cool stuff".to_string(),
    };

    assert_eq!(
      changeset.to_string(),
      "---
\"mol\": minor
\"mol-core\": major
---

Do cool stuff
"
    )
  }
}
