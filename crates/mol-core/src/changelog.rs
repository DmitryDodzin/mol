use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;

use itertools::Itertools;
use tokio::{fs, io::AsyncWriteExt};

use crate::bump::PackageBump;
use crate::version::{Version, Versioned};

pub struct Changelog;

impl Changelog {
  pub async fn update_changelog<T: AsRef<Path> + Debug, U: Versioned + Hash>(
    changelog_path: T,
    next_version: String,
    package_bump: &PackageBump<'_, U>,
    dry_run: bool,
  ) -> std::io::Result<()> {
    let package_name = package_bump.name();

    // TODO: move to validate
    if !changelog_path.as_ref().exists() {
      fs::write(
        &changelog_path,
        &format!("# {}\n", package_bump.name()).into_bytes(),
      )
      .await?;
    }

    if let Some(changesets) = package_bump.changesets() {
      let mut patches: HashMap<Version<U>, Vec<String>> = HashMap::new();

      for changset in changesets {
        let mut changeset_summary = String::new();

        let mut parts = changset.message.split('\n');

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

        if let Some(version) = changset.packages.get(package_name) {
          if let Some(changes) = patches.get_mut(version) {
            changes.push(changeset_summary);
          } else {
            patches.insert(version.clone(), vec![changeset_summary]);
          }
        }
      }

      if dry_run {
        println!("dry_run - update changelog {:?}", changelog_path);
      }

      let changelog = fs::read_to_string(&changelog_path).await?;
      let mut changelog_lines = changelog.split('\n');

      if let Some(title) = changelog_lines.next() {
        let mut output = String::new();

        if !dry_run {
          output.push_str(title);
          output.push('\n');
          output.push('\n');
        }

        output.push_str("## ");
        output.push_str(&next_version);
        output.push('\n');

        for (version, changes) in patches.iter().sorted_by(|(a, _), (b, _)| Ord::cmp(&b, &a)) {
          output.push_str(&version.as_changelog_fmt());
          output.push('\n');

          output.push_str(&changes.join("\n"));
        }

        if dry_run {
          println!(
            "{}",
            output
              .split('\n')
              .map(|val| format!("dry_run: + {}", val))
              .join("\n")
          );
        } else {
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
