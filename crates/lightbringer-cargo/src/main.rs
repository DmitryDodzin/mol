use std::path::{Path, PathBuf};

use async_recursion::async_recursion;
use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::fs;
use toml_edit::Document;

#[async_recursion]
async fn check_dir(globs: &GlobSet, mut current_dir: fs::ReadDir) -> std::io::Result<()> {
  while let Some(entry) = current_dir.next_entry().await? {
    let entry_path = entry.path();

    println!("entring: {:?}", entry_path);

    if entry_path.starts_with("target") {
      continue;
    }

    if let Ok(file_type) = entry.file_type().await {
      if file_type.is_dir() {
        check_dir(globs, fs::read_dir(&entry_path).await?).await?;
        continue;
      }

      if globs.is_match(entry.path()) {
        if file_type.is_file() && entry.file_name() == "Cargo.toml" {
          read_package(&entry_path).await?;
        }
      }
    }
  }

  Ok(())
}

async fn read_package<T: AsRef<Path> + Into<PathBuf>>(crate_path: T) -> std::io::Result<()> {
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

    check_dir(
      &builder.build().expect("Globs did not set together"),
      fs::read_dir(crate_path.parent().unwrap_or_else(|| &crate_path)).await?,
    )
    .await?;
  }

  println!("{:?}", package_name);

  Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  read_package("Cargo.toml").await
}
