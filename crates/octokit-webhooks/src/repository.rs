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
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    created,
    RepositoryEvent,
    "./sample/repository/created.payload.json"
  );
  test_from_sample!(
    created_with_installation,
    RepositoryEvent,
    "./sample/repository/created.with-installation.payload.json"
  );
  test_from_sample!(
    edited,
    RepositoryEvent,
    "./sample/repository/edited.payload.json"
  );
  test_from_sample!(
    edited_with_default_branch_edit,
    RepositoryEvent,
    "./sample/repository/edited.with-default_branch-edit.payload.json"
  );
  test_from_sample!(
    privatized,
    RepositoryEvent,
    "./sample/repository/privatized.payload.json"
  );
  test_from_sample!(
    privatized_with_organization,
    RepositoryEvent,
    "./sample/repository/privatized.with-organization.payload.json"
  );
  test_from_sample!(
    publicized,
    RepositoryEvent,
    "./sample/repository/publicized.payload.json"
  );
  test_from_sample!(
    publicized_with_organization,
    RepositoryEvent,
    "./sample/repository/publicized.with-organization.payload.json"
  );
  test_from_sample!(
    renamed,
    RepositoryEvent,
    "./sample/repository/renamed.payload.json"
  );
  test_from_sample!(
    transferred,
    RepositoryEvent,
    "./sample/repository/transferred.payload.json"
  );
  test_from_sample!(
    transferred_with_installation,
    RepositoryEvent,
    "./sample/repository/transferred.with-installation.payload.json"
  );
  test_from_sample!(
    transferred_with_organization,
    RepositoryEvent,
    "./sample/repository/transferred.with-organization.payload.json"
  );
}
