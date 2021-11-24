use std::collections::HashMap;
use std::path::PathBuf;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Package {
  pub path: PathBuf,
  pub name: String,
  pub version: String,
  pub dependencies: Vec<(String, String)>,
}

pub trait AsPackageGraph {
  fn as_package_graph(&self) -> PackageGraph<'_>;
}

#[derive(Debug, PartialEq)]
pub struct PackageGraph<'a> {
  edges: Vec<(&'a str, &'a Package)>,
  nodes: Vec<&'a Package>,
}

impl<'a> PackageGraph<'a> {
  pub fn child_changes(&self, name: &'a str) -> Vec<&'a Package> {
    self
      .edges
      .iter()
      .filter(|(p_name, _)| &name == p_name)
      .map(|(_, package)| *package)
      .collect()
  }

  pub fn update_order(&self) -> Vec<&'a Package> {
    let name_map: HashMap<&str, &'a Package> = self
      .nodes
      .iter()
      .map(|package| (package.name.as_str(), *package))
      .collect();

    let base = self
      .nodes
      .iter()
      .map(|package| (package.name.as_str(), 0))
      .collect();

    let scores: HashMap<&'a str, isize> =
      self.edges.iter().fold(base, |mut acc, (parent, package)| {
        let value = acc
          .get(&*package.name)
          .map(|val| val + 1)
          .unwrap_or_else(|| 0);

        if let Some(score) = acc.get_mut(parent) {
          *score += value
        } else {
          acc.insert(parent, value);
        }

        acc
      });

    scores
      .into_iter()
      .sorted_by_key(|(_, score)| -*score)
      .map(|(name, _)| name_map[name])
      .collect()
  }
}

impl AsPackageGraph for Vec<Package> {
  fn as_package_graph(&self) -> PackageGraph<'_> {
    let nodes: Vec<&Package> = self.iter().collect();
    let edges: Vec<(&str, &Package)> = self.iter().fold(vec![], |mut acc, package| {
      acc.extend(
        package
          .dependencies
          .iter()
          .map(|(dep, _)| (dep.as_str(), package)),
      );
      acc
    });

    PackageGraph { edges, nodes }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn as_package_graph() {
    let packages = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    assert_eq!(
      graph,
      PackageGraph {
        nodes: vec![&packages[0], &packages[1], &packages[2]],
        edges: vec![("foo", &packages[1]), ("foo", &packages[2])]
      }
    );
  }

  #[test]
  fn update_order() {
    let packages = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    let update_order = graph.update_order();

    assert_eq!(update_order[0], &packages[0]);
  }

  #[test]
  fn child_changes() {
    let packages = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".to_owned(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    let child_changes = graph.child_changes("foo");

    assert_eq!(
      child_changes,
      packages[1..].iter().collect::<Vec<&Package>>()
    );
  }
}
