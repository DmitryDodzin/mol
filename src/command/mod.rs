use std::path::PathBuf;

use async_trait::async_trait;

use mol_core::prelude::*;

mod add;
mod init;
mod version;

pub use add::Add;
pub use init::Init;
pub use version::Version;

#[derive(Debug)]
pub struct ExecutableContext<T: PackageManager> {
  pub dry_run: bool,
  pub package_manager: T,
  pub packages: Vec<(PathBuf, String, String)>,
}

pub trait IntoExecutableCommand<T: PackageManager> {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T>>;
}

#[async_trait]
pub trait ExecutableCommand<T: PackageManager> {
  async fn execute(
    &self,
    changesets: &Changesets,
    context: &ExecutableContext<T>,
  ) -> anyhow::Result<()>;
}
