use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_recursion::async_recursion;
use dashmap::DashSet;
use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::fs;
use toml_edit::Document;

#[async_recursion]
async fn check_dir(
  exists: Arc<DashSet<PathBuf>>,
  globs: GlobSet,
  entry: fs::DirEntry,
) -> std::io::Result<Vec<(PathBuf, String)>> {
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
      return check_read_dir(exists, globs, fs::read_dir(&entry_path).await?).await;
    }

    if file_type.is_symlink() {
      let entry_path = fs::read_link(&entry_path).await?;
      return check_read_dir(exists, globs, fs::read_dir(&entry_path).await?).await;
    }

    if globs.is_match(entry.path()) && file_type.is_file() && entry.file_name() == "Cargo.toml" {
      result.extend(read_package(&entry_path).await?);
    }
  }

  Ok(result)
}

#[async_recursion]
async fn check_read_dir(
  exists: Arc<DashSet<PathBuf>>,
  globs: GlobSet,
  mut current_dir: fs::ReadDir,
) -> std::io::Result<Vec<(PathBuf, String)>> {
  let mut handles = Vec::new();

  while let Some(entry) = current_dir.next_entry().await? {
    let globs = globs.clone();
    handles.push(tokio::spawn(check_dir(exists.clone(), globs, entry)));
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

async fn read_package<T: AsRef<Path> + Into<PathBuf>>(
  crate_path: T,
) -> std::io::Result<Vec<(PathBuf, String)>> {
  let mut result = Vec::new();
  let crate_path = crate_path.into();

  let cargo = fs::read_to_string(&crate_path)
    .await?
    .parse::<Document>()
    .expect("Invalid Cargo.toml");

  let package_name = if cargo.contains_key("package") {
    cargo["package"]["name"].as_str()
  } else {
    None
  };

  if let Some(package_name) = package_name {
    result.push((crate_path.clone(), package_name.to_owned()));
  }

  let workspace = if cargo.contains_key("workspace") {
    cargo["workspace"]["members"].as_array().map(|val| {
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
      check_read_dir(
        exists,
        builder.build().expect("Globs did not set together"),
        fs::read_dir(crate_path.parent().unwrap_or(&crate_path)).await?,
      )
      .await?,
    );
  }

  Ok(result)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let packages = read_package("Cargo.toml").await?;

  println!("{:#?}", packages);

  Ok(())
}
