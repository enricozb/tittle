use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn diff<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<bool> {
  let (from, to) = (from.as_ref(), to.as_ref());

  let diff_bin = match which::which("colordiff") {
    Ok(_) => "colordiff",
    Err(_) => "diff"
  };

  let status = Command::new(diff_bin)
    .arg("-qr")
    .arg(from)
    .arg(to)
    .status()?
    .code()
    .unwrap();

  Ok(status == 1)
}
