use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{AuthorAssociation, Label, Link, Milestone, Repository, Team, User};

#[derive(Debug, Deserialize)]
pub struct PullRequest {
  pub url: String,
  pub id: u64,
  pub node_id: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  /// Number uniquely identifying the pull request within its repository.
  pub number: u64,
  /// State of this Pull Request. Either `open` or `closed`.
  pub state: PullRequestState,
  pub locked: bool,
  /// The title of the pull request.
  pub title: String,
  pub user: User,
  pub body: Option<String>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub closed_at: Option<DateTime<Utc>>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub merged_at: Option<DateTime<Utc>>,
  pub merge_commit_sha: Option<String>,
  pub assignee: Option<User>,
  pub assignees: Vec<User>,
  pub requested_reviewers: Vec<UserOrTeam>,
  pub requested_teams: Vec<Team>,
  pub labels: Vec<Label>,
  pub milestone: Option<Milestone>,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  pub head: PullRequestRef,
  pub base: PullRequestRef,
  pub _links: PullRequestLinks,
  pub author_association: AuthorAssociation,
  // pub auto_merge: null,
  pub active_lock_reason: Option<PullRequestActiveLockReason>,
  /// Indicates whether or not the pull request is a draft.
  pub draft: bool,
  pub merged: Option<bool>,
  pub mergeable: Option<bool>,
  pub rebaseable: Option<bool>,
  pub mergeable_state: String,
  pub merged_by: Option<User>,
  pub comments: u64,
  pub review_comments: u64,
  /// Indicates whether maintainers can modify the pull request.
  pub maintainer_can_modify: bool,
  pub commits: u64,
  pub additions: u64,
  pub deletions: u64,
  pub changed_files: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
  Open,
  Closed,
}

#[derive(Debug, Deserialize)]
pub enum PullRequestActiveLockReason {
  #[serde(rename = "resolved")]
  Resolved,
  #[serde(rename = "off-topic")]
  OffTopic,
  #[serde(rename = "too heated")]
  TooHeated,
  #[serde(rename = "spam")]
  Spam,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UserOrTeam {
  User(User),
  Team(Team),
}

#[derive(Debug, Deserialize)]
pub struct PullRequestRef {
  pub label: String,
  pub r#ref: String,
  pub sha: String,
  pub user: User,
  pub repo: Repository,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestLinks {
  #[serde(rename = "self")]
  pub _self: Link,
  pub html: Link,
  pub issue: Link,
  pub comments: Link,
  pub review_comments: Link,
  pub review_comment: Link,
  pub commits: Link,
  pub statuses: Link,
}
