use std::collections::HashMap;

use crate::changeset::Changeset;
use crate::version::{Version, Versioned};

#[derive(Debug, Default)]
pub struct Bump<T> {
  changesets: Vec<Changeset<T>>,
  package_update: HashMap<String, Version<T>>,
  package_changesets: HashMap<String, Vec<usize>>,
}

impl<'a, T: Versioned> Bump<T> {
  pub fn add(&mut self, changeset: Changeset<T>) {
    let index = self.changesets.len();
    self.changesets.insert(index, changeset);

    for (name, version) in &self.changesets[index].packages {
      if let Some(changesets) = self.package_changesets.get_mut(name) {
        changesets.push(index);
      } else {
        self.package_changesets.insert(name.clone(), vec![index]);
      }

      if let Some(concat_version) = self.package_update.get_mut(name) {
        if version > concat_version {
          *concat_version = version.clone();
        }
      } else {
        self.package_update.insert(name.to_owned(), version.clone());
      }
    }
  }

  pub fn is_empty(&self) -> bool {
    self.changesets.is_empty()
  }

  pub fn package(&'a self, name: &'a str) -> PackageBump<'a, T> {
    PackageBump { name, bump: self }
  }
}

impl<T, K> From<K> for Bump<T>
where
  T: Default + Versioned,
  K: Iterator<Item = Changeset<T>>,
{
  fn from(items: K) -> Self {
    let mut bump = Self::default();

    for item in items {
      bump.add(item);
    }

    bump
  }
}

pub struct PackageBump<'a, T> {
  name: &'a str,
  bump: &'a Bump<T>,
}

impl<'a, T> PackageBump<'a, T> {
  pub fn changesets(&self) -> Option<Vec<&'a Changeset<T>>> {
    self.bump.package_changesets.get(self.name).map(|indexes| {
      indexes
        .iter()
        .map(|index| &self.bump.changesets[*index])
        .collect()
    })
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn version(&self) -> Option<&Version<T>> {
    self.bump.package_update.get(self.name)
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::semantic::Semantic;

  #[test]
  fn add() {
    let mut bump = Bump::default();

    bump.add(Changeset {
      packages: vec![("lightbringer".to_owned(), Version::new(Semantic::minor()))]
        .into_iter()
        .collect(),
      message: "Hi".to_owned(),
    });
    bump.add(Changeset {
      packages: vec![
        ("lightbringer".to_owned(), Version::new(Semantic::patch())),
        (
          "lightbringer-core".to_owned(),
          Version::new(Semantic::major()),
        ),
      ]
      .into_iter()
      .collect(),
      message: "Too bad we dont play games".to_owned(),
    });

    assert_eq!(bump.changesets.len(), 2);
    assert_eq!(
      bump.package_changesets.get("lightbringer-core"),
      Some(&vec![1])
    );
    assert_eq!(
      bump.package_changesets.get("lightbringer"),
      Some(&vec![0, 1])
    );
    assert_eq!(
      bump.package_update.get("lightbringer"),
      Some(&Version::new(Semantic::minor()))
    );
    assert_eq!(
      bump.package_update.get("lightbringer-core"),
      Some(&Version::new(Semantic::major()))
    );
  }

  #[test]
  fn changesets() {
    let mut bump = Bump::default();

    bump.add(Changeset {
      packages: vec![("lightbringer".to_owned(), Version::new(Semantic::minor()))]
        .into_iter()
        .collect(),
      message: "Hi".to_owned(),
    });
    bump.add(Changeset {
      packages: vec![
        ("lightbringer".to_owned(), Version::new(Semantic::major())),
        (
          "lightbringer-core".to_owned(),
          Version::new(Semantic::major()),
        ),
      ]
      .into_iter()
      .collect(),
      message: "Too bad we dont play games".to_owned(),
    });

    let changesets = bump.package("lightbringer").changesets();

    assert!(changesets.is_some());

    let changesets = changesets.unwrap();

    assert_eq!(changesets.len(), 2);
    assert_eq!(changesets[0].message, "Hi");

    let changesets = bump.package("lightbringer-core").changesets();

    assert!(changesets.is_some());

    let changesets = changesets.unwrap();

    assert_eq!(changesets.len(), 1);
    assert_eq!(changesets[0].message, "Too bad we dont play games");
  }
}
