use mol_cargo::Cargo;
use mol_core::semantic::Semantic;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  mol::exec::<Cargo, Semantic>().await
}
