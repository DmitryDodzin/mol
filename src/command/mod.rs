use std::sync::Arc;

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

pub struct ExecutableContext<T: PackageManager, V: VersionEditor> {
  pub changesets: Changesets,
  pub dry_run: bool,
  pub package_manager: T,
  pub packages: Vec<Package<V>>,
  pub plugins: Arc<PluginManager>,
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager,
  V: VersionEditor,
{
  pub fn as_plugin<'a>(&'a self) -> PluginContext<'a> {
    PluginContext {
      dry_run: self.dry_run,
      config: &self.changesets,
      // packages: &self.packages as &Vec<dyn Versioned>,
    }
  }
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager + Default + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
{
  pub async fn new(plugins: Arc<PluginManager>, dry_run: bool) -> anyhow::Result<Self> {
    let package_manager = T::default();

    let packages = package_manager.read_package(T::default_path()).await?;

    Ok(ExecutableContext {
      changesets: Changesets::default(),
      dry_run,
      package_manager,
      packages,
      plugins,
    })
  }
}

pub trait IntoExecutableCommand<T: PackageManager, V: VersionEditor> {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T, V>>;
}

#[async_trait]
pub trait ExecutableCommand<T: PackageManager, V: VersionEditor> {
  async fn execute(&self, context: &ExecutableContext<T, V>) -> anyhow::Result<()>;
}

unsafe impl<T, V> Send for ExecutableContext<T, V>
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync,
{
}
unsafe impl<T, V> Sync for ExecutableContext<T, V>
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync,
{
}
