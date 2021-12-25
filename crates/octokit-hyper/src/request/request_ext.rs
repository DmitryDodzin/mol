use async_trait::async_trait;
use hyper::{Body, Request};
use serde::de::DeserializeOwned;

use super::builder::RequestBuilder;
use super::middleware::RequestMiddleware;

use crate::client::Client;

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
