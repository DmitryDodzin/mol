use std::convert::TryInto;

use async_trait::async_trait;
use ntex::{
  util::BytesMut,
  web::{self, error, HttpRequest},
};

use octokit_webhooks::*;

#[async_trait]
pub trait Octokit {
  fn on_event(&self, event: Events);
}

pub async fn octokit_route<T>(
  req: HttpRequest,
  mut body: web::types::Payload,
  octokit: web::types::Data<T>,
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

  let event = (action, &bytes as &[u8]).try_into().map_err(|_| ())?;

  octokit.on_event(event);

  Ok("Ok")
}
