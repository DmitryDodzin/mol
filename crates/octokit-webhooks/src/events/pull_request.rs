use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum PullRequestEvent {
  Assigned {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    assignee: User,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  AutoMergeDisabled {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  AutoMergeEnabled {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Closed {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ConvertedToDraft {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Edited {
    /// The pull request number.
    number: u64,
    /// The changes to the comment if the action was `edited`.
    changes: PullRequestEditedEventChanges,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Labeled {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    label: Label,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Locked {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Opened {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ReadyForReview {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Reopened {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ReviewRequestRemoved {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    requested_reviewer: Option<User>,
    requested_team: Option<Team>,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ReviewRequested {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    requested_reviewer: Option<User>,
    requested_team: Option<Team>,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Synchronize {
    /// The pull request number.
    number: u64,
    before: String,
    after: String,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Unassigned {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    assignee: User,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Unlabeled {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    label: Label,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Unlocked {
    /// The pull request number.
    number: u64,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
}

#[derive(Debug, Deserialize)]
pub struct PullRequestEditedEventChanges {
  pub body: Option<WrappedSource<String>>,
  pub title: Option<WrappedSource<String>>,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    assigned,
    PullRequestEvent,
    "./sample/pull_request/assigned.payload.json"
  );
  test_from_sample!(
    assigned_with_organization,
    PullRequestEvent,
    "./sample/pull_request/assigned.with-organization.payload.json"
  );
  test_from_sample!(
    closed,
    PullRequestEvent,
    "./sample/pull_request/closed.payload.json"
  );
  test_from_sample!(
    closed_with_organization,
    PullRequestEvent,
    "./sample/pull_request/closed.with-organization.payload.json"
  );
  test_from_sample!(
    converted_to_draft,
    PullRequestEvent,
    "./sample/pull_request/converted_to_draft.payload.json"
  );
  test_from_sample!(
    converted_to_draft_with_installation,
    PullRequestEvent,
    "./sample/pull_request/converted_to_draft.with-installation.payload.json"
  );
  test_from_sample!(
    converted_to_draft_with_organization,
    PullRequestEvent,
    "./sample/pull_request/converted_to_draft.with-organization.payload.json"
  );
  test_from_sample!(
    labeled,
    PullRequestEvent,
    "./sample/pull_request/labeled.payload.json"
  );
  test_from_sample!(
    labeled_with_organization,
    PullRequestEvent,
    "./sample/pull_request/labeled.with-organization.payload.json"
  );
  test_from_sample!(
    locked,
    PullRequestEvent,
    "./sample/pull_request/locked.payload.json"
  );
  test_from_sample!(
    locked_with_organization,
    PullRequestEvent,
    "./sample/pull_request/locked.with-organization.payload.json"
  );
  test_from_sample!(
    opened,
    PullRequestEvent,
    "./sample/pull_request/opened.payload.json"
  );
  test_from_sample!(
    opened_with_null_body,
    PullRequestEvent,
    "./sample/pull_request/opened.with-null-body.json"
  );
  test_from_sample!(
    opened_with_organization,
    PullRequestEvent,
    "./sample/pull_request/opened.with-organization.payload.json"
  );
  test_from_sample!(
    ready_for_review,
    PullRequestEvent,
    "./sample/pull_request/ready_for_review.payload.json"
  );
  test_from_sample!(
    ready_for_review_with_installation,
    PullRequestEvent,
    "./sample/pull_request/ready_for_review.with-installation.payload.json"
  );
  test_from_sample!(
    ready_for_review_with_organization,
    PullRequestEvent,
    "./sample/pull_request/ready_for_review.with-organization.payload.json"
  );
  test_from_sample!(
    reopened,
    PullRequestEvent,
    "./sample/pull_request/reopened.payload.json"
  );
  test_from_sample!(
    reopened_with_organization,
    PullRequestEvent,
    "./sample/pull_request/reopened.with-organization.payload.json"
  );
  test_from_sample!(
    review_request_removed,
    PullRequestEvent,
    "./sample/pull_request/review_request_removed.payload.json"
  );
  test_from_sample!(
    review_requested,
    PullRequestEvent,
    "./sample/pull_request/review_requested.payload.json"
  );
  test_from_sample!(
    synchronize,
    PullRequestEvent,
    "./sample/pull_request/synchronize.payload.json"
  );
  test_from_sample!(
    unassigned,
    PullRequestEvent,
    "./sample/pull_request/unassigned.payload.json"
  );
  test_from_sample!(
    unassigned_with_organization,
    PullRequestEvent,
    "./sample/pull_request/unassigned.with-organization.payload.json"
  );
  test_from_sample!(
    unlabeled,
    PullRequestEvent,
    "./sample/pull_request/unlabeled.payload.json"
  );
  test_from_sample!(
    unlabeled_with_organization,
    PullRequestEvent,
    "./sample/pull_request/unlabeled.with-organization.payload.json"
  );
  test_from_sample!(
    unlocked,
    PullRequestEvent,
    "./sample/pull_request/unlocked.payload.json"
  );
  test_from_sample!(
    unlocked_with_organization,
    PullRequestEvent,
    "./sample/pull_request/unlocked.with-organization.payload.json"
  );
}
