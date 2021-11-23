use std::path::Path;

use async_trait::async_trait;

use crate::package::Package;

#[async_trait]
pub trait PackageManager {
  async fn read_package<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
  ) -> std::io::Result<Vec<Package>>;
  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> std::io::Result<()>;
}
