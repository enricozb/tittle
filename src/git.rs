use crate::{config, util};

use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn timestamp<P: AsRef<Path>>(path: P) -> Result<u64> {
  use chrono::prelude::*;
  let output = Command::new("git")
    .arg("-C")
    .arg(config::rot_config_dir())
    .args(&["log", "--pretty=format:%cd", "-n", "1", "--date=iso", "--"])
    .arg(path.as_ref())
    .output()?;

  Ok(
    std::str::from_utf8(&output.stdout)?
      .trim()
      .parse::<DateTime<Utc>>()?
      .timestamp() as u64,
  )
}

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

    commit("initial commit")?;
  }

  Ok(())
}