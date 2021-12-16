use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::{parse_flexible_timestamp, parse_flexible_timestamp_option};

use super::{License, User};

#[derive(Debug, Deserialize)]
pub struct Repository {
  /// Unique identifier of the repository.
  pub id: u32,
  pub node_id: String,
  /// The name of the repository.
  pub name: String,
  pub full_name: String,
  /// Whether the repository is private or public.
  pub private: bool,

  pub owner: User,
  pub html_url: String,
  pub description: Option<String>,
  pub fork: bool,
  pub url: String,
  pub forks_url: String,
  pub keys_url: String,
  pub collaborators_url: String,
  pub teams_url: String,
  pub hooks_url: String,
  pub issue_events_url: String,
  pub events_url: String,
  pub assignees_url: String,
  pub branches_url: String,
  pub tags_url: String,
  pub blobs_url: String,
  pub git_tags_url: String,
  pub git_refs_url: String,
  pub trees_url: String,
  pub statuses_url: String,
  pub languages_url: String,
  pub stargazers_url: String,
  pub contributors_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub commits_url: String,
  pub git_commits_url: String,
  pub comments_url: String,
  pub issue_comment_url: String,
  pub contents_url: String,
  pub compare_url: String,
  pub merges_url: String,
  pub archive_url: String,
  pub downloads_url: String,
  pub issues_url: String,
  pub pulls_url: String,
  pub milestones_url: String,
  pub notifications_url: String,
  pub labels_url: String,
  pub releases_url: String,
  pub deployments_url: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp_option")]
  pub pushed_at: Option<DateTime<Utc>>,
  pub git_url: String,
  pub ssh_url: String,
  pub clone_url: String,
  pub svn_url: String,
  pub homepage: Option<String>,
  pub size: u32,
  pub stargazers_count: u32,
  pub watchers_count: u32,
  pub language: Option<String>,
  /// Whether issues are enabled.
  pub has_issues: bool,
  /// Whether projects are enabled.
  pub has_projects: bool,
  /// Whether downloads are enabled.
  pub has_downloads: bool,
  /// Whether the wiki is enabled.
  pub has_wiki: bool,
  pub has_pages: bool,
  pub forks_count: u32,
  pub mirror_url: Option<String>,
  /// Whether the repository is archived.
  pub archived: bool,
  /// Returns whether or not this repository is disabled.
  pub disabled: Option<bool>,
  pub open_issues_count: u32,
  pub license: Option<License>,
  pub forks: u32,
  pub open_issues: u32,
  pub watchers: u32,
  pub stargazers: Option<u32>,
  /// The default branch of the repository.
  pub default_branch: String,
  /// Whether to allow squash merges for pull requests.
  pub allow_squash_merge: Option<bool>,
  /// Whether to allow merge commits for pull requests.
  pub allow_merge_commit: Option<bool>,
  /// Whether to allow rebase merges for pull requests.
  pub allow_rebase_merge: Option<bool>,
  /// Whether to allow auto-merge for pull requests.
  pub allow_auto_merge: Option<bool>,
  /// Whether to allow private forks.
  pub allow_forking: Option<bool>,
  pub allow_update_branch: Option<bool>,
  pub is_template: bool,
  pub topics: Vec<String>,
  pub visibility: RepositoryVisibility,
  /// Whether to delete head branches when pull requests are merged
  pub delete_branch_on_merge: Option<bool>,
  pub master_branch: Option<String>,
  pub permissions: Option<RepositoryPermission>,
  pub public: Option<bool>,
  pub organization: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepositoryVisibility {
  Public,
  Private,
  Internal,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryPermission {
  pub pull: bool,
  pub push: bool,
  pub admin: bool,
  pub maintain: Option<bool>,
  pub triage: Option<bool>,
}
