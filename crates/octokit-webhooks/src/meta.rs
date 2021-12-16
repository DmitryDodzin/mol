use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum MetaEvent {
  Deleted {
    hook_id: u64,
    /// The modified webhook. This will contain different keys based on the type of webhook it is: repository, organization, business, app, or GitHub Marketplace.
    hook: Hook,
    repository: Repository,
    sender: User,
  },
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(deleted, MetaEvent, "./sample/meta/deleted.payload.json");
}
