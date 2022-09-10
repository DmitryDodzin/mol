use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
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
  pub root_dir: PathBuf,
  pub metadata: T::Metadata,
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager,
  V: VersionEditor,
{
  pub fn as_plugin(&self) -> PluginContext<'_> {
    PluginContext {
      dry_run: self.dry_run,
      config: &self.changesets,
      root_dir: &self.root_dir,
    }
  }
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager + Default + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
{
  pub async fn new(root_dir: PathBuf, dry_run: bool) -> anyhow::Result<Self> {
    let package_manager = T::default();

    let package_path: PathBuf = root_dir
      .iter()
      .chain(&PathBuf::from(T::default_path()))
      .collect();

    let metadata = T::load_metadata(&package_path).await?;

    T::validate_package(&package_path, &metadata)
      .await
      .with_context(|| format!("Validation error for package at dir {:?}", package_path))?;

    let packages = T::seek_packages(&package_path, &metadata)
      .await
      .with_context(|| format!("Could not open read pacakges at dir {:?}", package_path))?;

    Ok(ExecutableContext {
      changesets: Changesets::default(),
      dry_run,
      package_manager,
      packages,
      root_dir,
      metadata,
    })
  }
}

pub trait IntoExecutableCommand<T: PackageManager, V: VersionEditor + 'static> {
  fn as_executable(&self) -> Option<&dyn ExecutableCommand<T, V>>;
}

#[async_trait]
pub trait ExecutableCommand<T: PackageManager, V: VersionEditor + 'static> {
  async fn execute(
    &self,
    context: &ExecutableContext<T, V>,
    plugins: Arc<PluginManager>,
  ) -> anyhow::Result<()>;
}

unsafe impl<T, V> Send for ExecutableContext<T, V>
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
{
}
unsafe impl<T, V> Sync for ExecutableContext<T, V>
where
  T: PackageManager + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
{
}
