use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum PullRequestEvent {
  Assigned {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    assignee: User,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  AutoMergeDisabled {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  AutoMergeEnabled {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Closed {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ConvertedToDraft {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Edited {
    /// The pull request number.
    number: u32,
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
    number: u32,
    pull_request: PullRequest,
    label: Label,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Locked {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Opened {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ReadyForReview {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Reopened {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  ReviewRequestRemoved {
    /// The pull request number.
    number: u32,
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
    number: u32,
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
    number: u32,
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
    number: u32,
    pull_request: PullRequest,
    assignee: User,
    repository: Repository,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Unlabeled {
    /// The pull request number.
    number: u32,
    pull_request: PullRequest,
    repository: Repository,
    label: Label,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
    sender: User,
  },
  Unlocked {
    /// The pull request number.
    number: u32,
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

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn assigned() {
    let raw = std::fs::read_to_string("./sample/pull_request/assigned.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn assigned_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/assigned.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn closed() {
    let raw = std::fs::read_to_string("./sample/pull_request/closed.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn closed_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/closed.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn converted_to_draft() {
    let raw = std::fs::read_to_string("./sample/pull_request/converted_to_draft.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn converted_to_draft_with_installation() {
    let raw = std::fs::read_to_string(
      "./sample/pull_request/converted_to_draft.with-installation.payload.json",
    )
    .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn converted_to_draft_with_organization() {
    let raw = std::fs::read_to_string(
      "./sample/pull_request/converted_to_draft.with-organization.payload.json",
    )
    .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn labeled() {
    let raw = std::fs::read_to_string("./sample/pull_request/labeled.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn labeled_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/labeled.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn locked() {
    let raw = std::fs::read_to_string("./sample/pull_request/locked.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn locked_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/locked.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn opened() {
    let raw = std::fs::read_to_string("./sample/pull_request/opened.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn opened_with_null_body() {
    let raw = std::fs::read_to_string("./sample/pull_request/opened.with-null-body.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn opened_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/opened.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn ready_for_review() {
    let raw = std::fs::read_to_string("./sample/pull_request/ready_for_review.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn ready_for_review_with_installation() {
    let raw = std::fs::read_to_string(
      "./sample/pull_request/ready_for_review.with-installation.payload.json",
    )
    .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn ready_for_review_with_organization() {
    let raw = std::fs::read_to_string(
      "./sample/pull_request/ready_for_review.with-organization.payload.json",
    )
    .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn reopened() {
    let raw = std::fs::read_to_string("./sample/pull_request/reopened.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn reopened_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/reopened.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn review_request_removed() {
    let raw = std::fs::read_to_string("./sample/pull_request/review_request_removed.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn review_requested() {
    let raw = std::fs::read_to_string("./sample/pull_request/review_requested.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn synchronize() {
    let raw = std::fs::read_to_string("./sample/pull_request/synchronize.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unassigned() {
    let raw = std::fs::read_to_string("./sample/pull_request/unassigned.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unassigned_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/unassigned.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unlabeled() {
    let raw = std::fs::read_to_string("./sample/pull_request/unlabeled.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unlabeled_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/unlabeled.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unlocked() {
    let raw = std::fs::read_to_string("./sample/pull_request/unlocked.payload.json")
      .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }

  #[test]
  fn unlocked_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/pull_request/unlocked.with-organization.payload.json")
        .expect("test case not found");

    let pull_request = serde_json::from_str::<PullRequestEvent>(&raw);

    println!("{:?}", pull_request);

    assert!(pull_request.is_ok());
  }
}
