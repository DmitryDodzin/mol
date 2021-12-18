use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteEvent {
  Completed {
    check_suite: CheckSuite,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Requested {
    check_suite: CheckSuite,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Rerequested {
    check_suite: CheckSuite,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    completed,
    CheckSuiteEvent,
    "./sample/check_suite/completed.payload.json"
  );
  test_from_sample!(
    completed_1,
    CheckSuiteEvent,
    "./sample/check_suite/completed.1.payload.json"
  );
  test_from_sample!(
    completed_with_organization,
    CheckSuiteEvent,
    "./sample/check_suite/completed.with-organization.payload.json"
  );
  test_from_sample!(
    requested,
    CheckSuiteEvent,
    "./sample/check_suite/requested.payload.json"
  );
  test_from_sample!(
    requested_with_organization,
    CheckSuiteEvent,
    "./sample/check_suite/requested.with-organization.payload.json"
  );
  test_from_sample!(
    rerequested,
    CheckSuiteEvent,
    "./sample/check_suite/rerequested.payload.json"
  );
  test_from_sample!(
    rerequested_with_organization,
    CheckSuiteEvent,
    "./sample/check_suite/rerequested.with-organization.payload.json"
  );
}
