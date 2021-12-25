use hyper::{client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

lazy_static! {
  static ref USER_AGENT_HEADER: &'static str = "user-agent";
  static ref USER_AGENT_VALUE: String = format!(
    "{}/{} (https://github.com/DmitryDodzin/mol)",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
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

  pub async fn get<T>(&self, url: &str) -> anyhow::Result<T>
  where
    T: DeserializeOwned,
  {
    let req = Request::builder()
      .method(Method::GET)
      .uri(url)
      .header(*USER_AGENT_HEADER, USER_AGENT_VALUE.clone())
      .body(Body::empty())?;

    let res = self.hyper.request(req).await?;
    let buf = hyper::body::to_bytes(res).await?;

    serde_json::from_slice(&buf).map_err(|err| err.into())
  }
}
