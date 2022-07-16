use serde::Deserialize;

use super::User;

#[derive(Debug, Deserialize)]
pub struct RepositoryLite {
  pub archive_url: String,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub deployments_url: String,
  pub description: Option<String>,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub hooks_url: String,
  pub html_url: String,
  /**
   * Unique identifier of the repository
   */
  pub id: u64,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub keys_url: String,
  pub labels_url: String,
  pub languages_url: String,
  pub merges_url: String,
  pub milestones_url: String,
  /**
   * The name of the repository.
   */
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub owner: User,
  /**
   * Whether the repository is private or public.
   */
  pub private: bool,
  pub pulls_url: String,
  pub releases_url: String,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub trees_url: String,
  pub url: String,
}
