use serde::Deserialize;

use crate::properties::*;

#[derive(Debug, Deserialize)]
pub struct PushEvent {
  /// The full git ref that was pushed. Example: `refs/heads/main` or `refs/tags/v3.14.1`.
  pub r#ref: String,
  /// The SHA of the most recent commit on `ref` before the push.
  pub before: String,
  /// The SHA of the most recent commit on `ref` after the push.
  pub after: String,
  /// Whether this push created the `ref`.
  pub created: bool,
  /// Whether this push deleted the `ref`.
  pub deleted: bool,
  /// Whether this push was a force push of the `ref`.
  pub forced: bool,
  pub base_ref: Option<String>,
  /// URL that shows the changes in this `ref` update, from the `before` commit to the `after` commit. For a newly created `ref` that is directly based on the default branch, this is the comparison between the head of the default branch and the `after` commit. Otherwise, this shows all commits until the `after` commit.
  pub compare: String,
  /// An array of commit objects describing the pushed commits. (Pushed commits are all commits that are included in the `compare` between the `before` commit and the `after` commit.) The array includes a maximum of 20 commits. If necessary, you can use the [Commits API](https://docs.github.com/en/rest/reference/repos#commits) to fetch additional commits. This limit is applied to timeline events only and isn't applied to webhook deliveries.
  pub commits: Vec<Commit>,
  /// For pushes where `after` is or points to a commit object, an expanded representation of that commit. For pushes where `after` refers to an annotated tag object, an expanded representation of the commit pointed to by the annotated tag.
  pub head_commit: Option<Commit>,
  pub repository: Repository,
  pub pusher: Committer,
  pub sender: User,
  pub installation: Option<InstallationLite>,
  pub organization: Option<Organization>,
}

#[cfg(test)]
mod tests {

  use super::*;

  // TODO: fetch latest jsons from https://github.com/octokit/webhooks/tree/master/payload-examples/api.github.com

  #[test]
  fn payload() {
    let raw = std::fs::read_to_string("./sample/push/payload.json").expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn payload_1() {
    let raw = std::fs::read_to_string("./sample/push/1.payload.json").expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_installation() {
    let raw = std::fs::read_to_string("./sample/push/with-installation.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_new_branch() {
    let raw = std::fs::read_to_string("./sample/push/with-new-branch.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_no_username_committer() {
    let raw = std::fs::read_to_string("./sample/push/with-no-username-committer.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }

  #[test]
  fn with_organization() {
    let raw = std::fs::read_to_string("./sample/push/with-organization.payload.json")
      .expect("test case not found");

    let event = serde_json::from_str::<PushEvent>(&raw);

    if let Err(ref error) = event {
      println!("{:#?}", error);
    }

    assert!(event.is_ok());
  }
}
