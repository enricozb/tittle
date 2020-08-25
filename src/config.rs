use anyhow::Result;

use std::{
  collections::HashMap,
  env, fs,
  io::prelude::*,
  path::{self, Path},
};

use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Serialize, Deserialize)]
pub struct OverrideConfig {
  pub dest: HashMap<String, String>,
  pub vars: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub repo: Option<String>,
  pub dest: HashMap<String, String>,
  pub overrides: HashMap<String, OverrideConfig>,
}

pub fn user_config_dir() -> path::PathBuf {
  if let Ok(user_config_dir) = env::var("XDG_CONFIG_HOME") {
    return Path::new(&user_config_dir).to_path_buf();
  }

  Path::new(&env::var("HOME").unwrap()).join(".config")
}

pub fn rot_config_dir() -> path::PathBuf {
  user_config_dir().join("rot")
}

fn rot_config_file() -> path::PathBuf {
  rot_config_dir().join("rot_config.json")
}

fn create_config_dir_if_not_exists() -> Result<()> {
  let rot_config_dir = rot_config_dir();
  if !rot_config_dir.exists() {
    fs::create_dir(&rot_config_dir)?;
  }
  Ok(())
}

fn create_config_if_not_exists() -> Result<()> {
  let config_file = rot_config_file();
  if !config_file.exists() {
    let config = Config {
      repo: None,
      dest: HashMap::new(),
      overrides: HashMap::new(),
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

/// Initializes rot config directory and file.
/// Must be called before any other config function is used.
pub fn init() -> Result<()> {
  create_config_dir_if_not_exists()?;
  create_config_if_not_exists()
}

pub fn get_config() -> Result<Config> {
  let config: Config =
    serde_json::from_str(&fs::read_to_string(rot_config_file()).unwrap())?;

  Ok(config)
}

pub fn write_config(config: &Config) -> Result<()> {
  let config_file = rot_config_file();
  writeln!(
    fs::File::create(&config_file)?,
    "{}",
    serde_json::to_string_pretty(&config)?
  )?;

  Ok(())
}
