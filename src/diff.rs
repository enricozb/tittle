use anyhow::Result;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn diff<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<bool> {
  let (from, to) = (from.as_ref(), to.as_ref());

  let diff_bin = match which::which("colordiff") {
    Ok(_) => "colordiff",
    Err(_) => "diff",
  };

  let mut child = Command::new(diff_bin)
    .arg("-qr")
    .arg(from)
    .arg(to)
    .stderr(Stdio::null())
    .stdout(Stdio::piped())
    .spawn()?;

  Ok(child.wait()?.code().unwrap() == 1)
}
