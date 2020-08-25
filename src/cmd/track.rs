use crate::{cmd::config, err, util};
use std::{
  io::Result,
  path::{self, Path},
};

fn infer_name<'a>(path: &'a path::Path, name: Option<&'a str>) -> Option<&'a str> {
  if name.is_some() {
    name
  } else if path.is_dir() {
    Some(path.file_name().unwrap().to_str().unwrap())
  } else if path.parent().unwrap() != config::user_config_dir() {
    dbg!(path.parent().unwrap());
    dbg!(config::user_config_dir());
    Some(
      path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap(),
    )
  } else {
    None
  }
}

pub fn track(path: &str, name: Option<&str>, template: bool) -> Result<()> {
  let path = Path::new(path);
  if !path.exists() {
    return err::err(format!("Path does not exist: '{}'", path.display()));
  }

  let name = match infer_name(path, name) {
    Some(name) => name,
    None => {
      return err::err(format!(
        "Couldn't infer --name for file '{}', set it manually",
        path.display()
      ))
    }
  };

  let mut config = config::get_config()?;
  if config.dest.contains_key(name) {
    return err::err("The name '{}' is already in use");
  } else {
    config
      .dest
      .insert(name.to_string(), path.to_string_lossy().into_owned());

    util::info(format!(
      "tracking '{}' as '{}'",
      path.to_string_lossy().into_owned(),
      name,
    ));
  }

  config::write_config(&config)
}
