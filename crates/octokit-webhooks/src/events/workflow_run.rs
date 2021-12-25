use serde::Deserialize;

use octokit_hyper::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum WorkflowRunEvent {
  Completed {
    organization: Option<Organization>,
    repository: Repository,
    sender: User,
    workflow: Workflow,
    workflow_run: WorkflowRun,
    installation: Option<InstallationLite>,
  },
  Requested {
    organization: Option<Organization>,
    repository: Repository,
    sender: User,
    workflow: Workflow,
    workflow_run: WorkflowRun,
    installation: Option<InstallationLite>,
  },
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    completed,
    WorkflowRunEvent,
    "./sample/workflow_run/completed.payload.json"
  );
  test_from_sample!(
    completed_with_pull_requests,
    WorkflowRunEvent,
    "./sample/workflow_run/completed.with-pull-requests.payload.json"
  );
  test_from_sample!(
    requested,
    WorkflowRunEvent,
    "./sample/workflow_run/requested.payload.json"
  );
  test_from_sample!(
    requested_with_conclusion,
    WorkflowRunEvent,
    "./sample/workflow_run/requested.with-conclusion.payload.json"
  );
}
