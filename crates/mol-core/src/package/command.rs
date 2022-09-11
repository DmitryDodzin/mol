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

  async fn execute(context: &CommandContext<'_, Metadata>) -> Result<Res, Self::Error>;
}

#[async_trait]
pub trait PackageManagerCommandWithArgs<Metadata: Send + Sync, Res = ()> {
  type Error: Error + Send + Sync + 'static;

  async fn execute_with_args(
    context: &CommandContext<'_, Metadata>,
    _: Vec<String>,
  ) -> Result<Res, Self::Error>;
}

#[async_trait]
impl<M, R, T> PackageManagerCommand<M, R> for T
where
  T: PackageManagerCommandWithArgs<M, R>,
  M: Send + Sync,
{
  type Error = T::Error;

  async fn execute(context: &CommandContext<'_, M>) -> Result<R, Self::Error> {
    T::execute_with_args(context, vec![]).await
  }
}

#[async_trait]
impl<M> PackageManagerCommandWithArgs<M> for ()
where
  M: Send + Sync,
{
  type Error = Infallible;

  async fn execute_with_args(_: &CommandContext<'_, M>, _: Vec<String>) -> Result<(), Self::Error> {
    Ok(())
  }
}
