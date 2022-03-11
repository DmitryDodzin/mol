use hyper::{client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use crate::request::builder::RequestBuilder;

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
