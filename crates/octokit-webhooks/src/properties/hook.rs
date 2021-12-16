use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{util::parse_flexible_timestamp, WebhookEvents};

#[derive(Debug, Deserialize)]
pub struct Hook {
  pub r#type: String,
  pub id: u32,
  pub name: String,
  pub active: bool,
  /// When you register a new GitHub App, GitHub sends a ping event to the **webhook URL** you specified during registration. The event contains the `app_id`, which is required for [authenticating](https://docs.github.com/en/apps/building-integrations/setting-up-and-registering-github-apps/about-authentication-options-for-github-apps) an app.
  pub app_id: Option<u32>,
  pub events: Vec<WebhookEvents>,
  pub config: HookConfig,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub updated_at: DateTime<Utc>,
  #[serde(deserialize_with = "parse_flexible_timestamp")]
  pub created_at: DateTime<Utc>,
  pub url: Option<String>,
  pub test_url: Option<String>,
  pub ping_url: Option<String>,
  pub deliveries_url: Option<String>,
  pub last_response: Option<HookLastResponse>,
}

#[derive(Debug, Deserialize)]
pub struct HookLastResponse {
  pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct HookConfig {
  pub content_type: HookConfigContentType,
  pub secret: Option<String>,
  pub url: String,
  pub insecure_ssl: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookConfigContentType {
  Json,
  Form,
}
