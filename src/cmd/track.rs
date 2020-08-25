use crate::{config, err, util};
use anyhow::Result;
use std::{fs, path::Path};

fn infer_name_from_path<P: AsRef<Path>>(path: &P) -> Option<&std::ffi::OsStr> {
  let path = path.as_ref();

  if path.is_dir() {
    path.file_name()
  } else if path.parent().unwrap() != config::user_config_dir() {
    path.parent().unwrap().file_name()
  } else {
    None
  }
}

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

pub fn track<P: AsRef<Path>>(path: P, name: Option<&str>, template: bool) -> Result<()> {
  let path = path.as_ref();

  println!("{} {:?} {}", path.display(), name, template);

  if !path.exists() {
    return err::err(format!("Path does not exist: '{}'", path.display()));
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
  if config.dest.contains_key(&name) {
    return err::err(format!("The name '{}' is already being tracked", name));
  } else {
    println!("Copying '{}' to '{}'", &path.display(), &name);
    copy(&path, &name)?;

    config
      .dest
      .insert(name.to_string(), path.to_string_lossy().into_owned());

    util::info(format!(
      "tracking '{}' under '{}'",
      path.to_string_lossy().into_owned(),
      name,
    ));
  }

  config::write_config(&config)
}
