use std::convert::TryInto;

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use ntex::{
  util::BytesMut,
  web::{self, error, HttpRequest},
};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use octokit_webhooks::*;

#[async_trait]
pub trait Octokit {
  async fn on_event(&self, event: Events);
}

pub fn octokit_route_validate_signature(secret: &[u8], body: &[u8], req: &HttpRequest) -> bool {
  let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("HMAC can take key of any size");

  mac.update(body);

  let signature = format!("sha256={:x}", mac.finalize().into_bytes());

  req
    .headers()
    .get("X-Hub-Signature-256")
    .map(|header| header.to_str().ok())
    .flatten()
    .map(|value| signature.as_bytes().ct_eq(value.as_bytes()).unwrap_u8() == 1)
    .unwrap_or(false)
}

pub struct OctokitConfig {
  pub secret: String,
}

pub async fn octokit_route<T>(
  req: HttpRequest,
  mut body: web::types::Payload,
  octokit: web::types::Data<T>,
  octokit_config: web::types::Data<OctokitConfig>,
) -> Result<&'static str, error::PayloadError>
where
  T: Octokit,
{
  let action = req
    .headers()
    .get("X-Github-Event")
    .map(|val| val.to_str().ok())
    .flatten()
    .map(|val| serde_plain::from_str::<WebhookEvents>(val).ok())
    .flatten()
    .ok_or(())?;

  let mut bytes = BytesMut::new();
  while let Some(item) = ntex::util::next(&mut body).await {
    bytes.extend_from_slice(&item?);
  }

  if !octokit_route_validate_signature(octokit_config.secret.as_bytes(), &bytes, &req) {
    return Err(().into());
  }

  let event = (action, &bytes as &[u8]).try_into().map_err(|_| ())?;

  octokit.on_event(event).await;

  Ok("Ok")
}
