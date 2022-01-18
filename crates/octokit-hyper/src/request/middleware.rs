use http::request::Builder;
use lazy_static::lazy_static;

lazy_static! {
  static ref AUTHORIZATION_HEADER: &'static str = "Authorization";
}

#[derive(Debug, Clone, Default)]
pub struct OAuth {
  pub access_token: String,
  pub scope: String,
  pub token_type: String,
}

impl OAuth {
  pub fn from_env() -> Result<Self, std::env::VarError> {
    Ok(OAuth {
      access_token: std::env::var("GITHUB_TOKEN")?,
      token_type: "token".to_owned(),
      scope: "".to_owned(),
    })
  }
}

impl From<&OAuth> for OAuth {
  fn from(auth: &OAuth) -> Self {
    auth.clone()
  }
}

pub struct Unauthorized;
pub struct WithAuth {
  pub(crate) oauth: OAuth,
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
