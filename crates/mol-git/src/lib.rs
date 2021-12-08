use git2::Repository;

use mol_core::{declare_plugin, prelude::*};

#[repr(C)]
#[derive(ToBox, Default)]
pub struct GitExt {
  repo: Option<Repository>,
}

impl Plugin for GitExt {
  fn name(&self) -> &str {
    env!("CARGO_PKG_NAME")
  }
  fn on_load(&mut self) {
    self.repo = Repository::open(DEFAULT_PACKAGE_DIR.as_path()).ok();
  }
}

extern "C" fn register(registrar: &mut PluginRegistrar) {
  registrar.register(GitExt::default().to_box());
}

declare_plugin!(register);
