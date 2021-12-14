use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_recursion::async_recursion;
use async_trait::async_trait;
use dashmap::DashSet;
use globset::{Glob, GlobSet, GlobSetBuilder};
use hyper::{Body, Client, Method, Request};
use serde::{Deserialize, Serialize};
use tokio::{fs, process::Command};
use toml_edit::{value, Document};

use mol_core::prelude::*;

fn remove_start_dot(dir: PathBuf) -> PathBuf {
  if dir.starts_with("./") {
    dir.iter().skip(1).collect()
  } else {
    dir
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct CratesError {
  detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CratesVersion {
  version: CratesVersionMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct CratesVersionMetadata {
  #[serde(rename(serialize = "crate"))]
  name: String,
  num: String,
  yanked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum CratesResult<T, E = CratesError> {
  Ok(T),
  Err { errors: Vec<E> },
}

impl<T, E> CratesResult<T, E> {
  fn is_ok(&self) -> bool {
    matches!(self, Self::Ok(_))
  }
}

#[derive(Default)]
pub struct Cargo;

impl Cargo {
  #[async_recursion]
  async fn check_dir<V: Versioned + Send + Sync + 'static>(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    entry: fs::DirEntry,
  ) -> anyhow::Result<Vec<Package<V>>> {
    let mut result = Vec::new();
    let entry_path = remove_start_dot(entry.path());

    if exists.contains(&entry_path) {
      return Ok(result);
    } else {
      exists.insert(entry_path.clone());
    }

    if entry_path.starts_with("target") || entry_path.starts_with(".git") {
      return Ok(result);
    }

    if let Ok(file_type) = entry.file_type().await {
      if file_type.is_dir() {
        return Cargo::check_read_dir(exists, globs, fs::read_dir(entry.path()).await?).await;
      }

      if file_type.is_symlink() {
        let link_value = fs::read_link(entry.path()).await?;
        return Cargo::check_read_dir(exists, globs, fs::read_dir(&link_value).await?).await;
      }

      if globs.is_match(entry_path) && file_type.is_file() && entry.file_name() == "Cargo.toml" {
        result.extend(Cargo.read_package(entry.path()).await?);
      }
    }

    Ok(result)
  }

  #[async_recursion]
  async fn check_read_dir<V: Versioned + Send + Sync + 'static>(
    exists: Arc<DashSet<PathBuf>>,
    globs: GlobSet,
    mut current_dir: fs::ReadDir,
  ) -> anyhow::Result<Vec<Package<V>>> {
    let mut handles = Vec::new();

    while let Some(entry) = current_dir.next_entry().await? {
      let globs = globs.clone();
      handles.push(tokio::spawn(Cargo::check_dir(exists.clone(), globs, entry)));
    }

    let mut result = Vec::new();

    for task in futures::future::join_all(handles)
      .await
      .into_iter()
      .flatten()
    {
      result.extend(task?);
    }

    Ok(result)
  }

  async fn run_command<T: AsRef<Path> + Send + Sync>(
    &self,
    command: &str,
    crate_path: T,
    args: Vec<&str>,
  ) -> anyhow::Result<()> {
    if let Ok(canon_path) = dunce::canonicalize(crate_path) {
      Command::new("cargo")
        .current_dir(canon_path)
        .arg(command)
        .args(args)
        .spawn()
        .expect("Cargo command failed to start")
        .wait()
        .await?;
    }

    Ok(())
  }

  async fn load_document<T: AsRef<Path>>(
    &self,
    crate_path: T,
  ) -> anyhow::Result<(PathBuf, Document)> {
    let document = fs::read_to_string(&crate_path)
      .await?
      .parse::<Document>()
      .expect("Invalid Cargo.toml");

    Ok((crate_path.as_ref().to_path_buf(), document))
  }
}

#[async_trait]
impl PackageManager for Cargo {
  fn default_path() -> &'static str {
    "Cargo.toml"
  }

  async fn read_package<T: AsRef<Path> + Send + Sync, V: Versioned + Send + Sync + 'static>(
    &self,
    crate_path: T,
  ) -> anyhow::Result<Vec<Package<V>>> {
    let mut result = Vec::new();
    let (crate_path, document) = self.load_document(crate_path).await?;

    let (package_name, version) = if document.contains_key("package") {
      (
        document["package"]["name"].as_str(),
        document["package"]["version"].as_str(),
      )
    } else {
      (None, None)
    };

    let mut dependencies = Vec::new();

    if document.contains_key("dependencies") {
      // TODO: support [dependencies.xyz] patterns
      if let Some(deps) = document["dependencies"].as_table() {
        for (key, value) in deps.iter() {
          if value.is_str() {
            dependencies.push((
              key.to_owned(),
              value.as_str().unwrap_or_default().to_owned(),
            ));
          } else if value.is_table() || value.is_inline_table() {
            dependencies.push((
              key.to_owned(),
              value["version"].as_str().unwrap_or_default().to_owned(),
            ));
          }
        }
      }
    }

    if let (Some(package_name), Some(version)) = (package_name, version) {
      result.push(Package {
        path: crate_path.clone(),
        name: package_name.to_owned(),
        version: version.into(),
        dependencies,
      });
    }

    let workspace = if document.contains_key("workspace") {
      document["workspace"]["members"].as_array().map(|val| {
        val
          .iter()
          .filter_map(|v| v.as_str())
          .filter_map(|glob| Glob::new(glob).ok())
          .collect::<Vec<Glob>>()
      })
    } else {
      None
    };

    if let Some(ref workspace) = workspace {
      let mut builder = GlobSetBuilder::new();

      for glob in workspace {
        builder.add(glob.clone());
      }

      let exists = Arc::new(DashSet::new());

      result.extend(
        Cargo::check_read_dir(
          exists,
          builder.build().expect("Globs did not set together"),
          fs::read_dir(crate_path.parent().unwrap_or(&crate_path)).await?,
        )
        .await?,
      );
    }

    Ok(result)
  }

  async fn check_version<V: Versioned + Send + Sync + 'static>(
    &self,
    package: &Package<V>,
  ) -> anyhow::Result<bool> {
    let client = Client::new();

    let request = Request::builder()
      .method(Method::GET)
      .uri(format!(
        "https://crates.io/api/v1/crates/{}/{}/",
        package.name, package.version.value
      ))
      .header(
        hyper::header::USER_AGENT,
        format!(
          "mol-cargo/{} (https://github.com/DmitryDodzin/mol)",
          env!("CARGO_PKG_VERSION")
        ),
      )
      .body(Body::empty())?;

    let response = client.request(request).await?;

    let bytes = hyper::body::to_bytes(response.into_body()).await?;

    Ok(serde_json::from_slice::<CratesResult<CratesVersion>>(&bytes)?.is_ok())
  }

  async fn run_build<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    build_args: Vec<String>,
  ) -> anyhow::Result<()> {
    self
      .run_command(
        "build",
        crate_path,
        build_args.iter().map(Deref::deref).collect(),
      )
      .await
  }

  async fn run_publish<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    publish_args: Vec<String>,
    dry_run: bool,
  ) -> anyhow::Result<()> {
    let args = if dry_run {
      vec!["--dry-run"]
        .into_iter()
        .chain(publish_args.iter().map(Deref::deref))
        .collect()
    } else {
      publish_args.iter().map(Deref::deref).collect()
    };

    self.run_command("publish", crate_path, args).await
  }

  async fn apply_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    version: &str,
  ) -> anyhow::Result<()> {
    let (crate_path, mut document) = self.load_document(crate_path).await?;

    if document.contains_key("package") {
      document["package"]["version"] = value(version);
    }

    fs::write(&crate_path, document.to_string()).await?;

    Ok(())
  }

  async fn apply_dependency_version<T: AsRef<Path> + Send + Sync>(
    &self,
    crate_path: T,
    name: &str,
    version: &str,
  ) -> anyhow::Result<()> {
    let (crate_path, mut document) = self.load_document(crate_path).await?;

    if document.contains_key("dependencies") {
      let dep = &document["dependencies"][name];

      if dep.is_inline_table() {
        document["dependencies"][name]["version"] = value(version);
      } else if dep.is_str() {
        document["dependencies"][name] = value(version);
      }
    }

    fs::write(&crate_path, document.to_string()).await?;

    Ok(())
  }
}
