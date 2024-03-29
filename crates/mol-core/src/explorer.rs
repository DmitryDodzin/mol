use std::path::PathBuf;
use std::sync::Arc;

use async_recursion::async_recursion;
use dashmap::DashSet;
use globset::GlobSet;
use tokio::fs;

use crate::package::Package;
use crate::package_manager::PackageManager;
use crate::version::Versioned;

fn remove_start_dot(dir: PathBuf) -> PathBuf {
  if dir.starts_with("./") {
    dir.iter().skip(1).collect()
  } else {
    dir
  }
}

pub struct Explorer;

impl Explorer {
  #[async_recursion]
  async fn seek_packeges_in_dir_entry<T, V>(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    entry: fs::DirEntry,
    metadata: Arc<T::Metadata>,
  ) -> anyhow::Result<Vec<Package<V>>>
  where
    T: PackageManager + Send + Sync + 'static,
    V: Versioned + Send + Sync + 'static,
    T::Metadata: Send + Sync,
  {
    let mut result = Vec::new();
    let entry_path = remove_start_dot(entry.path());

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
        return Explorer::seek_packages_in_directory::<T, V>(
          exists,
          globs,
          fs::read_dir(entry.path()).await?,
          metadata,
        )
        .await;
      }

      if file_type.is_symlink() {
        let link_value = fs::read_link(entry.path()).await?;
        return Explorer::seek_packages_in_directory::<T, V>(
          exists,
          globs,
          fs::read_dir(&link_value).await?,
          metadata,
        )
        .await;
      }

      if globs.is_match(entry_path) && file_type.is_file() && entry.file_name() == "Cargo.toml" {
        result.extend(T::seek_packages(entry.path(), &metadata).await?);
      }
    }

    Ok(result)
  }

  #[async_recursion]
  pub async fn seek_packages_in_directory<T, V>(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    mut current_dir: fs::ReadDir,
    metadata: Arc<T::Metadata>,
  ) -> anyhow::Result<Vec<Package<V>>>
  where
    T: PackageManager + Send + Sync + 'static,
    V: Versioned + Send + Sync + 'static,
    T::Metadata: Send + Sync,
  {
    let mut handles = Vec::new();

    while let Some(entry) = current_dir.next_entry().await? {
      let globs = globs.clone();
      let metadata = metadata.clone();
      handles.push(tokio::spawn(Explorer::seek_packeges_in_dir_entry::<T, V>(
        exists.clone(),
        globs,
        entry,
        metadata,
      )));
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
}
