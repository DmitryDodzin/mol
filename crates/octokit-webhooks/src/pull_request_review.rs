use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewEvent {
  Dismissed {
    review: Review,
    pull_request: SimplePullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Edited {
    changes: PullRequestReviewEditedEventChange,
    review: Review,
    pull_request: SimplePullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Submitted {
    review: Review,
    pull_request: SimplePullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
}

#[derive(Debug, Deserialize)]
pub struct PullRequestReviewEditedEventChange {
  pub body: Option<WrappedSource<String>>,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn dismissed() {
    let raw = std::fs::read_to_string("./sample/pull_request_review/dismissed.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PullRequestReviewEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn submitted() {
    let raw = std::fs::read_to_string("./sample/pull_request_review/submitted.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PullRequestReviewEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn submitted_with_organization() {
    let raw = std::fs::read_to_string(
      "./sample/pull_request_review/submitted.with-organization.payload.json",
    )
    .expect("test case not found");

    let event = serde_json::from_str::<PullRequestReviewEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
