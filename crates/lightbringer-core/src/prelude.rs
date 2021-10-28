pub use crate::changeset::Changeset;
pub use crate::error::{ChangesetParseError, ExplorerError, VersionParseError};
pub use crate::explorer::Explorer;
pub use crate::version::Version;

pub struct Lightbringer<'a> {
  explorer: Box<dyn Explorer + 'a>,
}

impl<'a, T: 'a + Explorer> From<T> for Lightbringer<'a> {
  fn from(explorer: T) -> Self {
    Lightbringer {
      explorer: Box::new(explorer),
    }
  }
}

impl<'a> Lightbringer<'a> {
  pub fn get_packages(&self) -> Vec<String> {
    self.explorer.list_packages().unwrap_or_else(|_| vec![])
  }
}
