use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CheckRunDeployment {
  pub url: String,
  pub id: u64,
  pub node_id: String,
  pub task: String,
  pub original_environment: String,
  pub environment: String,
  pub description: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub statuses_url: String,
  pub repository_url: String,
}
