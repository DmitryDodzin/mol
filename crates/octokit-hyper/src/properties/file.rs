use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilePatch {
  pub sha: String,
  pub filename: String,
  pub status: String, // TODO: Update to enum
  pub additions: u64,
  pub deletions: u64,
  pub changes: u64,
  pub blob_url: String,
  pub raw_url: String,
  pub contents_url: String,
  pub patch: String,
}
