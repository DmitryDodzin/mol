use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum UserType {
  Bot,
  User,
  Organization,
}

#[derive(Debug, Deserialize)]
pub struct User {
  pub login: String,
  pub id: u64,
  pub node_id: String,
  pub name: Option<String>,
  pub email: Option<String>,
  pub avatar_url: String,
  pub gravatar_id: String,
  pub url: String,
  pub html_url: String,
  pub followers_url: String,
  pub following_url: String,
  pub gists_url: String,
  pub starred_url: String,
  pub subscriptions_url: String,
  pub organizations_url: String,
  pub repos_url: String,
  pub events_url: String,
  pub received_events_url: String,
  pub r#type: UserType,
  pub site_admin: bool,
}
