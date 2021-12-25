use std::convert::TryFrom;

use async_trait::async_trait;
use http::request::Builder;
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
  hyper: HyperClient<HttpsConnector<HttpConnector>>,
}

impl Client {
  pub fn new() -> Self {
    let https = HttpsConnector::new();

    Client {
      hyper: HyperClient::builder().build::<_, hyper::Body>(https),
    }
  }

  pub fn request(url: &str) -> RequestBuilder {
    RequestBuilder::new(url)
  }

  pub fn get(url: &str) -> RequestBuilder {
    Self::request(url).method(Method::GET)
  }

  pub async fn send<T>(&self, request: Request<Body>) -> anyhow::Result<T>
  where
    T: DeserializeOwned,
  {
    let res = self.hyper.request(request).await?;
    let buf = hyper::body::to_bytes(res).await?;

    serde_json::from_slice(&buf).map_err(|err| err.into())
  }
}

impl Default for Client {
  fn default() -> Self {
    Self::new()
  }
}

pub struct Unauthorized;
pub struct WithAuth {
  oauth: OAuth,
}

pub trait RequestMiddleware {
  fn handle(&self, builder: Builder) -> Builder {
    builder
  }
}

impl RequestMiddleware for Unauthorized {}
impl RequestMiddleware for WithAuth {
  fn handle(&self, builder: Builder) -> Builder {
    builder.header(
      *AUTHORIZATION_HEADER,
      format!("{} {}", self.oauth.token_type, self.oauth.access_token),
    )
  }
}

pub struct RequestBuilder<T = Unauthorized>(Builder, T);

impl RequestBuilder {
  pub fn new(url: &str) -> RequestBuilder<Unauthorized> {
    RequestBuilder(
      Builder::new()
        .uri(url)
        .header(*USER_AGENT_HEADER, USER_AGENT_VALUE.clone()),
      Unauthorized,
    )
  }

  pub fn with_auth(self, oauth: OAuth) -> RequestBuilder<WithAuth> {
    RequestBuilder(self.0, WithAuth { oauth })
  }
}

impl RequestBuilder<WithAuth> {
  pub fn oauth(&self) -> &'_ OAuth {
    &self.1.oauth
  }
}

impl<T> RequestBuilder<T> {
  pub fn method<U>(self, method: U) -> Self
  where
    Method: TryFrom<U>,
    <Method as TryFrom<U>>::Error: Into<http::Error>,
  {
    Self(self.0.method(method), self.1)
  }
}

impl<T> RequestBuilder<T>
where
  T: RequestMiddleware,
{
  pub fn build(self, body: Body) -> http::Result<Request<Body>> {
    self.1.handle(self.0).body(body)
  }
}

#[async_trait]
pub trait RequestExt {
  async fn send<T: DeserializeOwned>(self, client: &Client) -> anyhow::Result<T>;
}

#[async_trait]
impl RequestExt for Request<Body> {
  async fn send<T: DeserializeOwned>(self, client: &Client) -> anyhow::Result<T> {
    client.send(self).await
  }
}

#[async_trait]
impl<T> RequestExt for RequestBuilder<T>
where
  T: RequestMiddleware + Send + Sync,
{
  async fn send<D: DeserializeOwned>(self, client: &Client) -> anyhow::Result<D> {
    let request = self.build(Body::empty())?;
    client.send(request).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_empty_request() {
    let builder = RequestBuilder::new("https://carbon14.xyz");

    let request = builder.build(Body::empty());

    assert!(request.is_ok());
  }

  #[test]
  fn create_empty_request_with_auth() {
    let builder = RequestBuilder::new("https://carbon14.xyz").with_auth(OAuth {
      access_token: "foobar".to_owned(),
      token_type: "barer".to_owned(),
      scope: "".to_owned(),
    });

    assert_eq!(builder.oauth().access_token, "foobar");
    assert_eq!(builder.oauth().token_type, "barer");

    let request = builder.build(Body::empty());

    assert!(request.is_ok());
  }
}
