use async_trait::async_trait;

use mol_core::prelude::*;

mod add;
mod init;
mod publish;
mod version;

pub use add::Add;
pub use init::Init;
pub use publish::Publish;
pub use version::Version;

#[derive(Debug)]
pub struct ExecutableContext<T: PackageManager, V: Versioned> {
  pub dry_run: bool,
  pub package_manager: T,
  pub packages: Vec<Package<V>>,
}

pub trait IntoExecutableCommand<T: PackageManager, V: Versioned> {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T, V>>;
}

#[async_trait]
pub trait ExecutableCommand<T: PackageManager, V: Versioned> {
  async fn execute(
    &self,
    changesets: &Changesets,
    context: &ExecutableContext<T, V>,
  ) -> anyhow::Result<()>;
}
