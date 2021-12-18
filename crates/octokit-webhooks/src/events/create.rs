use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub struct CreateEvent {
  /// The [`git ref`](https://docs.github.com/en/rest/reference/git#get-a-reference) resource.
  pub r#ref: String,
  /// The type of Git ref object created in the repository. Can be either `branch` or `tag`.
  pub ref_type: CreateEventRefType,
  /// The name of the repository's default branch (usually `main`).
  pub master_branch: String,
  /// The repository's current description.
  pub description: Option<String>,
  /// The pusher type for the event. Can be either `user` or a deploy key.
  pub pusher_type: String,
  pub repository: Repository,
  pub sender: User,
  pub installation: Option<InstallationLite>,
  pub organization: Option<Organization>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateEventRefType {
  Tag,
  Branch,
}

#[cfg(test)]
mod tests {

  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(payload, CreateEvent, "./sample/create/payload.json");
  test_from_sample!(
    with_description,
    CreateEvent,
    "./sample/create/with-description.payload.json"
  );
  test_from_sample!(
    with_installation,
    CreateEvent,
    "./sample/create/with-installation.payload.json"
  );
  test_from_sample!(
    with_organization,
    CreateEvent,
    "./sample/create/with-organization.payload.json"
  );
}
