use std::fs::read_dir;

use lightbringer_core::prelude::{Explorer, ExplorerError};

mod editor;

use editor::CrateEditor;

pub struct CargoExplorer;

impl Explorer for CargoExplorer {
  fn list_packages(&self) -> Result<Vec<String>, ExplorerError> {
    let context = std::env::current_dir()?;

    for entry in read_dir(&context)?.filter_map(|path| path.ok()) {
      match entry.file_name().to_str() {
        Some("Cargo.toml") => {
          let path = entry.path();
          if path.is_dir() {
            return Err(ExplorerError);
          }

          return Ok(CrateEditor::from(context).get_packages()?.list_names());
        }
        _ => {}
      }
    }

    Err(ExplorerError)
  }
}
