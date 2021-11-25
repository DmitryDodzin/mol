use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

use itertools::Itertools;
use tokio::{fs, io::AsyncWriteExt};

use crate::bump::PackageBump;
use crate::changeset::Changeset;
use crate::semantic::Semantic;
use crate::version::{Version, Versioned};

fn capitalize(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().chain(c).collect(),
  }
}

fn fill_output<V: Versioned>(
  next_version: &str,
  patches: &HashMap<Version<V>, Vec<String>>,
) -> String {
  let mut output = String::new();

  output.push_str(&next_version.as_version().as_changelog_fmt());

  for (version, changes) in patches.iter().sorted_by(|(a, _), (b, _)| Ord::cmp(&b, &a)) {
    output.push('\n');
    output.push_str(&version.as_changelog_fmt());
    output.push('\n');

    output.push_str(&changes.join("\n"));
  }

  output
}

fn create_patches<V: Versioned>(
  package_name: &str,
  changesets: Vec<&Changeset<V>>,
) -> HashMap<Version<V>, Vec<String>> {
  let mut patches: HashMap<Version<V>, Vec<String>> = HashMap::new();

  for changset in changesets {
    let changeset_summary = changset.as_changelog_fmt();

    if let Some(version) = changset.packages.get(package_name) {
      if let Some(changes) = patches.get_mut(version) {
        changes.push(changeset_summary);
      } else {
        patches.insert(version.clone(), vec![changeset_summary]);
      }
    }
  }

  patches
}

pub struct Changelog;

impl Changelog {
  pub async fn update_changelog<T: AsRef<Path> + Debug, V: Versioned>(
    changelog_path: T,
    next_version: String,
    package_bump: &PackageBump<'_, V>,
    dry_run: bool,
  ) -> std::io::Result<()> {
    let package_name = package_bump.name();

    if let Some(patches) = package_bump
      .changesets()
      .map(|changesets| create_patches(package_name, changesets))
    {
      if dry_run {
        println!(
          "dry_run - update changelog {:?}\n{}",
          changelog_path,
          fill_output(&next_version, &patches)
            .split('\n')
            .map(|val| format!("dry_run: + {}", val))
            .join("\n")
        );
      } else {
        let changelog = fs::read_to_string(&changelog_path)
          .await
          .unwrap_or_else(|_| format!("# {}\n", package_name));

        let mut changelog_lines = changelog.split('\n');

        if let Some(title) = changelog_lines.next() {
          let mut output = String::new();

          output.push_str(title);
          output.push('\n');
          output.push('\n');

          output.push_str(&fill_output(&next_version, &patches));

          let mut changelog = fs::File::create(&changelog_path).await?;

          changelog.write(output.as_bytes()).await?;

          changelog
            .write(changelog_lines.join("\n").as_bytes())
            .await?;
        }
      }
    }

    Ok(())
  }
}

pub trait AsChangelogFmt {
  fn as_changelog_fmt(&self) -> String;
}

impl<T> AsChangelogFmt for Changeset<T> {
  fn as_changelog_fmt(&self) -> String {
    let mut changeset_summary = String::new();

    let mut parts = self.message.split('\n');

    if let Some(value) = parts.next() {
      changeset_summary.push_str("- ");
      changeset_summary.push_str(value);
      changeset_summary.push('\n');

      for part in parts {
        changeset_summary.push_str("  ");
        changeset_summary.push_str(part);
        changeset_summary.push('\n');
      }
    }

    changeset_summary
  }
}

impl AsChangelogFmt for Semantic {
  fn as_changelog_fmt(&self) -> String {
    capitalize(&self.to_string())
  }
}

impl<T: AsChangelogFmt> AsChangelogFmt for Version<T> {
  fn as_changelog_fmt(&self) -> String {
    format!("### {} Changes\n", self.version.as_changelog_fmt())
  }
}

struct ChangelogVersion(String);

impl AsChangelogFmt for ChangelogVersion {
  fn as_changelog_fmt(&self) -> String {
    format!("## {}\n", self.0)
  }
}

trait ChangelogStrFmt {
  fn as_version(&self) -> ChangelogVersion;
}

impl ChangelogStrFmt for &str {
  fn as_version(&self) -> ChangelogVersion {
    ChangelogVersion(self.to_string())
  }
}
