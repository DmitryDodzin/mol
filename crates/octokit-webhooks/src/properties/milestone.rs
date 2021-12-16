use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::User;

#[derive(Debug, Deserialize)]
pub struct Milestone {
  pub url: String,
  pub html_url: String,
  pub labels_url: String,
  pub id: u64,
  pub node_id: String,
  /// The number of the milestone.
  pub number: u64,
  /// The title of the milestone.
  pub title: String,
  pub description: Option<String>,
  pub creator: User,
  pub open_issues: u64,
  pub closed_issues: u64,
  /// The state of the milestone.
  pub state: MilestoneState,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub due_on: Option<DateTime<Utc>>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MilestoneState {
  Open,
  Closed,
}
