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

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn deleted() {
    let raw =
      std::fs::read_to_string("./sample/meta/deleted.payload.json").expect("test case not found");

    let event = serde_json::from_str::<MetaEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
