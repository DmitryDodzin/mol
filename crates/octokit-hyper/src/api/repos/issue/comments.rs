use serde::Serialize;

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

#[cfg(feature = "client")]
pub mod client {
  use crate::octokit_request;
  use crate::request::{middleware::Unauthorized, proxy::RequestProxy};

  use super::{IssueCreateCommentBody, IssueListCommentQuery};

  pub fn create_issue_comment(
    owner: &str,
    repo: &str,
    issue_number: u64,
    payload: IssueCreateCommentBody,
  ) -> RequestProxy<(), Unauthorized> {
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
  ) -> RequestProxy<(), Unauthorized> {
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
}
