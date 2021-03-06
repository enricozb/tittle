use crate::{config, err, util};

use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Returns the timestamp of the most recent commit modifying `path` in seconds.
pub fn timestamp<P: AsRef<Path>>(path: P) -> Result<u64> {
  use chrono::prelude::*;
  let output = Command::new("git")
    .arg("-C")
    .arg(config::tittle_config_dir())
    .args(&["log", "--pretty=format:%cd", "-n", "1", "--date=iso", "--"])
    .arg(path.as_ref())
    .output()?;

  Ok(
    String::from_utf8(output.stdout)?
      .trim()
      .parse::<DateTime<Utc>>()?
      .timestamp() as u64,
  )
}

fn has_remote() -> Result<bool> {
  Ok(
    Command::new("git")
      .arg("-C")
      .arg(config::tittle_config_dir())
      .args(&["remote", "-v"])
      .output()?
      .stdout
      .len()
      != 0,
  )
}

/// Sets the url as the upstream repository.
pub fn set_remote(url: &str) -> Result<()> {
  let status = Command::new("git")
    .arg("-C")
    .arg(config::tittle_config_dir())
    .args(&["remote", "add", "origin", url])
    .status()?;

  if status.success() {
    Ok(())
  } else {
    err::err("git error")
  }
}

/// Execute a git command.
fn git_cmd(cmd: &[&str]) -> Result<()> {
  if !has_remote()? {
    return err::err("Attempting git command without existing repo.");
  }

  let status = Command::new("git")
    .arg("-C")
    .arg(config::tittle_config_dir())
    .args(cmd)
    .status()?;

  if status.success() {
    Ok(())
  } else {
    err::err("git error")
  }
}

/// Clones an existing tittle directory
pub fn clone(url: &str) -> Result<()> {
  if config::tittle_config_dir().is_dir() {
    return err::err(
      "Can't clone remote dotfile repository if local \
      repository already exists. Delete ~/.tittle first.",
    );
  }

  let output = Command::new("git")
    .args(&["clone", url])
    .arg(config::tittle_config_dir())
    .output()?;

  if output.status.success() {
    Ok(())
  } else {
    util::error(String::from_utf8(output.stderr)?.trim());
    err::err("Couldn't clone repo.")
  }
}

/// Pull in any changes in the tittle Git repository.
pub fn pull() -> Result<()> {
  git_cmd(&["pull", "origin", "master"])
}

/// Push any changes in the tittle Git repository.
pub fn push() -> Result<()> {
  git_cmd(&["push", "-u", "origin", "master"])
}
/// Create a commit under `tittle_config_dir()` with the message `msg`.
pub fn commit(msg: &str) -> Result<()> {
  Command::new("git")
    .arg("-C")
    .arg(config::tittle_config_dir())
    .args(&["add", "."])
    .output()?;

  Command::new("git")
    .arg("-C")
    .arg(config::tittle_config_dir())
    .args(&["commit", "-m", &format!("{}: {}", util::machine_id()?, msg)])
    .output()?;

  Ok(())
}

/// Initializes a Git repository under the tittle config directory. This must be called
/// before any other functions from `git::*` are called.
pub fn init() -> Result<()> {
  if !config::tittle_config_dir().join(".git").exists() {
    let output = Command::new("git")
      .arg("-C")
      .arg(config::tittle_config_dir())
      .arg("init")
      .output()?;

    util::info(String::from_utf8(output.stdout)?.trim());

    commit("initial commit")?;
  }

  Ok(())
}
