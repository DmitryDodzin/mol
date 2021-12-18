use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Team {
  /// Name of the team
  pub name: String,
  /// Unique identifier of the team
  pub id: u64,
  pub node_id: String,
  pub slug: String,
  /// Description of the team
  pub description: Option<String>,
  pub privacy: TeamPrivacy,
  /// URL for the team
  pub url: String,
  pub html_url: String,
  pub members_url: String,
  pub repositories_url: String,
  /// Permission that the team will have for its repositories
  pub permission: String,
  pub parent: Option<TeamParent>,
}

#[derive(Debug, Deserialize)]
pub struct TeamParent {
  /// Name of the team
  pub name: String,
  /// Unique identifier of the team
  pub id: u64,
  pub node_id: String,
  pub slug: String,
  /// Description of the team
  pub description: Option<String>,
  pub privacy: TeamPrivacy,
  /// URL for the team
  pub url: String,
  pub html_url: String,
  pub members_url: String,
  pub repositories_url: String,
  /// Permission that the team will have for its repositories
  pub permission: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamPrivacy {
  Open,
  Closed,
  Secret,
}
