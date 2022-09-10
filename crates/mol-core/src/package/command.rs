use std::convert::Infallible;
use std::error::Error;
use std::path::Path;

use async_trait::async_trait;

pub struct CommandContext<'a, Metadata> {
  pub dry_run: bool,
  pub metadata: &'a Metadata,
  pub path: &'a Path,
}

#[async_trait]
pub trait PackageManagerCommand<Metadata: Send + Sync, Res = ()> {
  type Error: Error + Send + Sync + 'static;

  async fn execute(context: &CommandContext<'_, Metadata>) -> Result<Res, Self::Error> {
    Self::execute_with_args(context, vec![]).await
  }

  async fn execute_with_args(
    context: &CommandContext<'_, Metadata>,
    _: Vec<String>,
  ) -> Result<Res, Self::Error> {
    Self::execute(context).await
  }
}

#[async_trait]
impl<M> PackageManagerCommand<M> for ()
where
  M: Send + Sync,
{
  type Error = Infallible;

  async fn execute(_: &CommandContext<'_, M>) -> Result<(), Self::Error> {
    Ok(())
  }
}
