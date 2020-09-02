use crate::util::{self, color};
use crate::{cmd::edit, config, err};

use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::process::Command;

pub fn remove() -> Result<()> {
  use std::io::prelude::*;

  let machine_id = util::machine_id()?;
  let mut temp_dests_path = env::temp_dir();
  temp_dests_path.push(format!("remove-{}", machine_id));

  let config = config::get_config()?;
  let old_dests = config.dests();

  let mut temp_dests_file = File::create(&temp_dests_path)?;
  temp_dests_file
    .write_all(&serde_json::to_string_pretty(&config.dests())?.as_bytes())?;
  temp_dests_file.flush()?;

  Command::new(edit::editor()?)
    .arg(&temp_dests_path)
    .status()?;

  let mut remaining_dests = String::new();
  File::open(temp_dests_path)?.read_to_string(&mut remaining_dests)?;
  let remaining_dests: HashMap<String, String> = serde_json::from_str(&remaining_dests)?;

  for (key, _) in remaining_dests.iter() {
    if !old_dests.contains_key(key) {
      return err::err(format!(
        "Found new key '{}' while removing, use '{}' to add an entry instead.",
        color::emphasis(key),
        color::emphasis("tittle track"),
      ));
    }
  }

  let mut removed_keys = Vec::new();

  for (key, _) in old_dests.iter() {
    if !remaining_dests.contains_key(key) {
      removed_keys.push(key);
      util::info(format!("Removing entry '{}'", color::emphasis(key)));
    }
  }

  // remove these keys from default and override dest/templates.

  Ok(())
}
