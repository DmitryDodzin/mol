use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Version {
  Patch,
  Minor,
  Major,
}

pub struct Changeset {
  pub packages: HashMap<String, Version>,
  pub message: String,
}

impl Changeset {
  pub fn parse(value: &str) -> Result<Self, <Self as FromStr>::Err> {
    Changeset::from_str(value)
  }
}

impl FromStr for Changeset {
  type Err = String;
  fn from_str(_: &str) -> Result<Self, Self::Err> {
    todo!()
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
}
