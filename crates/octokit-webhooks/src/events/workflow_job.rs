use serde::Deserialize;

use octokit_hyper::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum WorkflowJobEvent {
  Completed {
    organization: Option<Organization>,
    installation: Option<InstallationLite>,
    repository: Repository,
    sender: User,
    workflow_job: WorkflowJob,
  },
  InProgress {
    organization: Option<Organization>,
    installation: Option<InstallationLite>,
    repository: Repository,
    sender: User,
    workflow_job: WorkflowJob,
  },
  Queued {
    organization: Option<Organization>,
    installation: Option<InstallationLite>,
    repository: Repository,
    sender: User,
    workflow_job: WorkflowJob,
  },
  Started {
    organization: Option<Organization>,
    installation: Option<InstallationLite>,
    repository: Repository,
    sender: User,
    workflow_job: WorkflowJob,
  },
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    completed_failure_with_organization,
    WorkflowJobEvent,
    "./sample/workflow_job/completed.failure.with-organization.payload.json"
  );
  test_from_sample!(
    completed_success_with_organization,
    WorkflowJobEvent,
    "./sample/workflow_job/completed.success.with-organization.payload.json"
  );
  test_from_sample!(
    in_progress,
    WorkflowJobEvent,
    "./sample/workflow_job/in_progress.payload.json"
  );
  test_from_sample!(
    queued,
    WorkflowJobEvent,
    "./sample/workflow_job/queued.payload.json"
  );
  test_from_sample!(
    started_with_organization,
    WorkflowJobEvent,
    "./sample/workflow_job/started.with-organization.payload.json"
  );
}
