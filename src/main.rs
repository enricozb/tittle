use anyhow::Result;
use clap::{App, AppSettings, Arg, SubCommand};

mod cmd;
mod config;
mod err;
mod git;
mod util;

fn main() {
  let matches = App::new("tittle")
    .version("0.2.0")
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
      SubCommand::with_name("clone")
        .about(
          "Clone a dotfile repository. This cannot be run if any local dotfile \
         repo already exists.",
        )
        .arg(
          Arg::with_name("URL")
            .help("The upstream repo url")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      SubCommand::with_name("diff").about(
        "Show diffs between remote and local dotfiles. Uses colordiff if available",
      ),
    )
    .subcommand(
      SubCommand::with_name("edit")
        .about("Edit the tittle config")
        .arg(
          Arg::with_name("MODE")
            .help("One of [me]. Specifies which portion of the config to edit.")
            .index(1),
        ),
    )
    .subcommand(SubCommand::with_name("pull").about("Pulls the repository from upstream"))
    .subcommand(
      SubCommand::with_name("push").about("Pushes the current repository upstream"),
    )
    .subcommand(
      SubCommand::with_name("remove").about("Remove tracked files or directories"),
    )
    .subcommand(
      SubCommand::with_name("render")
        .about("Render templates to their respective locations"),
    )
    .subcommand(
      SubCommand::with_name("repo")
        .about("Sets the upstream dotfile repo")
        .arg(
          Arg::with_name("URL")
            .help("The upstream repo url")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      SubCommand::with_name("sync").about("Sync between remote and local dotfiles"),
    )
    .subcommand(
      SubCommand::with_name("track")
        .about("Track a file or directory")
        .arg(
          Arg::with_name("name")
            .short("n")
            .long("name")
            .value_name("NAME")
            .help("Sets a custom name for the tracked path"),
        )
        .arg(
          Arg::with_name("renders_to")
            .short("t")
            .long("renders_to")
            .value_name("PATH")
            .help("If set, then the tracked file is a template and renders to this path"),
        )
        .arg(
          Arg::with_name("PATH")
            .help("The path to track")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(SubCommand::with_name("tree").about("Show a tree of the tracked files"))
    .get_matches();

  let run = || -> Result<()> {
    // when cloning, don't initialize config and git first
    if let ("clone", Some(matches)) = matches.subcommand() {
      git::clone(matches.value_of("URL").unwrap())?;
      return Ok(());
    }

    config::init()?;
    git::init()?;

    match matches.subcommand() {
      ("diff", _) => cmd::diff::diff()?,

      ("edit", Some(matches)) => cmd::edit::edit(matches.value_of("MODE"))?,

      ("pull", _) => git::pull()?,
      ("push", _) => git::push()?,

      ("remove", _) => cmd::remove::remove()?,

      ("render", _) => cmd::render::render()?,

      ("repo", Some(matches)) => git::set_remote(matches.value_of("URL").unwrap())?,

      ("sync", _) => cmd::sync::sync()?,

      ("track", Some(matches)) => cmd::track::track(
        matches.value_of("PATH").unwrap(),
        matches.value_of("name"),
        matches.value_of("renders_to"),
      )?,

      ("tree", _) => cmd::tree::tree()?,

      _ => {}
    }

    Ok(())
  };

  if let Err(err) = run() {
    util::error(err);
  }
}
