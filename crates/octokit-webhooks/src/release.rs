use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "lowercase")]
pub enum ReleaseEvent {
  Created {
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Deleted {
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Edited {
    changes: ReleaseEditedEventChanges,
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  PreReleased {
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Published {
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Released {
    release: Release,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct ReleaseEditedEventChanges {
  pub body: Option<WrappedSource<String>>,
  pub name: Option<WrappedSource<String>>,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn created() {
    let raw = std::fs::read_to_string("./sample/release/created.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn created_with_discussion_url() {
    let raw = std::fs::read_to_string("./sample/release/created.with-discussion-url.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn created_with_installation() {
    let raw = std::fs::read_to_string("./sample/release/created.with-installation.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn deleted() {
    let raw = std::fs::read_to_string("./sample/release/deleted.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn deleted_with_reactions() {
    let raw = std::fs::read_to_string("./sample/release/deleted.with-reactions.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn edited() {
    let raw =
      std::fs::read_to_string("./sample/release/edited.payload.json").expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn edited_with_reactions() {
    let raw = std::fs::read_to_string("./sample/release/edited.with-reactions.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn prereleased() {
    let raw = std::fs::read_to_string("./sample/release/prereleased.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn prereleased_with_discussion_url() {
    let raw =
      std::fs::read_to_string("./sample/release/prereleased.with-disussion-url.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn published() {
    let raw = std::fs::read_to_string("./sample/release/published.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn published_with_discussion_url() {
    let raw =
      std::fs::read_to_string("./sample/release/published.with-discussion-url.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn released() {
    let raw =
      std::fs::read_to_string("./sample/release/released.json").expect("test case not found");

    let event = serde_json::from_str::<ReleaseEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
