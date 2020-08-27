use crate::util::color;
use crate::{config, err};

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Render a template to its location given the replacement variables.
fn render_template<P: AsRef<Path>, Q: AsRef<Path>>(
  template: P,
  render_to: Q,
  vars: &HashMap<String, String>,
) -> Result<()> {
  use std::io::prelude::*;

  let mut contents = String::new();
  File::open(&template)?.read_to_string(&mut contents)?;
  for (var, value) in vars.iter() {
    contents = contents.replace(&format!("{{{{{}}}}}", var.trim()), value);
  }
  let re = Regex::new(r"\{\{([^}]*)\}\}").unwrap();
  if let Some(var) = re.captures(&contents) {
    return err::err(format!(
      "In template {}: this machine has no value for variable {}",
      color::path(template),
      color::emphasis(var[1].to_owned())
    ));
  }

  write!(File::create(render_to)?, "{}", contents)?;

  Ok(())
}

/// Render all local templates to their location.
pub fn render() -> Result<()> {
  let config = config::get_config()?;
  let vars = config.vars();

  for (remote_name, render_to) in config.templates().iter() {
    render_template(config.dest(remote_name), render_to, &vars)?;
  }

  Ok(())
}
