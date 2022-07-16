use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::{App, AuthorAssociation, Reactions, User};

#[derive(Debug, Deserialize)]
pub struct IssueComment {
  /**
   * URL for the issue comment
   */
  pub url: String,
  pub html_url: String,
  pub issue_url: String,
  /**
   * Unique identifier of the issue comment
   */
  pub id: u64,
  pub node_id: String,
  pub user: User,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  pub author_association: AuthorAssociation,
  /**
   * Contents of the issue comment
   */
  pub body: String,
  pub reactions: Reactions,
  pub performed_via_github_app: Option<App>,
}
