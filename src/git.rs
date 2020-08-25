use crate::{config, util};

use anyhow::Result;
use std::process::Command;

pub fn commit(msg: &str) -> Result<()> {
  Command::new("git")
    .arg("-C")
    .arg(config::rot_config_dir())
    .args(&["add", "."])
    .output()?;

  Command::new("git")
    .arg("-C")
    .arg(config::rot_config_dir())
    .args(&["commit", "-m", msg])
    .output()?;

  Ok(())
}

pub fn init() -> Result<()> {
  if !config::rot_config_dir().join(".git").exists() {
    let output = Command::new("git")
      .arg("-C")
      .arg(config::rot_config_dir())
      .arg("init")
      .output()?;

    util::info(std::str::from_utf8(&output.stdout)?.trim());
  }

  Ok(())
}
