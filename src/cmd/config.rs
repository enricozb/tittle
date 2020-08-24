use std::{
  collections::HashMap,
  env, fs,
  io::{prelude::*, Result},
  path::{self, Path},
};

use serde::{Deserialize, Serialize};

use crate::util;

#[derive(Serialize, Deserialize)]
struct OverrideConfig {
  dest: HashMap<String, String>,
  vars: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
  dest: HashMap<String, String>,
  custom: HashMap<String, OverrideConfig>,
}

fn config_dir() -> path::PathBuf {
  if let Ok(config_dir) = env::var("XDG_CONFIG_HOME") {
    return Path::new(&config_dir).join("rot");
  }

  Path::new(&env::var("HOME").unwrap())
    .join(".config")
    .join("rot")
}

fn config_json() -> path::PathBuf {
  config_dir().join("rot_config.json")
}

fn create_config_dir_if_not_exists() -> Result<()> {
  let config_dir = config_dir();
  if !config_dir.exists() {
    fs::create_dir(&config_dir)?;
  }
  Ok(())
}

fn create_config_if_not_exists() -> Result<()> {
  let config_json = config_json();
  if !config_json.exists() {
    let config = Config {
      dest: HashMap::new(),
      custom: HashMap::new(),
    };

    writeln!(
      fs::File::create(&config_json)?,
      "{}",
      serde_json::to_string(&config)?
    )?;

    util::info(format!("Created '{}'", config_json.display()));
  }

  Ok(())
}

pub fn init() -> Result<()> {
  create_config_dir_if_not_exists()?;
  create_config_if_not_exists()
}
