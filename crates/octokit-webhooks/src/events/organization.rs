use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum OrganizationEvent {
  Deleted {
    membership: Option<Membership>,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Organization,
  },
  MemberAdded {
    membership: Membership,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Organization,
  },
  MemberInvited {
    invitation: MemberInvitation,
    user: User,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Organization,
  },
  MemberRemoved {
    membership: Membership,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Organization,
  },
  Renamed {
    membership: Membership,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Organization,
  },
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    member_added,
    OrganizationEvent,
    "./sample/organization/member_added.payload.json"
  );
  test_from_sample!(
    member_added_with_installation,
    OrganizationEvent,
    "./sample/organization/member_added.with-installation.payload.json"
  );
  test_from_sample!(
    member_invited,
    OrganizationEvent,
    "./sample/organization/member_invited.payload.json"
  );
}
