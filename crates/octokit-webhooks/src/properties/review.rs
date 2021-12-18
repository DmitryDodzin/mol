use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::{AuthorAssociation, Link, User};

#[derive(Debug, Deserialize)]
pub struct Review {
  /// Unique identifier of the review
  pub id: u64,
  pub node_id: String,
  pub user: User,
  /// The text of the review.
  pub body: Option<String>,
  /// A commit SHA for the review.
  pub commit_id: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub submitted_at: DateTime<Utc>,
  pub state: ReviewState,
  pub html_url: String,
  pub pull_request_url: String,
  pub author_association: AuthorAssociation,
  pub _links: ReviewLinks,
}

#[derive(Debug, Deserialize)]
pub struct ReviewLinks {
  pub html: Link,
  pub pull_request: Link,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewState {
  Commented,
  Dismissed,
}
