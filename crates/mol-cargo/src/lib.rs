use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use dashmap::DashSet;
use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::fs;
use toml_edit::{value, Document};

use mol_core::prelude::*;

#[derive(Default)]
pub struct Cargo;

impl Cargo {
  #[async_recursion]
  async fn check_dir(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    entry: fs::DirEntry,
  ) -> std::io::Result<Vec<Package>> {
    let mut result = Vec::new();
    let entry_path = entry.path();

    if exists.contains(&entry_path) {
      return Ok(result);
    } else {
      exists.insert(entry_path.clone());
    }

    if entry_path.starts_with("target") || entry_path.starts_with(".git") {
      return Ok(result);
    }

    if let Ok(file_type) = entry.file_type().await {
      if file_type.is_dir() {
        return Cargo::check_read_dir(exists, globs, fs::read_dir(&entry_path).await?).await;
      }

      if file_type.is_symlink() {
        let entry_path = fs::read_link(&entry_path).await?;
        return Cargo::check_read_dir(exists, globs, fs::read_dir(&entry_path).await?).await;
      }

      if globs.is_match(entry.path()) && file_type.is_file() && entry.file_name() == "Cargo.toml" {
        result.extend(Cargo.read_package(&entry_path).await?);
      }
    }

    Ok(result)
  }

  #[async_recursion]
  async fn check_read_dir(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    mut current_dir: fs::ReadDir,
  ) -> std::io::Result<Vec<Package>> {
    let mut handles = Vec::new();

    while let Some(entry) = current_dir.next_entry().await? {
      let globs = globs.clone();
      handles.push(tokio::spawn(Cargo::check_dir(exists.clone(), globs, entry)));
    }

    let mut result = Vec::new();

    for task in futures::future::join_all(handles)
      .await
      .into_iter()
      .flatten()
    {
      result.extend(task?);
    }

    Ok(result)
  }

  async fn load_document<T: AsRef<Path>>(
    &self,
    crate_path: T,
  ) -> std::io::Result<(PathBuf, Document)> {
    let document = fs::read_to_string(&crate_path)
      .await?
      .parse::<Document>()
      .expect("Invalid Cargo.toml");

    Ok((crate_path.as_ref().to_path_buf(), document))
  }
}

#[async_trait]
impl PackageManager for Cargo {
  async fn read_package<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
  ) -> std::io::Result<Vec<Package>> {
    let mut result = Vec::new();
    let (crate_path, document) = self.load_document(crate_path).await?;

    let (package_name, version) = if document.contains_key("package") {
      (
        document["package"]["name"].as_str(),
        document["package"]["version"].as_str(),
      )
    } else {
      (None, None)
    };

    if let (Some(package_name), Some(version)) = (package_name, version) {
      result.push(Package {
        path: crate_path.clone(),
        name: package_name.to_owned(),
        version: version.to_owned(),
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

      let exists = Arc::new(DashSet::new());

      result.extend(
        Cargo::check_read_dir(
          exists,
          builder.build().expect("Globs did not set together"),
          fs::read_dir(crate_path.parent().unwrap_or(&crate_path)).await?,
        )
        .await?,
      );
    }

    Ok(result)
  }

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> std::io::Result<()> {
    let (crate_path, mut document) = self.load_document(crate_path).await?;

    if document.contains_key("package") {
      document["package"]["version"] = value(version);
    }

    fs::write(&crate_path, document.to_string()).await?;

    Ok(())
  }
}
