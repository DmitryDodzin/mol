#[cfg(feature = "client")]
pub mod client {
  #[macro_export]
  macro_rules! octokit_request {
    ($metod:ident, $($arg:tt) *) => {
      $crate::request::builder::RequestBuilder::new(format!(
        "{}{}",
        *$crate::api::GITHUB_API, format!($($arg)*)
      ))
        .method(hyper::Method::$metod)
    };
  }
}
