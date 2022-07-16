use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Committer {
  /// The git author's name.
  pub name: String,
  /// The git author's email address.
  pub email: Option<String>,
  pub date: Option<String>,
  pub username: Option<String>,
}
