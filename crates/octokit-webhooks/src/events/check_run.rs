use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum CheckRunEvent {
  Completed {
    check_run: CheckRun,
    requested_action: Option<CheckRunRequestedAction>,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Created {
    check_run: CheckRun,
    requested_action: Option<CheckRunRequestedAction>,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct CheckRunRequestedAction {
  pub identifier: Option<String>,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    completed,
    CheckRunEvent,
    "./sample/check_run/completed.payload.json"
  );
  test_from_sample!(
    completed_1,
    CheckRunEvent,
    "./sample/check_run/completed.1.payload.json"
  );
  test_from_sample!(
    created,
    CheckRunEvent,
    "./sample/check_run/created.payload.json"
  );
}
