use mol_cargo::Cargo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  mol::exec::<Cargo>().await
}
