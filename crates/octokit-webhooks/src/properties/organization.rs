use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Organization {
  pub login: String,
  pub id: u32,
  pub node_id: String,
  pub url: String,
  pub html_url: Option<String>,
  pub repos_url: String,
  pub events_url: String,
  pub hooks_url: String,
  pub issues_url: String,
  pub members_url: String,
  pub public_members_url: String,
  pub avatar_url: String,
  pub description: Option<String>,
}
