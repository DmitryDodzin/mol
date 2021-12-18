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
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    created,
    ReleaseEvent,
    "./sample/release/created.payload.json"
  );
  test_from_sample!(
    created_with_discussion_url,
    ReleaseEvent,
    "./sample/release/created.with-discussion-url.payload.json"
  );
  test_from_sample!(
    created_with_installation,
    ReleaseEvent,
    "./sample/release/created.with-installation.payload.json"
  );
  test_from_sample!(
    deleted,
    ReleaseEvent,
    "./sample/release/deleted.payload.json"
  );
  test_from_sample!(
    deleted_with_reactions,
    ReleaseEvent,
    "./sample/release/deleted.with-reactions.payload.json"
  );
  test_from_sample!(edited, ReleaseEvent, "./sample/release/edited.payload.json");
  test_from_sample!(
    edited_with_reactions,
    ReleaseEvent,
    "./sample/release/edited.with-reactions.payload.json"
  );
  test_from_sample!(
    prereleased,
    ReleaseEvent,
    "./sample/release/prereleased.payload.json"
  );
  test_from_sample!(
    prereleased_with_discussion_url,
    ReleaseEvent,
    "./sample/release/prereleased.with-disussion-url.payload.json"
  );
  test_from_sample!(
    published,
    ReleaseEvent,
    "./sample/release/published.payload.json"
  );
  test_from_sample!(
    published_with_discussion_url,
    ReleaseEvent,
    "./sample/release/published.with-discussion-url.payload.json"
  );
  test_from_sample!(released, ReleaseEvent, "./sample/release/released.json");
}
