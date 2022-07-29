use std::convert::TryInto;

use async_trait::async_trait;
use hmac::{Hmac, Mac};
use ntex::{
  http::HttpMessage,
  util::BytesMut,
  web::{self, error, HttpRequest},
};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use octokit_webhooks::{util::UrlencodedWrapper, *};

#[async_trait]
pub trait Octokit {
  async fn on_event(&self, event: Events) -> anyhow::Result<()>;
}

pub fn octokit_route_validate_signature(secret: &[u8], body: &[u8], req: &HttpRequest) -> bool {
  let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("HMAC can take key of any size");

  mac.update(body);

  let signature = format!("sha256={:x}", mac.finalize().into_bytes());

  req
    .headers()
    .get("X-Hub-Signature-256")
    .and_then(|header| header.to_str().ok())
    .map(|value| signature.as_bytes().ct_eq(value.as_bytes()).unwrap_u8() == 1)
    .unwrap_or(false)
}

pub struct OctokitConfig {
  pub secret: String,
}

pub async fn octokit_route<T>(
  req: HttpRequest,
  mut body: web::types::Payload,
  octokit: web::types::State<T>,
  octokit_config: web::types::State<OctokitConfig>,
) -> Result<&'static str, error::InternalError<String>>
where
  T: Octokit,
{
  let action = req
    .headers()
    .get("X-Github-Event")
    .and_then(|val| val.to_str().ok())
    .and_then(|val| serde_plain::from_str::<WebhookEvents>(val).ok())
    .ok_or_else(|| error::ErrorBadRequest("invalid x-github-event".to_owned()))?;

  let mut bytes = BytesMut::new();
  while let Some(item) = ntex::util::stream_recv(&mut body).await {
    bytes.extend_from_slice(&item.map_err(|err| error::ErrorBadRequest(format!("{}", err)))?);
  }

  if !octokit_route_validate_signature(octokit_config.secret.as_bytes(), &bytes, &req) {
    return Err(error::ErrorUnauthorized("secret didn't match".to_owned()));
  }

  let event = match req.content_type() {
    "application/json" => (action, &bytes as &[u8]).try_into(),
    "application/x-www-form-urlencoded" => {
      let proxy = serde_urlencoded::from_bytes::<UrlencodedWrapper>(&bytes)
        .map_err(|err| error::ErrorBadRequest(format!("{}", err)))?;

      (action, proxy.payload.as_bytes()).try_into()
    }
    _ => unimplemented!(),
  }
  .map_err(|err| error::ErrorBadRequest(format!("{}", err)))?;

  if let Err(error) = octokit.on_event(event).await {
    println!("{:?}", error);
  }

  Ok("Ok")
}
