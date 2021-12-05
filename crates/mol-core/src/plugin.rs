use std::any::Any;
use std::ffi::{CString, OsStr};
use std::os::raw::c_char;

use anyhow::Context;
use libloading::{Library, Symbol};

use crate::error::PluginLoadError;
use crate::semantic::Semantic;
use crate::version::Versioned;

pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! declare_plugin {
  ($plugin_type:ty, $constructor:path) => {
    #[no_mangle]
    pub extern "C" fn _mol_core_version() -> *mut std::os::raw::c_char {
      std::ffi::CString::new(&*$crate::plugin::CORE_VERSION)
        .expect("CString::new failed")
        .into_raw()
    }

    #[no_mangle]
    pub extern "C" fn _mol_create() -> *mut $crate::plugin::Plugin {
      let constructor: fn() -> $plugin_type = $constructor;

      let object = constructor();
      let boxed: Box<$crate::plugin::Plugin> = Box::new(object);
      Box::into_raw(boxed)
    }
  };
}

pub trait Plugin: Any + Send + Sync {
  /// Name of the plugin
  fn name(&self) -> &'static str;
}

pub struct PluginManager {
  plugins: Vec<Box<dyn Plugin>>,
  loaded_libraries: Vec<Library>,
}

impl Default for PluginManager {
  fn default() -> Self {
    Self::new()
  }
}

impl PluginManager {
  pub fn new() -> PluginManager {
    PluginManager {
      plugins: Vec::new(),
      loaded_libraries: Vec::new(),
    }
  }

  /// # Safety
  ///
  /// This function opens a compiled cdylib and thus should not be called on cdylib that doesn't implement declare_plugin! macro
  pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> anyhow::Result<()> {
    type PluginVersion = unsafe fn() -> *mut c_char;
    type PluginCreate = unsafe fn() -> *mut dyn Plugin;

    let lib = {
      let lib = Library::new(filename.as_ref()).with_context(|| "Unable to load the plugin")?;

      self.loaded_libraries.push(lib);
      match self.loaded_libraries.last() {
        Some(lib) => lib,
        None => unreachable!(),
      }
    };

    let version_getter: Symbol<PluginVersion> = lib
      .get(b"_mol_core_version")
      .with_context(|| "The `_mol_core_version` symbol wasn't found.")?;

    let version = CString::from_raw(version_getter()).into_string()?;

    if Semantic::mask("*.*", &version) != Semantic::mask("*.*", CORE_VERSION) {
      return Err(PluginLoadError::IncompatibleVersion.into());
    }

    let constructor: Symbol<PluginCreate> = lib
      .get(b"_mol_create")
      .with_context(|| "The `_mol_create` symbol wasn't found.")?;
    let boxed_raw = constructor();

    let plugin = Box::from_raw(boxed_raw);
    println!("Loaded Plugin: {}", plugin.name());

    self.plugins.push(plugin);

    Ok(())
  }

  pub fn unload(&mut self) {
    for plugin in self.plugins.drain(..) {
      drop(plugin);
    }

    for lib in self.loaded_libraries.drain(..) {
      drop(lib);
    }
  }
}

impl Drop for PluginManager {
  fn drop(&mut self) {
    if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
      self.unload();
    }
  }
}
