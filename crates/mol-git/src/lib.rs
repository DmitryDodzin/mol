use std::path::Path;

use mol_core::{declare_plugin, prelude::*};

#[derive(Debug, Default)]
pub struct GitExt;

impl Plugin for GitExt {
  fn name(&self) -> &'static str {
    env!("CARGO_PKG_NAME")
  }

  fn pre_command(&self, changesets_path: &Path, repositry_path: &Path) {
    println!(
      "changesets: {:?} repo: {:?}",
      changesets_path, repositry_path
    );

    drop(git2::Repository::open(repositry_path));
  }
}

declare_plugin!(GitExt, GitExt::default);
