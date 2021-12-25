use hyper::{client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

lazy_static! {
  static ref AUTHORIZATION_HEADER: &'static str = "Authorization";
  static ref USER_AGENT_HEADER: &'static str = "User-Agent";
  static ref USER_AGENT_VALUE: String = format!(
    "{}/{} (https://github.com/DmitryDodzin/mol)",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
}

#[derive(Debug)]
pub struct OAuth {
  pub access_token: String,
  pub scope: String,
  pub token_type: String,
}

pub struct Client {
  oauth: Option<OAuth>,
  hyper: HyperClient<HttpsConnector<HttpConnector>>,
}

impl Client {
  pub fn new() -> Self {
    let https = HttpsConnector::new();

    Client {
      oauth: None,
      hyper: HyperClient::builder().build::<_, hyper::Body>(https),
    }
  }

  pub fn with_oauth(oauth: OAuth) -> Self {
    Client {
      oauth: Some(oauth),
      ..Self::new()
    }
  }

  pub fn set_oauth(&mut self, oauth: OAuth) {
    self.oauth.replace(oauth);
  }

  pub async fn get<T>(&self, url: &str) -> anyhow::Result<T>
  where
    T: DeserializeOwned,
  {
    let mut builder = Request::builder()
      .method(Method::GET)
      .uri(url)
      .header(*USER_AGENT_HEADER, USER_AGENT_VALUE.clone());

    if let Some(oauth) = &self.oauth {
      builder = builder.header(
        *AUTHORIZATION_HEADER,
        format!("{} {}", oauth.token_type, oauth.access_token),
      );
    }

    let req = builder.body(Body::empty())?;

    let res = self.hyper.request(req).await?;
    let buf = hyper::body::to_bytes(res).await?;

    serde_json::from_slice(&buf).map_err(|err| err.into())
  }
}

impl Default for Client {
  fn default() -> Self {
    Self::new()
  }
}
