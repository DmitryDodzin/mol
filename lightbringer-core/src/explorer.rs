#[derive(Debug)]
pub struct ExplorerError;

impl From<std::io::Error> for ExplorerError {
  fn from(_: std::io::Error) -> Self {
    ExplorerError
  }
}

pub trait Explorer {
  fn list_packages(&self) -> Result<Vec<String>, ExplorerError>;
}

#[derive(Debug)]
pub struct EmptyExplorer;

impl Explorer for EmptyExplorer {
  fn list_packages(&self) -> Result<Vec<String>, ExplorerError> {
    Ok(vec![])
  }
}
