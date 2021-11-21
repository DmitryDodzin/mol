use std::path::{Path, PathBuf};

use async_trait::async_trait;

#[async_trait]
pub trait PackageManager {
  async fn read_package<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
  ) -> std::io::Result<Vec<(PathBuf, String, String)>>;
  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> std::io::Result<()>;
}
