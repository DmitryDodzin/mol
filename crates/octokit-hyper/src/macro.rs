#[cfg(feature = "client")]
pub mod client {
  #[macro_export]
  macro_rules! octokit_builder {
    ($metod:ident, $path:literal, $($arg:tt) *) => {
      $crate::request::builder::RequestBuilder::new(format!(
        "{}{}",
        *$crate::api::GITHUB_API, format!($path, $($arg)*)
      ))
        .method(hyper::Method::$metod)
    };
  }

  #[macro_export]
  macro_rules! octokit_request {
    ($metod:ident, $path:literal, $($arg:tt) *) => {
      RequestProxy::new($crate::octokit_builder!($metod, $path, $($arg)*))
    };
  }
}
