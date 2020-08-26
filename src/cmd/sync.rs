use crate::{config, git, util};

use anyhow::Result;
use std::cmp::max;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(PartialEq)]
enum SyncDirection {
  FromRemote,
  ToRemote,
  NoDiff,
}

pub fn sync() -> Result<()> {
  use SyncDirection::*;

  let config = config::get_config()?;
  for (remote, local) in config.dest.iter() {
    let files = remote_and_local_files(&remote, &local)?;
    match sync_direction(&files) {
      NoDiff => continue,
      direction => {
        for (remote_file, local_file) in files.iter() {
          let arrow_str = if direction == FromRemote {
            fs::copy(remote_file, local_file)?;
            "->"
          } else {
            fs::copy(local_file, remote_file)?;
            "<-"
          };

          util::info(format!(
            "sync {} {} {}",
            util::path_color(remote_file.to_str().unwrap()),
            arrow_str,
            util::path_color(local_file.to_str().unwrap())
          ));
        }
      }
    }
  }

  Ok(())
}

/// Given a key, value pair into Config::dest, return a vector
/// of the pair of corresponding remote and local files.
pub fn remote_and_local_files<P: AsRef<Path>, Q: AsRef<Path>>(
  remote: P,
  local: Q,
) -> Result<Vec<(PathBuf, PathBuf)>> {
  let remote = remote.as_ref();
  let local = local.as_ref();

  let rot_config_dir = config::rot_config_dir();
  let remote = &rot_config_dir.join(remote);

  if local.is_file() {
    return Ok(vec![(remote.to_path_buf(), local.to_path_buf())]);
  }

  let mut vec = Vec::new();

  for remote_file in WalkDir::new(remote).into_iter() {
    let remote_file = remote_file?;
    let remote_file = remote_file.path();

    if remote_file.is_dir() {
      continue;
    }

    let local_file = local.join(remote_file.strip_prefix(&remote)?);

    vec.push((remote_file.to_path_buf(), local_file.to_path_buf()));
  }

  return Ok(vec);
}

fn file_timestamp<P: AsRef<Path>>(path: P) -> u64 {
  fs::metadata(path)
    .unwrap()
    .modified()
    .unwrap()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

fn sync_direction<P: AsRef<Path>, Q: AsRef<Path>>(files: &[(P, Q)]) -> SyncDirection {
  use SyncDirection::*;

  let (remote_time, local_time, diff) = files
    .iter()
    .map(|(remote_f, local_f)| {
      (
        git::timestamp(remote_f).unwrap(),
        file_timestamp(local_f),
        util::diff(remote_f, local_f).unwrap(),
      )
    })
    .fold(
      (0, 0, false),
      |(r_acc, l_acc, d_acc), (r_time, l_time, diff)| {
        (
          max(r_acc, r_time),
          max(l_acc, l_time),
          d_acc | (None != diff),
        )
      },
    );

  if !diff {
    NoDiff
  } else if remote_time > local_time {
    FromRemote
  } else {
    ToRemote
  }
}
