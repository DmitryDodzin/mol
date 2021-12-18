use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{App, CheckRunDeployment, CheckRunPullRequest};

#[derive(Debug, Deserialize)]
pub struct CheckRun {
  /**
   * The id of the check.
   */
  pub id: u64,
  pub node_id: Option<String>,
  /**
   * The SHA of the commit that is being checked.
   */
  pub head_sha: String,
  pub external_id: String,
  pub url: String,
  pub html_url: String,
  pub details_url: Option<String>,
  /**
   * The current status of the check run. Can be `queued`, `in_progress`, or `completed`.
   */
  pub status: CheckRunStatus,
  /**
   * The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed.
   */
  pub conclusion: Option<CheckRunConclusion>,
  /**
   * The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
   */
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub started_at: DateTime<Utc>,
  /**
   * The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
   */
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub completed_at: Option<DateTime<Utc>>,
  pub output: CheckRunOutput,
  /**
   * The name of the check run.
   */
  pub name: String,
  pub check_suite: CheckRunCheckSuite,
  pub app: App,
  pub pull_requests: Vec<CheckRunPullRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CheckRunOutput {
  pub title: Option<String>,
  pub summary: Option<String>,
  pub text: Option<String>,
  pub annotations_count: u64,
  pub annotations_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckRunCheckSuite {
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
  pub pull_requests: Vec<CheckRunPullRequest>,
  pub deployment: Option<CheckRunDeployment>,
  pub app: App,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunConclusion {
  Success,
  Failure,
  Neutral,
  Cancelled,
  TimedOut,
  ActionRequired,
  Stale,
  Skipped,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckRunStatus {
  Queued,
  InProgress,
  Completed,
}
