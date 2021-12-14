use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashSet;
use globset::{Glob, GlobSetBuilder};
use hyper::{Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use tokio::{fs, process::Command};
use toml_edit::{value, Document};

use mol_core::prelude::*;

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
  #[serde(alias = "crate")]
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

#[derive(Default)]
pub struct Cargo;

impl Cargo {
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

  async fn load_document<T: AsRef<Path>>(crate_path: T) -> anyhow::Result<(PathBuf, Document)> {
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
    crate_path: T,
  ) -> anyhow::Result<Vec<Package<V>>> {
    let mut result = Vec::new();
    let (crate_path, document) = Self::load_document(crate_path).await?;

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

      result.extend(
        Explorer::check_read_dir::<Self, V>(
          Arc::new(DashSet::new()),
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
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
      .method(Method::GET)
      .uri(format!(
        "https://crates.io/api/v1/crates/{}/{}",
        package.name, package.version.value
      ))
      .header(
        hyper::header::USER_AGENT,
        format!(
          "mol-cargo/{} (https://github.com/DmitryDodzin/mol)",
          env!("CARGO_PKG_VERSION")
        ),
      )
      .body(hyper::Body::empty())?;

    let response = client.request(request).await?;

    let bytes = hyper::body::to_bytes(response.into_body()).await?;

    let crates_result = serde_json::from_slice::<CratesResult<CratesVersion>>(&bytes)?;

    match crates_result {
      CratesResult::Ok(val) => Ok(val.version.name == package.name),
      CratesResult::Err { errors } => {
        for error in errors {
          println!("crates-error:\n{}", error.detail);
        }
        Ok(false)
      }
    }
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
    let (crate_path, mut document) = Self::load_document(crate_path).await?;

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
    let (crate_path, mut document) = Self::load_document(crate_path).await?;

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
