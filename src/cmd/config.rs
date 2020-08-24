use std::{
  env, fs,
  path::{self, Path},
};

fn config_dir() -> path::PathBuf {
  if let Ok(config_dir) = env::var("XDG_CONFIG_HOME") {
    return Path::new(&config_dir).join("rot");
  }

  return Path::new(&env::var("HOME").unwrap())
    .join(".config")
    .join("rot");
}

pub fn init() -> std::io::Result<()> {
  let config_dir = config_dir();
  if !config_dir.exists() {
    fs::create_dir(config_dir)?;
  }
  Ok(())
}

