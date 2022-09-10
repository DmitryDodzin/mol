use std::path::{Path, PathBuf};

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

  pub fn as_command(&self) -> CommandContext<'_, T::Metadata> {
    self.as_command_with_path(&self.root_dir)
  }

  pub fn as_command_with_path<'a>(&'a self, path: &'a Path) -> CommandContext<'a, T::Metadata> {
    CommandContext {
      dry_run: self.dry_run,
      metadata: &self.metadata,
      path,
    }
  }
}

impl<T, V> ExecutableContext<T, V>
where
  T: PackageManager + Default + Send + Sync,
  V: VersionEditor + Send + Sync + 'static,
  T::Validate: Send + Sync,
{
  pub async fn new(root_dir: PathBuf, dry_run: bool) -> anyhow::Result<Self> {
    let package_manager = T::default();

    let package_path: PathBuf = root_dir
      .iter()
      .chain(&PathBuf::from(T::default_path()))
      .collect();

    let metadata = T::load_metadata(&package_path).await?;

    let packages = T::Loader::load(&package_path, &metadata).await?;

    let context = ExecutableContext {
      changesets: Changesets::default(),
      dry_run,
      package_manager,
      packages,
      root_dir,
      metadata,
    };

    T::Validate::execute(&context.as_command())
      .await
      .with_context(|| format!("Validation error for package at dir {:?}", package_path))?;

    Ok(context)
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
    plugins: &PluginManager,
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
