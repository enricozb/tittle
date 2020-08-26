use crate::{config, git, util};

use anyhow::Result;
use std::cmp::max;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

/// Represents which direction to sync dotfiles.
#[derive(PartialEq)]
enum SyncDirection {
  FromRemote,
  ToRemote,
  NoDiff,
}

/// Synchronizes the remote and local dotfiles.
///
/// If two files are the same, then no copying occurs. Otherwise, the newer file
/// remains and is copied onto the other. The timestamp of a file depends on whether
/// it is a local file or a remote file. Remote files have their timestamps determined
/// by the time of the most recent commit which modifies them. Local files' timestamps
/// are determined by the filesystem.
pub fn sync() -> Result<()> {
  use SyncDirection::*;

  for (remote, local) in config::get_config()?.dests().iter() {
    let files = remote_and_local_files(&remote, &local)?;

    let direction = sync_direction(&files);
    if direction == NoDiff {
      continue;
    }

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
        util::path_color(remote_file),
        arrow_str,
        util::path_color(local_file)
      ));
    }
  }

  Ok(())
}

/// Returns the pairs of corresponding files under a tracked file or directory.
///
/// # Arguments
///
/// * `remote` - A key from `Config::dest`.
/// * `local` - The corresponding value to the key `remote`.
pub fn remote_and_local_files<P: AsRef<Path>, Q: AsRef<Path>>(
  remote: P,
  local: Q,
) -> Result<Vec<(PathBuf, PathBuf)>> {
  let remote = remote.as_ref();
  let local = local.as_ref();

  let rot_config_dir = config::rot_config_dir();
  let remote = &rot_config_dir.join(remote);

  if remote.is_file() {
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

/// Returns a file's timestamp in seconds. If the file does not exist then return 0.
fn file_timestamp<P: AsRef<Path>>(path: P) -> u64 {
  if !path.as_ref().exists() {
    return 0;
  }

  fs::metadata(path)
    .unwrap()
    .modified()
    .unwrap()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

/// Returns which direction to sync this sequence of `(remote, local)` files.
///
/// This function assumes that this is an return value of `remote_and_local_files`.
/// If no diffs occur between the pairs of files in `files`, then `NoDiff` will be
/// returned. If the remote files contain the newest file, then `FromRemote`
/// is returned. Otherwise, `ToRemote` is returned.
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
