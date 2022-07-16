use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{AuthorAssociation, Label, Milestone, PullRequestLinks, Team, User};
use super::{PullRequestActiveLockReason, PullRequestRef, PullRequestState, UserOrTeam};

#[derive(Debug, Deserialize)]
pub struct SimplePullRequest {
  pub url: String,
  pub id: u64,
  pub node_id: String,
  pub html_url: String,
  pub diff_url: String,
  pub patch_url: String,
  pub issue_url: String,
  pub number: u64,
  pub state: PullRequestState,
  pub locked: bool,
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
  pub draft: bool,
  pub commits_url: String,
  pub review_comments_url: String,
  pub review_comment_url: String,
  pub comments_url: String,
  pub statuses_url: String,
  pub head: PullRequestRef,
  pub base: PullRequestRef,
  pub _links: PullRequestLinks,
  pub author_association: AuthorAssociation,
  pub active_lock_reason: Option<PullRequestActiveLockReason>,
}
