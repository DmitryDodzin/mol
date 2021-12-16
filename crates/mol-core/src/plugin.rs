use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

use anyhow::Context;
use libloading::Library;

use crate::changesets::Changesets;
use crate::error::PluginLoadError;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub struct PluginContext<'a> {
  pub root_dir: &'a Path,
  pub dry_run: bool,
  pub config: &'a Changesets,
}

pub trait Plugin {
  fn name(&self) -> &str;

  fn on_load(&mut self, _context: &PluginContext) -> anyhow::Result<()> {
    Ok(())
  }

  fn on_unload(&mut self) {}

  fn pre_command(&self, _command: &str, _context: &PluginContext) -> anyhow::Result<()> {
    Ok(())
  }

  fn post_command(&self, _command: &str, _context: &PluginContext) -> anyhow::Result<()> {
    Ok(())
  }
}

pub struct PluginProxy {
  plugin: Box<dyn Plugin>,
  _lib: Rc<Library>,
}

impl Plugin for PluginProxy {
  fn name(&self) -> &str {
    self.plugin.name()
  }

  fn on_load(&mut self, context: &PluginContext) -> anyhow::Result<()> {
    self.plugin.on_load(context)
  }

  fn on_unload(&mut self) {
    self.plugin.on_unload()
  }

  fn pre_command(&self, command: &str, context: &PluginContext) -> anyhow::Result<()> {
    self.plugin.pre_command(command, context)
  }

  fn post_command(&self, command: &str, context: &PluginContext) -> anyhow::Result<()> {
    self.plugin.post_command(command, context)
  }
}

#[repr(C)]
pub struct PluginRegistrar {
  plugins: Vec<PluginProxy>,
  lib: Rc<Library>,
}

impl PluginRegistrar {
  fn new(lib: Rc<Library>) -> PluginRegistrar {
    PluginRegistrar {
      lib,
      plugins: Default::default(),
    }
  }

  fn consume(self) -> Vec<PluginProxy> {
    self.plugins
  }

  pub fn register(&mut self, plugin: Box<dyn Plugin>) {
    let proxy = PluginProxy {
      plugin,
      _lib: Rc::clone(&self.lib),
    };
    self.plugins.push(proxy);
  }
}

pub struct PluginDeclaration {
  pub rustc_version: &'static str,
  pub core_version: &'static str,
  pub register: unsafe extern "C" fn(&mut PluginRegistrar),
}

#[macro_export]
macro_rules! declare_plugin {
  ($register:expr) => {
    #[doc(hidden)]
    #[no_mangle]
    pub static plugin_declaration: $crate::plugin::PluginDeclaration =
      $crate::plugin::PluginDeclaration {
        rustc_version: $crate::plugin::RUSTC_VERSION,
        core_version: $crate::plugin::CORE_VERSION,
        register: $register,
      };
  };
}

#[derive(Default)]
pub struct PluginManager {
  pub plugins: Vec<PluginProxy>,
  libraries: Vec<Rc<Library>>,
}

impl PluginManager {
  /// # Safety
  ///
  /// This function opens a compiled cdylib and thus should not be called on cdylib that doesn't implement declare_plugin! macro
  pub unsafe fn load<P: AsRef<OsStr>>(
    &mut self,
    library_path: P,
    context: &PluginContext,
  ) -> Result<(), PluginLoadError> {
    let library = Rc::new(
      Library::new(&library_path)
        .with_context(|| format!("Unable to load library at {:?}", library_path.as_ref()))?,
    );

    let decl = library
      .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
      .with_context(|| {
        format!(
          "Unable to read plugin_declaration at {:?}",
          library_path.as_ref()
        )
      })?
      .read();

    // version checks to prevent accidental ABI incompatibilities
    if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
      return Err(PluginLoadError::IncompatibleVersion(
        decl.core_version.to_owned(),
        decl.rustc_version.to_owned(),
      ));
    }

    let mut registrar = PluginRegistrar::new(Rc::clone(&library));

    (decl.register)(&mut registrar);

    let mut plugins = registrar.consume();

    for plugin in &mut plugins {
      plugin.on_load(context).with_context(|| {
        format!(
          "Initialization error at plugin: {} for library at {:?}",
          plugin.name(),
          library_path.as_ref()
        )
      })?;
    }

    self.plugins.extend(plugins);
    self.libraries.push(library);

    Ok(())
  }
}

unsafe impl Send for PluginManager {}

unsafe impl Sync for PluginManager {}

impl Drop for PluginManager {
  fn drop(&mut self) {
    for mut plugin in self.plugins.drain(..) {
      plugin.on_unload();
    }

    for library in self.libraries.drain(..) {
      drop(library);
    }
  }
}

impl Plugin for PluginManager {
  fn name(&self) -> &str {
    "PluginManager"
  }

  fn pre_command(&self, command: &str, context: &PluginContext) -> anyhow::Result<()> {
    for plugin in &self.plugins {
      plugin
        .pre_command(command, context)
        .with_context(|| format!("Faliure at pre-command for {} plugin", plugin.name()))?;
    }

    Ok(())
  }

  fn post_command(&self, command: &str, context: &PluginContext) -> anyhow::Result<()> {
    for plugin in &self.plugins {
      plugin
        .post_command(command, context)
        .with_context(|| format!("Faliure at post-command for {} plugin", plugin.name()))?;
    }

    Ok(())
  }
}
