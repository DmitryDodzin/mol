use mol_cargo::Cargo;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
  mol::exec::<Cargo>().await
}
