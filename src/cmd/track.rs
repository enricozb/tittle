use crate::{config, err, util};

use anyhow::Result;
use std::{fs, path::Path};

/// Returns a remote name given a local `path`.
fn infer_name_from_path<P: AsRef<Path>>(path: &P) -> Option<&std::ffi::OsStr> {
  let path = path.as_ref();

  if path.is_dir() {
    path.file_name()
  } else {
    None
  }
}

/// Returns a remote name given a remote `path` and optionally an override `name`.
fn infer_name<P: AsRef<Path>>(path: &P, name: Option<&str>) -> Option<String> {
  let path = path.as_ref();

  let name = match (name, infer_name_from_path(&path)) {
    (Some(name), _) => name,
    (_, Some(osstr_name)) => osstr_name.to_str().unwrap(),
    (None, None) => return None,
  };

  if path.is_file() {
    Some(
      Path::new(name)
        .join(path.file_name().unwrap())
        .to_str()
        .unwrap()
        .to_owned(),
    )
  } else {
    Some(name.to_owned())
  }
}

/// Copies local `path` to be tracked under the remote directory `name`.
///
/// If `path` is a directory then all of its contents are copied to the remote `name`.
fn copy<P: AsRef<Path>>(path: P, name: &str) -> Result<()> {
  let path = path.as_ref();

  let dest = config::rot_config_dir().join(name);

  if path.is_dir() {
    util::copy_dir(path, dest)?;
  } else {
    if !dest.parent().unwrap().exists() {
      fs::create_dir(&dest.parent().unwrap())?;
    }
    fs::copy(path, dest)?;
  }

  Ok(())
}

/// Track a local `path` under a remote `name`, potentially making it a template.
/// The `renders_to` argument points to the path that the template `path` renders to.
/// If `renders_to` is not `None` then `path` must be pointing to a template file.
pub fn track<P: AsRef<Path>, Q: AsRef<Path>>(
  path: P,
  name: Option<&str>,
  renders_to: Option<Q>,
) -> Result<()> {
  let path = path.as_ref().canonicalize()?;

  if !path.exists() {
    return err::err(format!("Path does not exist: '{}'", path.display()));
  }

  // Ensure that `renders_to` is set only if path is a file.
  match renders_to {
    Some(_) if path.is_dir() => {
      return err::err("--renders_to can only be set if PATH is a file, not a directory")
    }
    _ => (),
  }

  let name = match infer_name(&path, name) {
    Some(name) => name,
    None => {
      return err::err(format!(
        "Couldn't infer --name for file '{}', set it manually",
        path.display()
      ))
    }
  };

  let mut config = config::get_config()?;
  let path_string = path.to_string_lossy();

  if config.dest.contains_key(&name) {
    return err::err(format!("The name '{}' is already being tracked", name));
  } else {
    copy(&path, &name)?;

    config
      .dest
      .insert(name.to_string(), path_string.to_string());

    util::info(format!(
      "tracking {} under {}",
      util::path_color(&path),
      util::path_color(name),
    ));
  }

  if let Some(renders_to) = renders_to {
    config.templates.insert(
      path_string.to_string(),
      renders_to
        .as_ref()
        .canonicalize()?
        .to_string_lossy()
        .to_string(),
    );

    util::info(format!(
      "template {} renders to {}",
      util::path_color(&path),
      util::path_color(renders_to),
    ));
  }

  config::write_config(&config)
}
