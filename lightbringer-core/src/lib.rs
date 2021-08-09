pub mod changeset;
pub mod explorer;
pub mod version;

pub struct Lightbringer<'a> {
  explorer: Box<dyn explorer::Explorer + 'a>,
}

impl<'a, T: 'a + explorer::Explorer> From<T> for Lightbringer<'a> {
  fn from(explorer: T) -> Self {
    Lightbringer {
      explorer: Box::new(explorer),
    }
  }
}

impl<'a> Lightbringer<'a> {
  pub fn get_packages(&self) -> Vec<String> {
    self.explorer.list_packages().unwrap_or(vec![])
  }
}
