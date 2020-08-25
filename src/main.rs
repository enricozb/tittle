use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};

mod cmd;
mod config;
mod err;
mod util;

fn main() {
  let matches = App::new("rot")
    .version("0.0.1")
    .author("Enrico Z. Borba <enricozb@gmail.com>")
    .about("Dotfile manager")
    .setting(AppSettings::VersionlessSubcommands)
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .help("Print commands as they are run"),
    )
    .subcommand(
      SubCommand::with_name("track")
        .about("track a file or directory")
        .arg(
          Arg::with_name("name")
            .short("n")
            .long("name")
            .value_name("NAME")
            .help("Sets a custom name for the tracked path"),
        )
        .arg(
          Arg::with_name("template")
            .short("t")
            .long("template")
            .help("Whether or not this path is a template"),
        )
        .arg(
          Arg::with_name("PATH")
            .help("the path to track")
            .required(true)
            .index(1),
        ),
    )
    .get_matches();

  let run = || -> Result<()> {
    config::init()?;

    if let Some(matches) = matches.subcommand_matches("track") {
      cmd::track::track(
        matches.value_of("PATH").unwrap(),
        matches.value_of("name"),
        matches.is_present("template"),
      )?
    }

    Ok(())
  };

  if let Err(err) = run() {
    util::error(err);
  }
}
