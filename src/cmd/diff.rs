use crate::cmd::sync;
use crate::{config, util};

use anyhow::Result;

pub fn diff() -> Result<()> {
  let config = config::get_config()?;
  for (remote, local) in config.dest.iter() {
    let files = sync::remote_and_local_files(&remote, &local)?;

    for (remote_file, local_file) in files.iter() {
      match util::diff(remote_file, local_file)? {
        None => continue,
        Some(diff) => {
          println!(
            "\ndiff '{}' '{}'",
            remote_file.display(),
            local_file.display()
          );
          println!("{}", diff);
        }
      }
    }
  }

  Ok(())
}
