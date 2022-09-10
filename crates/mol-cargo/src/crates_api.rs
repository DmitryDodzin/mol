use hyper::{Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

use mol_core::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CratesError {
  pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CratesVersion {
  pub version: CratesVersionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CratesVersionMetadata {
  #[serde(alias = "crate")]
  pub name: String,
  pub num: String,
  pub yanked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum CratesResult<T, E = CratesError> {
  Ok(T),
  Err { errors: Vec<E> },
}

pub(crate) async fn fetch_version<V: Versioned>(
  package: &Package<V>,
) -> anyhow::Result<CratesResult<CratesVersion>> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  let request = Request::builder()
    .method(Method::GET)
    .uri(format!(
      "https://crates.io/api/v1/crates/{}/{}",
      package.name, package.version.value
    ))
    .header(
      hyper::header::USER_AGENT,
      format!(
        "{}/{} (https://github.com/DmitryDodzin/mol)",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
      ),
    )
    .body(hyper::Body::empty())?;

  let response = client.request(request).await?;

  let bytes = hyper::body::to_bytes(response.into_body()).await?;

  serde_json::from_slice::<CratesResult<CratesVersion>>(&bytes).map_err(|err| err.into())
}
