use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub struct PingEvent {
  pub zen: String,
  /// The ID of the webhook that triggered the ping.
  pub hook_id: u32,
  /// The [webhook configuration](https://docs.github.com/en/rest/reference/repos#get-a-repository-webhook).
  pub hook: Hook,
  pub repository: Option<Repository>,
  pub sender: Option<User>,
  pub organization: Option<Organization>,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn payload() {
    let raw = std::fs::read_to_string("./sample/ping/payload.json").expect("test case not found");

    let event = serde_json::from_str::<PingEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_app_id() {
    let raw = std::fs::read_to_string("./sample/ping/with-app_id.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PingEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_organization() {
    let raw = std::fs::read_to_string("./sample/ping/with-organization.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PingEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
