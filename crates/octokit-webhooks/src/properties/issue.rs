use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{App, AuthorAssociation, Label, Milestone, Reactions, User};

#[derive(Debug, Deserialize)]
pub struct Issue {
  /**
   * URL for the issue
   */
  pub url: String,
  pub repository_url: String,
  pub labels_url: String,
  pub comments_url: String,
  pub events_url: String,
  pub html_url: String,
  pub id: u64,
  pub node_id: String,
  pub number: u64,
  /**
   * Title of the issue
   */
  pub title: String,
  pub user: User,
  pub labels: Option<Vec<Label>>,
  /**
  pub * State of the issue, either 'open' or 'closed'
  */
  pub state: Option<IssueState>,
  pub locked: Option<bool>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  pub milestone: Option<Milestone>,
  pub comments: u64,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub closed_at: Option<DateTime<Utc>>,
  pub author_association: AuthorAssociation,
  pub active_lock_reason: Option<IssueActiveLockReason>,
  pub draft: bool,
  pub performed_via_github_app: Option<App>,
  pub pull_request: Option<IssuePullRequestRef>,
  /**
   * Contents of the issue
   */
  pub body: Option<String>,
  pub reactions: Reactions,
  pub timeline_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
  Open,
  Closed,
}

#[derive(Debug, Deserialize)]
pub enum IssueActiveLockReason {
  #[serde(rename = "resolved")]
  Resolved,
  #[serde(rename = "off-topic")]
  OffTopic,
  #[serde(rename = "too heated")]
  TooHeated,
  #[serde(rename = "spam")]
  Spam,
}

#[derive(Debug, Deserialize)]
pub struct IssuePullRequestRef {
  pub url: Option<String>,
  pub html_url: Option<String>,
  pub diff_url: Option<String>,
  pub patch_url: Option<String>,
}
