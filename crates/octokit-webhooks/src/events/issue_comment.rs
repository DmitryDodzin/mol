use serde::Deserialize;

use octokit_hyper::properties::*;

use crate::util::WrappedSource;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentEvent {
  Created {
    issue: Issue,
    comment: IssueComment,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Deleted {
    issue: Issue,
    comment: IssueComment,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
  Edited {
    changes: IssueCommentEditedEventChanges,
    issue: Issue,
    comment: IssueComment,
    repository: Repository,
    sender: User,
    installation: Option<InstallationLite>,
    organization: Option<Organization>,
  },
}

#[derive(Debug, Deserialize)]
pub struct IssueCommentEditedEventChanges {
  pub body: Option<WrappedSource<String>>,
}

#[cfg(test)]
mod tests {
  use crate::test_from_sample;

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  test_from_sample!(
    created,
    IssueCommentEvent,
    "./sample/issue_comment/created.payload.json"
  );
  test_from_sample!(
    created_1,
    IssueCommentEvent,
    "./sample/issue_comment/created.1.payload.json"
  );
  test_from_sample!(
    created_with_installation,
    IssueCommentEvent,
    "./sample/issue_comment/created.with-installation.payload.json"
  );
  test_from_sample!(
    created_with_organization,
    IssueCommentEvent,
    "./sample/issue_comment/created.with-organization.payload.json"
  );
  test_from_sample!(
    deleted,
    IssueCommentEvent,
    "./sample/issue_comment/deleted.payload.json"
  );
  test_from_sample!(
    deleted_with_organization,
    IssueCommentEvent,
    "./sample/issue_comment/deleted.with-organization.payload.json"
  );
  test_from_sample!(
    edited,
    IssueCommentEvent,
    "./sample/issue_comment/edited.payload.json"
  );
  test_from_sample!(
    edited_with_organization,
    IssueCommentEvent,
    "./sample/issue_comment/edited.with-organization.payload.json"
  );
}
