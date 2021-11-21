use std::fmt::Debug;
use std::path::Path;

// use tokio::fs;

use crate::bump::PackageBump;
use crate::version::Versioned;

pub struct Changelog;

impl Changelog {
  pub async fn update_changelog<T: AsRef<Path>, U: Debug + Versioned>(
    _changeset_path: T,
    _next_version: String,
    bump: &PackageBump<'_, U>,
  ) -> std::io::Result<()> {
    if let Some(changesets) = bump.changesets() {
      for changset in changesets {
        println!("{}", changset.to_string());
      }
    }

    // let changeset = fs::read_to_string(changeset_path).await?;
    // for line in changeset.split('\n').map(|line| line.trim()) {
    //   println!("{:?}", line);
    // }

    Ok(())
  }
}
