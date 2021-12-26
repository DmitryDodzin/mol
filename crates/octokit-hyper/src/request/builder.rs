use std::convert::TryFrom;

use http::request::Builder;
use hyper::{Body, Method, Request};
use lazy_static::lazy_static;

use super::middleware::{OAuth, RequestMiddleware, Unauthorized, WithAuth};

lazy_static! {
  static ref USER_AGENT_HEADER: &'static str = "User-Agent";
  static ref USER_AGENT_VALUE: String = format!(
    "{}/{} (https://github.com/DmitryDodzin/mol)",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
  );
}

pub struct RequestBuilder<T = Unauthorized>(Builder, T);

impl RequestBuilder {
  pub fn new<T>(url: T) -> RequestBuilder<Unauthorized>
  where
    http::Uri: TryFrom<T>,
    <http::Uri as TryFrom<T>>::Error: Into<http::Error>,
  {
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
