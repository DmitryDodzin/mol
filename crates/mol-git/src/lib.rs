use mol_core::{declare_plugins, prelude::*};

struct GitPlugin;

impl Plugin for GitPlugin {
  fn name(&self) -> &str {
    "Git Plugin"
  }

  fn pre_command(&self, command: &str, _context: &PluginContext) -> anyhow::Result<()> {
    println!("git plugin {:?}", command);

    Ok(())
  }

  fn post_command(&self, command: &str, _context: &PluginContext) -> anyhow::Result<()> {
    println!("git plugin {:?}", command);

    Ok(())
  }
}

declare_plugins![GitPlugin];
