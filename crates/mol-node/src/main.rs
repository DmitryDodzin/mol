mod npm;

use mol_core::semantic::Semantic;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  mol::exec::<npm::Npm, Semantic>().await
}
