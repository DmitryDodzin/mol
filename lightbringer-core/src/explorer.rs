use std::fs::read_dir;

pub struct ExplorerError;

pub trait Explorer {
  fn list_dir(&self) -> Result<Vec<String>, std::io::Error> {
    let context = std::env::current_dir()?;

    Ok(
      read_dir(context)?
        .filter_map(|path| path.ok())
        .filter_map(|path| path.file_name().into_string().ok())
        .collect(),
    )
  }
  fn list_packages(&self) -> Result<Vec<String>, ExplorerError>;
}
