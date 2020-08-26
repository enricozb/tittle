use crate::config;

use anyhow::Result;

pub fn repo(url: &str) -> Result<()> {
  let mut config = config::get_config()?;
  config.repo = Some(url.to_string());
  config::write_config(&config)
}
