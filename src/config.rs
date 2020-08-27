use crate::util::{self, color};

use anyhow::Result;

use std::collections::HashMap;
use std::io::prelude::*;
use std::path::{self, Path};
use std::{env, fs};

use serde::{Deserialize, Serialize};

/// A JSON-serializable struct representing machine-specific overrides. This is used
/// to specify values of variables in templates for a specific machine for each
/// remote name that is tracked. Machines can also specify where each config should
/// end up.
///
/// # Fields
///
/// * `dest` - A map of remote paths to local paths, overriding the default `dest` map.
/// * `templates` - A map from remote template paths to their location after rendering,
///                 overriding the default `templates` map.
/// * `vars` - A map from variable names to values, used for template rendering.
#[derive(Serialize, Deserialize, Clone)]
pub struct OverrideConfig {
  dest: HashMap<String, String>,
  templates: HashMap<String, String>,
  vars: HashMap<String, String>,
}

/// A struct representing the JSON in `rot_config.json`.
///
/// # Fields
///
/// * `repo` - The url, if any, to the upstream repository of the dotfiles.
/// * `overrides` - A map from a machine-id to an OverrideConfig.
/// * `dest` - A map of remote paths to local paths, describing where each dotfile
///            or directory is stored on the local filesystem.
/// * `templates` - A map from remote template paths to their location after rendering.
#[derive(Serialize, Deserialize)]
pub struct Config {
  pub repo: Option<String>,
  dest: HashMap<String, String>,
  overrides: HashMap<String, OverrideConfig>,
  templates: HashMap<String, String>,
}

impl Config {
  pub fn dest<S: Into<String>>(&self, remote: S) -> String {
    let remote = remote.into();
    let default_local = &self.dest[&remote];
    match util::machine_id() {
      Err(_) => default_local.clone(),
      Ok(machine_id) => match self.overrides.get(&machine_id) {
        None => default_local.clone(),
        Some(override_config) => override_config
          .dest
          .get(&remote)
          .unwrap_or(default_local)
          .clone(),
      },
    }
  }

  pub fn dests(&self) -> HashMap<String, String> {
    match util::machine_id() {
      Err(_) => self.dest.clone(),
      Ok(machine_id) => match self.overrides.get(&machine_id) {
        None => self.dest.clone(),
        Some(override_config) => update_hash_map(&self.dest, &override_config.dest),
      },
    }
  }

  pub fn templates(&self) -> HashMap<String, String> {
    match util::machine_id() {
      Err(_) => self.templates.clone(),
      Ok(machine_id) => match self.overrides.get(&machine_id) {
        None => self.templates.clone(),
        Some(override_config) => {
          update_hash_map(&self.templates, &override_config.templates)
        }
      },
    }
  }

  pub fn track_template<R: Into<String>, S: Into<String>>(
    &mut self,
    name: R,
    render_to: S,
  ) {
    self.templates.insert(name.into(), render_to.into());
  }

  pub fn track<R: Into<String>, S: Into<String>>(&mut self, remote: R, local: S) {
    self.dest.insert(remote.into(), local.into());
  }

  pub fn has_remote<S: Into<String>>(&self, remote: S) -> bool {
    self.dest.contains_key(&remote.into())
  }

  pub fn my_overrides(&self) -> OverrideConfig {
    let empty = OverrideConfig {
      dest: HashMap::new(),
      templates: HashMap::new(),
      vars: HashMap::new(),
    };

    match util::machine_id() {
      Err(_) => empty.clone(),
      Ok(machine_id) => self.overrides.get(&machine_id).unwrap_or(&empty).clone(),
    }
  }

  pub fn set_my_overrides(&mut self, override_config: OverrideConfig) -> Result<()> {
    let machine_id = util::machine_id()?;
    self.overrides.insert(machine_id, override_config);
    Ok(())
  }

  pub fn vars(&self) -> HashMap<String, String> {
    self.my_overrides().vars
  }
}

fn update_hash_map(
  old: &HashMap<String, String>,
  new: &HashMap<String, String>,
) -> HashMap<String, String> {
  old
    .iter()
    .map(|(k, v)| (k.to_owned(), new.get(k).unwrap_or(v).to_owned()))
    .collect()
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
pub fn rot_config_file() -> path::PathBuf {
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

    util::info(format!("Created {}", color::path(config_file)));
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
