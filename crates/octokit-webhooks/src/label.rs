use serde::Deserialize;

use crate::properties::*;
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

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn created() {
    let raw =
      std::fs::read_to_string("./sample/label/created.payload.json").expect("test case not found");

    let event = serde_json::from_str::<LabelEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }

  #[test]
  fn created_with_installation() {
    let raw = std::fs::read_to_string("./sample/label/created.with-installation.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<LabelEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }

  #[test]
  fn deleted() {
    let raw =
      std::fs::read_to_string("./sample/label/deleted.payload.json").expect("test case not found");

    let event = serde_json::from_str::<LabelEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }

  #[test]
  fn edited() {
    let raw =
      std::fs::read_to_string("./sample/label/edited.payload.json").expect("test case not found");

    let event = serde_json::from_str::<LabelEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }
}
