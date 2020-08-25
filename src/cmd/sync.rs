use crate::{config, diff};

use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;

pub fn sync() -> Result<()> {
  let config = config::get_config()?;
  for (remote, local) in config.dest.iter() {
    sync_info(&remote, &local)?;
  }

  Ok(())
}

fn sync_info<P: AsRef<Path>, Q: AsRef<Path>>(
  remote: P,
  local: Q,
) -> Result<(SystemTime, SystemTime, bool)> {
  let remote = remote.as_ref();
  let local = local.as_ref();

  let rot_config_dir = config::rot_config_dir();
  let remote_abs = &rot_config_dir.join(remote);

  if local.is_file() {
    return Ok((
      fs::metadata(&remote_abs)?.modified()?,
      fs::metadata(local)?.modified()?,
      diff::diff(remote_abs, local)?,
    ));
  }

  let walker = WalkDir::new(remote_abs).into_iter();
  for remote_file in walker {

    // TODO(enricozb): rewrite these two lines, i'm unsure how to satisfy
    // the borrow checker here. It looks like
    //
    //   let remote_file = remote_file?.path()
    //
    // doesn't work because remote_file? is freed at the end of that statement.
    let remote_file = remote_file?;
    let remote_file = remote_file.path();

    if remote_file.is_dir() {
      continue;
    }

    let local_file = local.join(remote_file.strip_prefix(&remote_abs)?);

    println!(
      "{} vs {}",
      remote_file.display(), local_file.display()
    );
  }

  Ok((SystemTime::now(), SystemTime::now(), true))
}
