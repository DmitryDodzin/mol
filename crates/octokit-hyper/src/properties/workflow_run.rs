use serde::Deserialize;

use super::{RepositoryLite, RunPullRequest, SimpleCommit};

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
  pub artifacts_url: String,
  pub cancel_url: String,
  pub check_suite_url: String,
  pub check_suite_id: u64,
  pub check_suite_node_id: String,
  pub conclusion: Option<WorkflowRunConclusion>,
  pub created_at: String,
  pub event: String,
  pub head_branch: String,
  pub head_commit: SimpleCommit,
  pub head_repository: RepositoryLite,
  pub head_sha: String,
  pub html_url: String,
  pub id: u64,
  pub jobs_url: String,
  pub logs_url: String,
  pub node_id: String,
  pub name: String,
  pub pull_requests: Vec<RunPullRequest>,
  pub repository: RepositoryLite,
  pub rerun_url: String,
  pub run_number: u64,
  pub status: WorkflowRunStatus,
  pub updated_at: String,
  pub url: String,
  pub workflow_id: u64,
  pub workflow_url: String,
  pub run_attempt: u64,
  pub run_started_at: String,
  pub previous_attempt_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunConclusion {
  Success,
  Failure,
  Neutral,
  Cancelled,
  TimedOut,
  ActionRequired,
  Stale,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunStatus {
  Requested,
  InProgress,
  Completed,
  Queued,
}
