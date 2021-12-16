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

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn created() {
    let raw = std::fs::read_to_string("./sample/branch_protection_rule/created.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<BranchProtectionRuleEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn deleted() {
    let raw = std::fs::read_to_string("./sample/branch_protection_rule/deleted.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<BranchProtectionRuleEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn edited() {
    let raw = std::fs::read_to_string("./sample/branch_protection_rule/edited.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<BranchProtectionRuleEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
