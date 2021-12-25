use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::{App, CheckRunConclusion, CheckRunDeployment, CheckRunStatus, RunPullRequest};

#[derive(Debug, Deserialize)]
pub struct CheckSuite {
  /**
   * The id of the check suite that this check run is part of.
   */
  pub id: u64,
  pub node_id: Option<String>,
  pub head_branch: Option<String>,
  /**
   * The SHA of the head commit that is being checked.
   */
  pub head_sha: String,
  pub status: CheckRunStatus,
  pub conclusion: Option<CheckRunConclusion>,
  pub url: String,
  pub before: Option<String>,
  pub after: Option<String>,
  /**
   * An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty.
   */
  pub pull_requests: Vec<RunPullRequest>,
  pub deployment: Option<CheckRunDeployment>,
  pub app: App,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  // TODO: update to timestamp when github will return RFC3339 format
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
}
