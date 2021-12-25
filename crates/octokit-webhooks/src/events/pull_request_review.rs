use serde::Deserialize;

use octokit_hyper::properties::*;

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
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    dismissed,
    PullRequestReviewEvent,
    "./sample/pull_request_review/dismissed.payload.json"
  );
  test_from_sample!(
    submitted,
    PullRequestReviewEvent,
    "./sample/pull_request_review/submitted.payload.json"
  );
  test_from_sample!(
    submitted_with_organization,
    PullRequestReviewEvent,
    "./sample/pull_request_review/submitted.with-organization.payload.json"
  );
}
