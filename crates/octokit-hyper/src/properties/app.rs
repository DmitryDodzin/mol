use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parse_flexible_timestamp;

use super::User;

#[derive(Debug, Deserialize)]
pub struct App {
  /**
   * Unique identifier of the GitHub app
   */
  pub id: u64,
  /**
   * The slug name of the GitHub app
   */
  pub slug: Option<String>,
  pub node_id: String,
  pub owner: User,
  /**
   * The name of the GitHub app
   */
  pub name: String,
  pub description: Option<String>,
  pub external_url: String,
  pub html_url: String,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  /**
   * The set of permissions for the GitHub app
   */
  pub permissions: Option<AppPermissions>,
  /**
   * The list of events for the GitHub app
   */
  pub events: Option<Vec<String>>, // TODO:: change to WebhookEvents
}

#[derive(Debug, Deserialize)]
pub struct AppPermissions {
  pub actions: Option<AppPermission>,
  pub administration: Option<AppPermission>,
  pub checks: Option<AppPermission>,
  pub content_references: Option<AppPermission>,
  pub contents: Option<AppPermission>,
  pub deployments: Option<AppPermission>,
  pub discussions: Option<AppPermission>,
  pub emails: Option<AppPermission>,
  pub environments: Option<AppPermission>,
  pub issues: Option<AppPermission>,
  pub keys: Option<AppPermission>,
  pub members: Option<AppPermission>,
  pub metadata: Option<AppPermission>,
  pub organization_administration: Option<AppPermission>,
  pub organization_hooks: Option<AppPermission>,
  pub organization_packages: Option<AppPermission>,
  pub organization_plan: Option<AppPermission>,
  pub organization_projects: Option<AppPermission>,
  pub organization_secrets: Option<AppPermission>,
  pub organization_self_hosted_runners: Option<AppPermission>,
  pub organization_user_blocking: Option<AppPermission>,
  pub packages: Option<AppPermission>,
  pub pages: Option<AppPermission>,
  pub pull_requests: Option<AppPermission>,
  pub repository_hooks: Option<AppPermission>,
  pub repository_projects: Option<AppPermission>,
  pub secret_scanning_alerts: Option<AppPermission>,
  pub secrets: Option<AppPermission>,
  pub security_events: Option<AppPermission>,
  pub security_scanning_alert: Option<AppPermission>,
  pub single_file: Option<AppPermission>,
  pub statuses: Option<AppPermission>,
  pub team_discussions: Option<AppPermission>,
  pub vulnerability_alerts: Option<AppPermission>,
  pub workflows: Option<AppPermission>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppPermission {
  Read,
  Write,
}
