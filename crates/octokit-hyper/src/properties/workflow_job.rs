use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{WorkflowConclusion, WorkflowStatus};

#[derive(Debug, Deserialize)]
pub struct WorkflowJob {
  pub id: u64,
  pub run_id: u64,
  pub run_attempt: u64,
  pub run_url: String,
  pub head_sha: String,
  pub node_id: String,
  pub name: String,
  pub check_run_url: String,
  pub html_url: String,
  pub url: String,
  /**
   * The current status of the job. Can be `queued`, `in_progress`, or `completed`.
   */
  pub status: WorkflowStatus,
  pub steps: Vec<WorkflowStep>,
  pub conclusion: Option<WorkflowConclusion>,
  /**
   * Custom labels for the job. Specified by the [`"runs-on"` attribute](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on) in the workflow YAML.
   */
  pub labels: Vec<String>,
  /**
   * The ID of the runner that is running this job. This will be `null` as long as `workflow_job[status]` is `queued`.
   */
  pub runner_id: Option<u64>,
  /**
   * The name of the runner that is running this job. This will be `null` as long as `workflow_job[status]` is `queued`.
   */
  pub runner_name: Option<String>,
  /**
   * The ID of the runner group that is running this job. This will be `null` as long as `workflow_job[status]` is `queued`.
   */
  pub runner_group_id: Option<u64>,
  /**
   * The name of the runner group that is running this job. This will be `null` as long as `workflow_job[status]` is `queued`.
   */
  pub runner_group_name: Option<String>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub started_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowStep {
  pub name: String,
  pub status: WorkflowStatus,
  pub conclusion: Option<WorkflowConclusion>,
  pub number: u64,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub started_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub completed_at: Option<DateTime<Utc>>,
}
