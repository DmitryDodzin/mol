use std::ffi::OsStr;
use std::rc::Rc;

use libloading::Library;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Plugin {
  fn name(&self) -> &str;
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
}

impl PluginRegistrar {
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

pub struct PluginProxy {
  plugin: Box<dyn Plugin>,
  _lib: Rc<Library>,
}

impl Plugin for PluginProxy {
  fn name(&self) -> &str {
    self.plugin.name()
  }
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
    // load the library into memory
    let library = Rc::new(Library::new(library_path)?);

    // get a pointer to the plugin_declaration symbol.
    let decl = library
      .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
      .read();

    // version checks to prevent accidental ABI incompatibilities
    if decl.rustc_version != RUSTC_VERSION || decl.core_version != CORE_VERSION {
      println!("Rust versions incompatible");
      // return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
    }

    let mut registrar = PluginRegistrar::new(Rc::clone(&library));

    (decl.register)(&mut registrar);

    // add all loaded plugins to the functions map
    self.plugins.extend(registrar.plugins);
    // and make sure ExternalFunctions keeps a reference to the library
    self.libraries.push(library);

    Ok(())
  }
}
