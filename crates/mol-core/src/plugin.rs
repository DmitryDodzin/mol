use std::ffi::OsStr;
use std::rc::Rc;

use libloading::Library;

use crate::error::PluginLoadError;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait ToBox {
  fn to_box(self) -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(self)
  }
}

pub trait Plugin {
  fn name(&self) -> &str;

  fn on_load(&mut self) {}

  fn on_unload(&mut self) {}
}

pub struct PluginProxy {
  plugin: Box<dyn Plugin>,
  _lib: Rc<Library>,
}

impl Plugin for PluginProxy {
  fn name(&self) -> &str {
    self.plugin.name()
  }

  fn on_load(&mut self) {
    self.plugin.on_load()
  }

  fn on_unload(&mut self) {
    self.plugin.on_unload()
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
    let mut proxy = PluginProxy {
      plugin,
      _lib: Rc::clone(&self.lib),
    };
    proxy.on_load();
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
  pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> anyhow::Result<()> {
    let library = Rc::new(Library::new(library_path)?);

    let decl = library
      .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
      .read();

    // version checks to prevent accidental ABI incompatibilities
    if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
      return Err(PluginLoadError::IncompatibleVersion.into());
    }

    let mut registrar = PluginRegistrar::new(Rc::clone(&library));

    (decl.register)(&mut registrar);

    self.plugins.extend(registrar.consume());
    self.libraries.push(library);

    Ok(())
  }
}

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
