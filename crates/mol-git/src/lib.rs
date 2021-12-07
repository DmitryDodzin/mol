use mol_core::{declare_plugin, prelude::*};

#[repr(C)]
#[derive(Debug, Default)]
pub struct GitExt;

impl Plugin for GitExt {
  fn name(&self) -> &str {
    match git2::Repository::open(".") {
      Ok(repo) => println!("{:?}", repo.state()),
      Err(err) => println!("{:?}", err),
    }

    env!("CARGO_PKG_NAME")
  }
}

extern "C" fn register(registrar: &mut PluginRegistrar) {
  registrar.register(Box::new(GitExt::default()));
}

declare_plugin!(register);
