pub mod compare;

#[cfg(feature = "client")]
lazy_static::lazy_static! {
  pub static ref GITHUB_API: String = std::env::var("GITHUB_API").unwrap_or_else(|_| "https://api.github.com".to_owned());
}
