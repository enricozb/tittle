use crate::util::{self, color};
use crate::{config, err};

use anyhow::Result;
use std::env;
use std::fs::File;
use std::process::Command;

/// Edit portions of the `rot_config.json`, depending on `mode`.
///
/// # Arguments
///
/// * `mode` - Optional specifier of which portion of the config should be edited.
///            Valid values are:
///              - `"me"`: edit this machine's specific overrides.
pub fn edit(mode: Option<&str>) -> Result<()> {
  match mode {
    None => {
      Command::new(editor()?)
        .arg(config::rot_config_file())
        .status()?;
    }
    Some("me") => {
      edit_machine()?;
    }
    Some(mode) => {
      return err::err(format!("Invalid edit mode {}", color::emphasis(mode)))
    }
  };

  Ok(())
}

/// Edit this machine's specific overrides.
fn edit_machine() -> Result<()> {
  use std::io::prelude::*;

  let machine_id = util::machine_id()?;
  let mut temp_override_path = env::temp_dir();
  temp_override_path.push(format!("overrides-{}", machine_id));

  let mut config = config::get_config()?;
  let old_overrides = serde_json::to_string_pretty(&config.my_overrides())?;
  let mut temp_override_file = File::create(&temp_override_path)?;
  temp_override_file.write_all(&old_overrides.as_bytes())?;
  temp_override_file.flush()?;

  Command::new(editor()?).arg(&temp_override_path).status()?;

  let mut new_overrides = String::new();
  File::open(temp_override_path)?.read_to_string(&mut new_overrides)?;

  config.set_my_overrides(serde_json::from_str(&new_overrides)?)?;

  config::write_config(&config)
}

/// Returns this machine's `$EDITOR`.
fn editor() -> Result<String> {
  match env::var("EDITOR") {
    Ok(editor) => Ok(editor),
    Err(_) => return err::err("Please set an $EDITOR to edit the rot config."),
  }
}
