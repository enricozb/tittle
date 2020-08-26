use crate::cmd::sync;
use crate::config;
use crate::util::{self, color};

use anyhow::Result;

/// Prints any diffs between any remote and local dotfiles.
pub fn diff() -> Result<()> {
  let config = config::get_config()?;
  let rot_config_dir = config::rot_config_dir();

  for (remote, local) in config.dests().iter() {
    let files = sync::remote_and_local_files(&remote, &local)?;

    for (remote_file, local_file) in files.iter() {
      match util::diff(remote_file, local_file)? {
        None => continue,
        Some(diff) => {
          util::info(format!(
            "diff {}\n{}\n",
            color::path(remote_file.strip_prefix(&rot_config_dir)?),
            diff
          ));
        }
      }
    }
  }

  Ok(())
}
