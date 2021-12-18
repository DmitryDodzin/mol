use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum BranchProtectionRuleEvent {
  Created {
    rule: BranchProtectionRule,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Deleted {
    rule: BranchProtectionRule,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Edited {
    rule: BranchProtectionRule,
    changes: BranchProtectionRuleEditedEventChange,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct BranchProtectionRuleEditedEventChange {
  pub authorized_actors_only: Option<WrappedSource<bool>>,
  pub authorized_actor_names: Option<WrappedSource<Vec<String>>>,
}

#[cfg(test)]
mod tests {

  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    created,
    BranchProtectionRuleEvent,
    "./sample/branch_protection_rule/created.payload.json"
  );
  test_from_sample!(
    deleted,
    BranchProtectionRuleEvent,
    "./sample/branch_protection_rule/deleted.payload.json"
  );
  test_from_sample!(
    edited,
    BranchProtectionRuleEvent,
    "./sample/branch_protection_rule/edited.payload.json"
  );
}
