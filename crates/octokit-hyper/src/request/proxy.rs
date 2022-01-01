use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::client::Client;

use super::builder::RequestBuilder;
use super::middleware::RequestMiddleware;

pub struct RequestProxy<T: DeserializeOwned, U>(
  RequestBuilder<U>,
  Box<dyn Fn() -> anyhow::Result<hyper::Body>>,
  PhantomData<T>,
);

impl<T, U> RequestProxy<T, U>
where
  T: DeserializeOwned,
  U: RequestMiddleware + Send + Sync,
{
  pub(crate) fn new(builder: RequestBuilder<U>) -> RequestProxy<T, U> {
    RequestProxy(builder, Box::new(|| Ok(hyper::Body::empty())), PhantomData)
  }

  pub(crate) fn with_body(
    builder: RequestBuilder<U>,
    creator: Box<dyn Fn() -> anyhow::Result<hyper::Body>>,
  ) -> RequestProxy<T, U> {
    RequestProxy(builder, creator, PhantomData)
  }

  pub fn map<K>(self, cb: &dyn Fn(RequestBuilder<U>) -> RequestBuilder<K>) -> RequestProxy<T, K> {
    RequestProxy(cb(self.0), self.1, self.2)
  }

  pub fn build(self) -> anyhow::Result<hyper::Request<hyper::Body>> {
    self.0.build(self.1()?).map_err(|err| err.into())
  }

  pub async fn send(self, client: &Client) -> anyhow::Result<T> {
    let request = self.build()?;

    client.send(request).await
  }
}

unsafe impl<T, U> Send for RequestProxy<T, U> where T: DeserializeOwned {}
unsafe impl<T, U> Sync for RequestProxy<T, U> where T: DeserializeOwned {}
