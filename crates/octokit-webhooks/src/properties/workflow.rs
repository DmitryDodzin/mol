use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workflow {
  pub badge_url: String,
  pub created_at: String,
  pub html_url: String,
  pub id: u64,
  pub name: String,
  pub node_id: String,
  pub path: String,
  pub state: String,
  pub updated_at: String,
  pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
  Queued,
  InProgress,
  Completed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowConclusion {
  Success,
  Skipped,
  Failure,
}
