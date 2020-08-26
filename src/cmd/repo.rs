use crate::config;

use anyhow::Result;

/// Sets the upstream dotfile repo.
///
/// # Arguments
///
/// * `url` - A string slice that holds the url of the repo.
pub fn repo(url: &str) -> Result<()> {
  let mut config = config::get_config()?;
  config.repo = Some(url.to_string());
  config::write_config(&config)
}
