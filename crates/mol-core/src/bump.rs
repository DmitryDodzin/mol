use std::collections::{HashMap, HashSet};

use crate::changeset::Changeset;

use crate::version::{Version, Versioned};

#[derive(Debug, Default)]
pub struct Bump<T> {
  changesets: Vec<Changeset<T>>,
  package_update: HashMap<String, Version<T>>,
  package_changesets: HashMap<String, HashSet<usize>>,
}

impl<'a, T: Versioned> Bump<T> {
  pub fn add(&mut self, changeset: Changeset<T>) {
    let index = self.changesets.len();
    self.changesets.insert(index, changeset);
    let changeset = &self.changesets[index];

    for (name, version) in &changeset.packages {
      if let Some(changesets) = self.package_changesets.get_mut(name) {
        changesets.insert(index);
      } else {
        self
          .package_changesets
          .insert(name.clone(), vec![index].into_iter().collect());
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
  use crate::prelude::*;

  #[test]
  fn add() {
    let mut bump = Bump::default();

    bump.add(Changeset {
      packages: vec![("mol".to_owned(), Version::new(Semantic::minor()))]
        .into_iter()
        .collect(),
      message: "Hi".to_owned(),
    });
    bump.add(Changeset {
      packages: vec![
        ("mol".to_owned(), Version::new(Semantic::patch())),
        ("mol-core".to_owned(), Version::new(Semantic::major())),
      ]
      .into_iter()
      .collect(),
      message: "Too bad we dont play games".to_owned(),
    });

    assert_eq!(bump.changesets.len(), 2);
    assert_eq!(
      bump.package_changesets.get("mol-core"),
      Some(&vec![1].into_iter().collect())
    );
    assert_eq!(
      bump.package_changesets.get("mol"),
      Some(&vec![0, 1].into_iter().collect())
    );
    assert_eq!(
      bump.package_update.get("mol"),
      Some(&Version::new(Semantic::minor()))
    );
    assert_eq!(
      bump.package_update.get("mol-core"),
      Some(&Version::new(Semantic::major()))
    );
  }

  #[test]
  fn changesets() {
    let mut bump = Bump::default();

    // let packages = vec![
    //   Package {
    //     path: "".into(),
    //     name: "mol-c".to_owned(),
    //     version: "0.1.0".to_owned(),
    //     dependencies: vec![("mol-core".to_owned(), "0.1.0".to_owned())],
    //   },
    //   Package {
    //     path: "".into(),
    //     name: "mol".to_owned(),
    //     version: "0.1.0".to_owned(),
    //     dependencies: vec![("mol-core".to_owned(), "0.1.0".to_owned())],
    //   },
    // ];
    // let graph = packages.as_package_graph();

    bump.add(
      Changeset {
        packages: vec![("mol".to_owned(), Version::new(Semantic::minor()))]
          .into_iter()
          .collect(),
        message: "Hi".to_owned(),
      },
      // &graph,
    );
    bump.add(
      Changeset {
        packages: vec![("mol-core".to_owned(), Version::new(Semantic::major()))]
          .into_iter()
          .collect(),
        message: "Too bad we dont play games".to_owned(),
      },
      // &graph,
    );

    let changesets = bump.package("mol").changesets();

    assert!(changesets.is_some());

    let changesets = changesets.unwrap();

    assert_eq!(changesets.len(), 1);
    assert_eq!(changesets[0].message, "Hi");

    let changesets = bump.package("mol-core").changesets();

    assert!(changesets.is_some());

    let changesets = changesets.unwrap();

    assert_eq!(changesets.len(), 1);
    assert_eq!(changesets[0].message, "Too bad we dont play games");
  }
}
