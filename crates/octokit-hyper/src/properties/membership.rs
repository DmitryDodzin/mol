use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::User;

#[derive(Debug, Deserialize)]
pub struct Membership {
  pub url: String,
  pub state: String,
  pub role: String,
  pub organization_url: String,
  pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct MemberInvitation {
  pub id: u64,
  pub node_id: String,
  pub login: String,
  pub email: Option<String>,
  pub role: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub failed_at: Option<DateTime<Utc>>,
  pub failed_reason: Option<String>,
  pub inviter: User,
  pub team_count: u64,
  pub invitation_teams_url: String,
}
