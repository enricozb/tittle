use crate::util;

use anyhow::Result;

use std::collections::HashMap;
use std::io::prelude::*;
use std::path::{self, Path};
use std::{env, fs};

use serde::{Deserialize, Serialize};

/// A JSON-serializable struct representing machine-specific overrides. This is used
/// to specify values of variables in templates for a specific machine for each
/// remote name that is tracked.
///
/// # Fields
///
/// * `dest` - A map of remote paths to local paths, overriding the default `dest` map.
/// * `vars` - A map from variable names to values, used for template rendering.
#[derive(Serialize, Deserialize)]
pub struct OverrideConfig {
  pub dest: HashMap<String, String>,
  pub vars: HashMap<String, String>,
}

/// A struct representing the JSON in `rot_config.json`.
///
/// # Fields
///
/// * `repo` - The url, if any, to the upstream repository of the dotfiles.
/// * `dest` - A map of remote paths to local paths, describing where each dotfile
///            or directory is stored on the local filesystem.
/// * `overrides` - A map from a machine-id to an OverrideConfig.
/// * `templates` - A map from local template paths to their location after rendering.
#[derive(Serialize, Deserialize)]
pub struct Config {
  pub repo: Option<String>,
  pub dest: HashMap<String, String>,
  pub overrides: HashMap<String, OverrideConfig>,
  pub templates: HashMap<String, String>,
}

/// Returns the path of the user's config directory.
pub fn user_config_dir() -> path::PathBuf {
  if let Ok(user_config_dir) = env::var("XDG_CONFIG_HOME") {
    return Path::new(&user_config_dir).to_path_buf();
  }

  Path::new(&env::var("HOME").unwrap()).join(".config")
}

/// Returns the path of the rot directory.
pub fn rot_config_dir() -> path::PathBuf {
  user_config_dir().join("rot")
}

/// Returns the path of the rot_cofig.json file.
fn rot_config_file() -> path::PathBuf {
  rot_config_dir().join("rot_config.json")
}

/// Create the directory under `rot_config_dir()` if it doesn't exist.
fn create_config_dir_if_not_exists() -> Result<()> {
  let rot_config_dir = rot_config_dir();
  if !rot_config_dir.exists() {
    fs::create_dir(&rot_config_dir)?;
  }
  Ok(())
}

/// Create the `rot_config.json` and initialize it if it doesn't exist.
fn create_config_if_not_exists() -> Result<()> {
  let config_file = rot_config_file();
  if !config_file.exists() {
    let config = Config {
      repo: None,
      dest: HashMap::new(),
      overrides: HashMap::new(),
      templates: HashMap::new(),
    };

    writeln!(
      fs::File::create(&config_file)?,
      "{}",
      serde_json::to_string_pretty(&config)?
    )?;

    util::info(format!("Created '{}'", config_file.display()));
  }

  Ok(())
}

/// Initializes rot config directory and file. This must be called before any other
/// functions from `config::*` are called.
pub fn init() -> Result<()> {
  create_config_dir_if_not_exists()?;
  create_config_if_not_exists()
}

/// Returns the Config struct representing `rot_config.json`.
pub fn get_config() -> Result<Config> {
  let config: Config =
    serde_json::from_str(&fs::read_to_string(rot_config_file()).unwrap())?;

  Ok(config)
}

/// Saves the config `config` to `rot_config.json`.
pub fn write_config(config: &Config) -> Result<()> {
  let config_file = rot_config_file();
  writeln!(
    fs::File::create(&config_file)?,
    "{}",
    serde_json::to_string_pretty(&config)?
  )?;

  Ok(())
}
