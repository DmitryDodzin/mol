use serde::Deserialize;

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
  pub created_at: String,
  pub updated_at: String,
  pub author_association: AuthorAssociation,
  /**
   * Contents of the issue comment
   */
  pub body: String,
  pub reactions: Reactions,
  pub performed_via_github_app: Option<App>,
}
