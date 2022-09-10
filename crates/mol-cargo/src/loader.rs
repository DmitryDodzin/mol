use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashSet;
use globset::{Glob, GlobSetBuilder};
use thiserror::Error;
use tokio::fs;

use crate::Cargo;
use crate::CrateMetadata;

use mol_core::prelude::*;

#[derive(Error, Debug)]
pub enum CrateLoaderError {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("{0}")]
  Generic(#[from] anyhow::Error),
}

pub struct CrateLoader;

#[async_trait]
impl PackageLoader for CrateLoader {
  type Metadata = CrateMetadata;
  type Error = CrateLoaderError;

  async fn load<T: AsRef<Path> + Send + Sync, V: Versioned + Send + Sync + 'static>(
    crate_path: T,
    metadata: &CrateMetadata,
  ) -> Result<Vec<Package<V>>, Self::Error> {
    let mut result = Vec::new();
    let (crate_path, document) = Cargo::load_document(crate_path).await?;

    let (package_name, version) = if document.contains_key("package") {
      (
        document["package"]["name"].as_str(),
        document["package"]["version"].as_str(),
      )
    } else {
      (None, None)
    };

    let mut dependencies = Vec::new();

    if document.contains_key("dependencies") {
      if let Some(deps) = document["dependencies"].as_table() {
        for (key, value) in deps.iter() {
          if value.is_str() {
            dependencies.push((
              key.to_owned(),
              value.as_str().unwrap_or_default().to_owned(),
            ));
          } else if value.is_table() || value.is_inline_table() {
            let version = if value.get("version").is_some() {
              value["version"].as_str().unwrap_or_default().to_owned()
            } else {
              "*".to_owned()
            };

            dependencies.push((key.to_owned(), version));
          }
        }
      }
    }

    if let (Some(package_name), Some(version)) = (package_name, version) {
      result.push(Package {
        path: crate_path.clone(),
        name: package_name.to_owned(),
        version: version.into(),
        dependencies,
      });
    }

    let workspace = if document.contains_key("workspace") {
      document["workspace"]["members"].as_array().map(|val| {
        val
          .iter()
          .filter_map(|v| v.as_str())
          .filter_map(|glob| Glob::new(glob).ok())
          .collect::<Vec<Glob>>()
      })
    } else {
      None
    };

    if let Some(ref workspace) = workspace {
      let mut builder = GlobSetBuilder::new();

      for glob in workspace {
        builder.add(glob.clone());
      }

      let metadata = Arc::new(metadata.clone());

      result.extend(
        Explorer::seek_packages_in_directory::<Self, V>(
          Arc::new(DashSet::new()),
          builder.build().expect("Globs did not set together"),
          fs::read_dir(crate_path.parent().unwrap_or(&crate_path)).await?,
          metadata,
        )
        .await?,
      );
    }

    Ok(result)
  }
}
