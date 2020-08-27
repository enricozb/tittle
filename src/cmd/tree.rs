use crate::config;

use anyhow::Result;
use std::process::Command;

/// Print a tree of all tracked files. Relies on the `tree` utility.
pub fn tree() -> Result<()> {
  Command::new("tree")
    .arg(config::tittle_config_dir())
    .arg("-aC")
    .arg("--noreport")
    .arg("-I")
    .arg(".git*|tittle_config.json")
    .status()?;

  Ok(())
}
