use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstallationLite {
  /// The ID of the installation.
  pub id: u64,
  pub node_id: String,
}
