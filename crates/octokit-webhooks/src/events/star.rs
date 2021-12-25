use chrono::{DateTime, Utc};
use serde::Deserialize;

use octokit_hyper::{properties::*, util::parse_flexible_timestamp};

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
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(created, StarEvent, "./sample/star/created.payload.json");
  test_from_sample!(deleted, StarEvent, "./sample/star/deleted.payload.json");
}
