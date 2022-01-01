use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateReviewEvent {
  Approve,
  Comment,
  Pending,
  RequestChanges,
}

#[derive(Debug, Default, Serialize)]
pub struct PullRequestCreateReviewBody {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub event: Option<CreateReviewEvent>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub body: Option<String>,
}

#[cfg(feature = "client")]
pub mod client {
  use crate::octokit_request;
  use crate::request::{middleware::Unauthorized, proxy::RequestProxy};

  use super::PullRequestCreateReviewBody;

  pub fn create_review_request(
    owner: &str,
    repo: &str,
    pull_request: u64,
    payload: PullRequestCreateReviewBody,
  ) -> RequestProxy<(), Unauthorized> {
    let builder = octokit_request!(
      POST,
      "/repos/{}/{}/pulls/{}/reviews",
      owner,
      repo,
      pull_request
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

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_emptry_review() {
    let request = client::create_review_request(
      "DmitryDodzin",
      "mol",
      1,
      PullRequestCreateReviewBody {
        event: Some(CreateReviewEvent::Comment),
        ..Default::default()
      },
    )
    .build();

    assert!(request.is_ok());

    let request = request.unwrap();

    println!("{:#?}", request.into_parts());
  }
}
