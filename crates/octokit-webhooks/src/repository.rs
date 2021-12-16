use serde::Deserialize;

use crate::properties::*;
use crate::util::WrappedSource;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum RepositoryEvent {
  Archived {
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Created {
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
  Edited {
    changes: RepositoryEditedEventChanges,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Privatized {
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Publicized {
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Renamed {
    changes: RepositoryRenamedEventChanges,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Transferred {
    changes: RepositoryTransferredEventChanges,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Unarchived {
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct RepositoryEditedEventChanges {
  description: Option<WrappedSource<Option<String>>>,
  default_branch: Option<WrappedSource<String>>,
  homepage: Option<WrappedSource<Option<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryRenamedEventChanges {
  name: Option<WrappedSource<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryTransferredEventUserWrapper {
  user: Option<User>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryTransferredEventChanges {
  owner: Option<WrappedSource<RepositoryTransferredEventUserWrapper>>,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn created() {
    let raw = std::fs::read_to_string("./sample/repository/created.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn created_with_installation() {
    let raw = std::fs::read_to_string("./sample/repository/created.with-installation.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn edited() {
    let raw = std::fs::read_to_string("./sample/repository/edited.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn edited_with_default_branch_edit() {
    let raw =
      std::fs::read_to_string("./sample/repository/edited.with-default_branch-edit.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn privatized() {
    let raw = std::fs::read_to_string("./sample/repository/privatized.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn privatized_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/repository/privatized.with-organization.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn publicized() {
    let raw = std::fs::read_to_string("./sample/repository/publicized.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn publicized_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/repository/publicized.with-organization.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn renamed() {
    let raw = std::fs::read_to_string("./sample/repository/renamed.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn transferred() {
    let raw = std::fs::read_to_string("./sample/repository/transferred.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn transferred_with_installation() {
    let raw =
      std::fs::read_to_string("./sample/repository/transferred.with-installation.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn transferred_with_organization() {
    let raw =
      std::fs::read_to_string("./sample/repository/transferred.with-organization.payload.json")
        .expect("test case not found");

    let event = serde_json::from_str::<RepositoryEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
