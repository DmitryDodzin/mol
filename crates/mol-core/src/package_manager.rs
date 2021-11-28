use std::path::Path;

use async_trait::async_trait;

use crate::package::Package;
use crate::version::Versioned;

#[async_trait]
pub trait PackageManager {
  async fn read_package<T: AsRef<Path> + Send + Sync, V: Versioned + Send + Sync + 'static>(
    &self,
    crate_path: T,
  ) -> std::io::Result<Vec<Package<V>>>;

  async fn run_build<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    build_args: Vec<String>,
  ) -> std::io::Result<()>;

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> std::io::Result<()>;

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    name: &str,
    version: &str,
  ) -> std::io::Result<()>;
}
