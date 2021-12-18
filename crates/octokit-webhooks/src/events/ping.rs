use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub struct PingEvent {
  pub zen: String,
  /// The ID of the webhook that triggered the ping.
  pub hook_id: u64,
  /// The [webhook configuration](https://docs.github.com/en/rest/reference/repos#get-a-repository-webhook).
  pub hook: Hook,
  pub repository: Option<Repository>,
  pub sender: Option<User>,
  pub organization: Option<Organization>,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com
  test_from_sample!(payload, PingEvent, "./sample/ping/payload.json");
  test_from_sample!(
    with_app_id,
    PingEvent,
    "./sample/ping/with-app_id.payload.json"
  );
  test_from_sample!(
    with_organization,
    PingEvent,
    "./sample/ping/with-organization.payload.json"
  );
}
