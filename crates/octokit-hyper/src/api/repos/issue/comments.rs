use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct IssueCreateCommentBody {
  pub body: String,
}

#[cfg(feature = "client")]
pub mod client {
  use crate::octokit_request;
  use crate::request::{middleware::Unauthorized, proxy::RequestProxy};

  use super::IssueCreateCommentBody;

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
}
