use std::path::Path;

use async_trait::async_trait;

use crate::package::{
  command::{PackageManagerCommand, PackageManagerCommandWithArgs},
  loader::PackageLoader,
  Package,
};
use crate::version::Versioned;

#[async_trait]
pub trait PackageManager {
  type Metadata: Clone + Send + Sync;
  type Loader: PackageLoader<Metadata = Self::Metadata> + Send + Sync;
  type Build: PackageManagerCommandWithArgs<Self::Metadata> + Send + Sync;
  type Publish: PackageManagerCommandWithArgs<Self::Metadata> + Send + Sync;
  type Validate: PackageManagerCommand<Self::Metadata> + Send + Sync;

  fn default_path() -> &'static str;

  async fn load_metadata<T: AsRef<Path> + Send + Sync>(
    crate_path: T,
  ) -> anyhow::Result<Self::Metadata>;

  async fn check_version<V: Versioned + Send + Sync + 'static>(
    &self,
    package: &Package<V>,
    metadata: &Self::Metadata,
  ) -> anyhow::Result<bool>;

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
    metadata: &Self::Metadata,
  ) -> anyhow::Result<()>;

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    name: &str,
    version: &str,
    metadata: &Self::Metadata,
  ) -> anyhow::Result<()>;
}
