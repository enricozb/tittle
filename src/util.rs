use anyhow::Result;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn info<S: std::fmt::Display>(msg: S) {
  println!("{} {}", "INFO:".green(), msg);
}

pub fn error<S: std::fmt::Display>(msg: S) {
  println!("{} {}", "ERROR:".red(), msg);
}

// https://stackoverflow.com/a/60406693/6101419
pub fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(
  from: U,
  to: V,
) -> Result<(), std::io::Error> {
  let mut stack = Vec::new();
  stack.push(PathBuf::from(from.as_ref()));

  let output_root = PathBuf::from(to.as_ref());
  let input_root = PathBuf::from(from.as_ref()).components().count();

  while let Some(working_path) = stack.pop() {
    // Generate a relative path
    let src: PathBuf = working_path.components().skip(input_root).collect();

    // Create a destination if missing
    let dest = if src.components().count() == 0 {
      output_root.clone()
    } else {
      output_root.join(&src)
    };
    if fs::metadata(&dest).is_err() {
      fs::create_dir_all(&dest)?;
    }

    for entry in fs::read_dir(working_path)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        stack.push(path);
      } else {
        match path.file_name() {
          Some(filename) => {
            let dest_path = dest.join(filename);
            fs::copy(&path, &dest_path)?;
          }
          None => (),
        }
      }
    }
  }

  Ok(())
}

pub fn diff<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<Option<String>> {
  let (from, to) = (from.as_ref(), to.as_ref());

  let diff_bin = match which::which("colordiff") {
    Ok(_) => "colordiff",
    Err(_) => "diff",
  };

  let output = Command::new(diff_bin)
    .arg("-r")
    .arg(from)
    .arg(to)
    .output()?;

  let output = match output.status.code() {
    None => None,
    Some(code) => {
      if code == 0 {
        None
      } else {
        Some(std::str::from_utf8(&output.stdout)?.to_string())
      }
    }
  };

  Ok(output)
}
