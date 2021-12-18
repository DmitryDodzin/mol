use serde::Deserialize;

use super::{App, CheckRunConclusion, CheckRunDeployment, CheckRunPullRequest, CheckRunStatus};

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
  pub pull_requests: Vec<CheckRunPullRequest>,
  pub deployment: Option<CheckRunDeployment>,
  pub app: App,
  // TODO: update to timestamp when github will return RFC3339 format
  pub created_at: String,
  // TODO: update to timestamp when github will return RFC3339 format
  pub updated_at: String,
}
