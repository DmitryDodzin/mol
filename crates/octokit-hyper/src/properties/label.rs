use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Label {
  pub id: u64,
  pub node_id: String,
  /// URL for the label
  pub url: String,
  /// The name of the label.
  pub name: String,
  pub description: Option<String>,
  /// 6-character hex code, without the leading #, identifying the color
  pub color: String,
  pub default: bool,
}
