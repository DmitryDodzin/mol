use std::{error::Error, path::Path};

use async_trait::async_trait;

use crate::package::Package;
use crate::version::Versioned;

#[async_trait]
pub trait PackageLoader {
  type Error: Error + Send + Sync + 'static;
  type Metadata;

  async fn load<T, V>(
    crate_path: T,
    metadata: &Self::Metadata,
  ) -> Result<Vec<Package<V>>, Self::Error>
  where
    T: AsRef<Path> + Send + Sync,
    V: Versioned + Send + Sync + 'static;
}
