use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::path::PathBuf;

use globset::Glob;
use toml_edit::{Document, Item, Value};

use lightbringer_core::explorer::ExplorerError;

#[derive(Debug)]
pub enum PackageType {
  Workspace { memebers: Vec<PackageType> },
  Package { name: String, version: String },
}

impl PackageType {
  pub fn list_names(&self) -> Vec<String> {
    match self {
      Self::Workspace { memebers } => memebers.iter().flat_map(|p| p.list_names()).collect(),
      Self::Package { name, version: _ } => vec![name.clone()],
    }
  }
}

#[derive(Debug)]
pub struct CrateEditor(PathBuf);

impl From<PathBuf> for CrateEditor {
  fn from(root_path: PathBuf) -> Self {
    CrateEditor(root_path)
  }
}

impl CrateEditor {
  fn load_document(cargo_path: &PathBuf) -> Result<Document, ExplorerError> {
    let mut file = File::open(cargo_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    match buffer.parse::<Document>() {
      Ok(document) => Ok(document),
      Err(_) => Err(ExplorerError),
    }
  }

  fn read_package(cargo_path: &PathBuf) -> Result<PackageType, ExplorerError> {
    let document = CrateEditor::load_document(&cargo_path.join("Cargo.toml"))?;
    for (key, value) in document.iter() {
      match key {
        "workspace" => {
          let workspace: Vec<DirEntry> = read_dir(&cargo_path)?
            .filter_map(|item| item.ok())
            .collect();
          match &value["members"] {
            Item::Value(Value::Array(array)) => {
              let memebers = array
                .iter()
                .filter_map(|item| match item {
                  Value::String(val) => Some(val.value()),
                  _ => None,
                })
                .filter_map(|glob| Glob::new(glob).ok())
                .map(|glob| glob.compile_matcher())
                .flat_map(|glob| {
                  workspace
                    .iter()
                    .filter(move |item| glob.is_match(item.file_name()))
                })
                .filter_map(|dir| CrateEditor::read_package(&dir.path()).ok())
                .collect();

              return Ok(PackageType::Workspace { memebers });
            }
            _ => return Err(ExplorerError),
          }
        }
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

  pub fn get_packages(&self) -> Result<PackageType, ExplorerError> {
    CrateEditor::read_package(&self.0)
  }
}
