use std::collections::HashMap;

use itertools::Itertools;

use crate::package::Package;
use crate::version::Versioned;

pub trait AsPackageGraph<T: Versioned> {
  fn as_package_graph(&self) -> PackageGraph<'_, T>;
}

#[derive(Debug, PartialEq)]
pub struct PackageGraph<'a, T: Versioned> {
  edges: Vec<(&'a str, &'a Package<T>)>,
  nodes: Vec<&'a Package<T>>,
}

impl<'a, T> PackageGraph<'a, T>
where
  T: Versioned,
{
  pub fn child_changes(&self, name: &'a str) -> Vec<&'a Package<T>> {
    self
      .edges
      .iter()
      .filter(|(p_name, _)| &name == p_name)
      .map(|(_, package)| *package)
      .collect()
  }

  fn stagger_scores(&self, scores: &mut HashMap<&'a str, isize>) {
    for (edge, target) in &self.edges {
      if let Some(value) = scores.get(&*target.name).copied() {
        if let Some(score) = scores.get_mut(&*edge) {
          *score += value;
        }
      }
    }
  }

  pub fn update_order(&self) -> Vec<&'a Package<T>> {
    let name_map: HashMap<&str, &'a Package<T>> = self
      .nodes
      .iter()
      .map(|package| (package.name.as_str(), *package))
      .collect();

    let base = self
      .nodes
      .iter()
      .map(|package| (package.name.as_str(), 0))
      .collect();

    let mut scores: HashMap<&'a str, isize> =
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

    for _ in 0..self.nodes.len() {
      self.stagger_scores(&mut scores);
    }

    scores
      .into_iter()
      .sorted_by_key(|(_, score)| -*score)
      .filter(|(name, _)| name_map.contains_key(name))
      .map(|(name, _)| name_map[name])
      .collect()
  }
}

impl<V> AsPackageGraph<V> for Vec<Package<V>>
where
  V: Versioned,
{
  fn as_package_graph(&self) -> PackageGraph<'_, V> {
    let mut nodes = Vec::with_capacity(self.len());
    let mut edges = Vec::new();

    for package in self.iter() {
      nodes.push(package);

      edges.extend(
        package
          .dependencies
          .iter()
          .map(|(dep, _)| (dep.as_str(), package)),
      );
    }

    PackageGraph { edges, nodes }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::semantic::Semantic;

  #[test]
  fn as_package_graph() {
    let packages: Vec<Package<Semantic>> = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".into(),
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
    let packages: Vec<Package<Semantic>> = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    let update_order = graph.update_order();

    assert_eq!(update_order[0], &packages[0]);
  }

  #[test]
  fn update_order_deep() {
    let packages: Vec<Package<Semantic>> = vec![
      Package {
        path: "".into(),
        name: "pre_foo".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("pre_foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    let update_order = graph.update_order();

    let packages_ref: Vec<&Package<Semantic>> = packages.iter().collect();
    assert_eq!(update_order[..2], packages_ref[..2]);
  }

  #[test]
  fn child_changes() {
    let packages = vec![
      Package {
        path: "".into(),
        name: "foo".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![],
      },
      Package {
        path: "".into(),
        name: "bar".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
      Package {
        path: "".into(),
        name: "baz".to_owned(),
        version: "1.0.0".into(),
        dependencies: vec![("foo".to_owned(), "1".to_owned())],
      },
    ];

    let graph = packages.as_package_graph();

    let child_changes = graph.child_changes("foo");

    assert_eq!(
      child_changes,
      packages[1..].iter().collect::<Vec<&Package<Semantic>>>()
    );
  }
}
