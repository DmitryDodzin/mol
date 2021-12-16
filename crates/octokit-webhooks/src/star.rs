use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::properties::*;
use crate::util::parse_flexible_timestamp;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum StarEvent {
  Created {
    /// The time the star was created.
    #[serde(deserialize_with = "parse_flexible_timestamp")]
    starred_at: DateTime<Utc>,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Deleted {
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn created() {
    let raw =
      std::fs::read_to_string("./sample/star/created.payload.json").expect("test case not found");

    let event = serde_json::from_str::<StarEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }

  #[test]
  fn deleted() {
    let raw =
      std::fs::read_to_string("./sample/star/deleted.payload.json").expect("test case not found");

    let event = serde_json::from_str::<StarEvent>(&raw);

    println!("{:?}", event);

    assert!(event.is_ok());
  }
}
