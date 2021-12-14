use std::path::Path;

use async_trait::async_trait;

use crate::package::Package;
use crate::version::Versioned;

#[async_trait]
pub trait PackageManager {
  fn default_path() -> &'static str;

  async fn read_package<T: AsRef<Path> + Send + Sync, V: Versioned + Send + Sync + 'static>(
    &self,
    crate_path: T,
  ) -> anyhow::Result<Vec<Package<V>>>;

  async fn check_version<V: Versioned + Send + Sync + 'static>(
    &self,
    package: &Package<V>,
  ) -> anyhow::Result<bool>;

  async fn run_build<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    build_args: Vec<String>,
  ) -> anyhow::Result<()>;

  async fn run_publish<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    publish_args: Vec<String>,
    dry_run: bool,
  ) -> anyhow::Result<()>;

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> anyhow::Result<()>;

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    name: &str,
    version: &str,
  ) -> anyhow::Result<()>;
}
