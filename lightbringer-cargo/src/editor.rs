use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use toml_edit::{Document, Item, Value};

use lightbringer_core::explorer::ExplorerError;

#[derive(Debug)]
pub enum PackageType {
  Workspace { memebers: Vec<String> },
  Package { name: String, version: String },
}

#[derive(Debug)]
pub struct CrateEditor(Document);

impl CrateEditor {
  pub fn load_editor(cargo_path: PathBuf) -> Result<CrateEditor, ExplorerError> {
    let mut file = File::open(cargo_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    match buffer.parse::<Document>() {
      Ok(document) => Ok(CrateEditor(document)),
      Err(_) => Err(ExplorerError),
    }
  }

  pub fn read_package(&self) -> Result<PackageType, ExplorerError> {
    for (key, value) in self.0.iter() {
      match key {
        "workspace" => match &value["members"] {
          Item::Value(Value::Array(array)) => {
            let memebers = array
              .iter()
              .filter_map(|item| match item {
                Value::String(val) => Some(val.value().clone()),
                _ => None,
              })
              .collect();

            return Ok(PackageType::Workspace { memebers });
          }
          _ => return Err(ExplorerError),
        },
        "package" => {
          return match (value["name"].as_str(), value["version"].as_str()) {
            (Some(name), Some(version)) => Ok(PackageType::Package {
              name: name.to_string(),
              version: version.to_string(),
            }),
            _ => Err(ExplorerError),
          };
        }
        _ => {}
      }
    }

    Err(ExplorerError)
  }
}
