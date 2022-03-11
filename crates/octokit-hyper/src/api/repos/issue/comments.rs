use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::properties::{Reactions, User};
use crate::util::parse_flexible_timestamp;

#[derive(Debug, Default, Serialize)]
pub struct IssueCreateCommentBody {
  pub body: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueListCommentDirection {
  Asc,
  Desc,
}

#[derive(Debug, Default, Serialize)]
pub struct IssueListCommentQuery {
  pub direction: Option<IssueListCommentDirection>,
  pub per_page: Option<i32>,
  pub page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct IssueListCommentReply {
  pub url: String,
  pub html_url: String,
  pub issue_url: String,
  pub id: u64,
  pub node_id: String,
  pub user: User,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  pub author_association: String,
  pub body: String,
  pub reactions: Reactions,
}

#[cfg(feature = "client")]
pub mod client {
  use crate::octokit_request;
  use crate::request::{middleware::Unauthorized, proxy::RequestProxy};

  use super::{IssueCreateCommentBody, IssueListCommentQuery, IssueListCommentReply};

  pub fn create_issue_comment(
    owner: &str,
    repo: &str,
    issue_number: u64,
    payload: IssueCreateCommentBody,
  ) -> RequestProxy<IssueListCommentReply, Unauthorized> {
    let builder = octokit_request!(
      POST,
      "/repos/{owner}/{repo}/issues/{issue_number}/comments",
      owner = owner,
      repo = repo,
      issue_number = issue_number
    );

    RequestProxy::with_body(
      builder,
      Box::new(move || {
        serde_json::to_vec(&payload)
          .map(|vec| vec.into())
          .map_err(|err| err.into())
      }),
    )
  }

  pub fn list_issue_comment(
    owner: &str,
    repo: &str,
    issue_number: u64,
    query: IssueListCommentQuery,
  ) -> RequestProxy<Vec<IssueListCommentReply>, Unauthorized> {
    // TODO: add paganation
    let query = serde_urlencoded::to_string(&query).unwrap_or_else(|_| String::new());

    let builder = octokit_request!(
      GET,
      "/repos/{owner}/{repo}/issues/{issue_number}/comments?{query}",
      owner = owner,
      repo = repo,
      issue_number = issue_number,
      query = query
    );

    RequestProxy::new(builder)
  }

  pub fn delete_issue_comment(
    owner: &str,
    repo: &str,
    comment_id: u64,
  ) -> RequestProxy<(), Unauthorized> {
    let builder = octokit_request!(
      DELETE,
      "/repos/{owner}/{repo}/issues/comments/{comment_id}",
      owner = owner,
      repo = repo,
      comment_id = comment_id
    );

    RequestProxy::new(builder)
  }
}
