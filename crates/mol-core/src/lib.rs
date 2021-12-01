pub mod bump;
pub mod changelog;
pub mod changeset;
pub mod changesets;
pub mod error;
pub mod package;
pub mod package_manager;
pub mod plugin;
pub mod prelude;
pub mod semantic;
pub mod version;

pub const CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
