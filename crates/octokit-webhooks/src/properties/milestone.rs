use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::User;

#[derive(Debug, Deserialize)]
pub struct Milestone {
  pub url: String,
  pub html_url: String,
  pub labels_url: String,
  pub id: u32,
  pub node_id: String,
  /// The number of the milestone.
  pub number: u32,
  /// The title of the milestone.
  pub title: String,
  pub description: Option<String>,
  pub creator: User,
  pub open_issues: u32,
  pub closed_issues: u32,
  /// The state of the milestone.
  pub state: MilestoneState,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub due_on: Option<DateTime<Utc>>,
  pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MilestoneState {
  Open,
  Closed,
}
