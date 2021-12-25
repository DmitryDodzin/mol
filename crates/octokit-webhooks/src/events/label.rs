use serde::Deserialize;

use octokit_hyper::properties::*;

use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum LabelEvent {
  Created {
    label: Label,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Deleted {
    label: Label,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Edited {
    label: Label,
    changes: LabelEditedEventChange,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct LabelEditedEventChange {
  pub color: Option<WrappedSource<String>>,
  pub name: Option<WrappedSource<String>>,
  pub description: Option<WrappedSource<String>>,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(created, LabelEvent, "./sample/label/created.payload.json");
  test_from_sample!(
    created_with_installation,
    LabelEvent,
    "./sample/label/created.with-installation.payload.json"
  );
  test_from_sample!(deleted, LabelEvent, "./sample/label/deleted.payload.json");
  test_from_sample!(edited, LabelEvent, "./sample/label/edited.payload.json");
}
