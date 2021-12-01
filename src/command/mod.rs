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

pub struct ExecutableContext<T: PackageManager, V: Versioned> {
  pub changesets: Changesets,
  pub dry_run: bool,
  pub package_manager: T,
  pub packages: Vec<Package<V>>,
  pub plugin_manager: PluginManager,
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager + Default + Send + Sync,
  V: Versioned + Send + Sync + 'static,
{
  pub async fn new(dry_run: bool) -> anyhow::Result<Self> {
    let package_manager = T::default();

    let packages = package_manager.read_package(T::default_path()).await?;

    Ok(ExecutableContext {
      changesets: Changesets::default(),
      dry_run,
      package_manager,
      packages,
      plugin_manager: PluginManager::default(),
    })
  }
}

pub trait IntoExecutableCommand<T: PackageManager, V: Versioned> {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T, V>>;
}

#[async_trait]
pub trait ExecutableCommand<T: PackageManager, V: Versioned> {
  async fn execute(&self, context: &ExecutableContext<T, V>) -> anyhow::Result<()>;
}
