pub mod bump;
pub mod changelog;
pub mod changeset;
pub mod changesets;
pub mod error;
pub mod explorer;
pub mod package;
pub mod package_manager;
pub mod plugin;
pub mod prelude;
pub mod semantic;
pub mod version;

lazy_static::lazy_static! {
  pub static ref DEFAULT_PACKAGE_DIR: std::path::PathBuf = ".".into();
}
