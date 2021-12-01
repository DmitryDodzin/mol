use mol_core::{declare_plugin, prelude::*};

#[derive(Debug, Default)]
pub struct GitExt;

impl Plugin for GitExt {
  fn name(&self) -> &'static str {
    env!("CARGO_PKG_NAME")
  }
}

declare_plugin!(GitExt, GitExt::default);
