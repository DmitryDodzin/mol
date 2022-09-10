use std::path::PathBuf;

use git2::Repository;

use mol_core::{declare_plugins, prelude::*};

struct GitPlugin {
  repo: Option<Repository>,
}

impl GitPlugin {
  fn load_changed_files(&self) -> Result<Vec<PathBuf>, git2::Error> {
    let repo = self.repo.as_ref().expect("should be initiated on_load");

    repo.statuses(None).map(|statuses| {
      statuses
        .iter()
        .filter(|status| !status.status().is_ignored())
        .filter_map(|status| status.path().and_then(|val| val.parse::<PathBuf>().ok()))
        .collect()
    })
  }
}

impl Plugin for GitPlugin {
  fn name(&self) -> &str {
    "Git Plugin"
  }

  fn on_load(&mut self, context: &PluginContext) -> anyhow::Result<()> {
    self.repo = Some(Repository::open(context.root_dir)?);

    Ok(())
  }

  fn pre_command(&self, command: &str, _context: &PluginContext) -> anyhow::Result<()> {
    match command {
      "version" | "publish" => {
        let changes = self.load_changed_files()?;

        if !changes.is_empty() {
          return Err(anyhow::Error::msg(format!(
            "found changed files can't run {:?}",
            command
          )));
        }
      }
      _ => {}
    }

    Ok(())
  }

  fn post_command(&self, _command: &str, context: &PluginContext) -> anyhow::Result<()> {
    if context.dry_run {
      return Ok(());
    }

    Ok(())
  }

  fn on_unload(&mut self) {
    self.repo = None;
  }
}

declare_plugins![GitPlugin { repo: None }];
