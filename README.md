# Rot - Dotfile Manager

Rot tracks your dotfiles under version control and supports fine-grained
user/machine-specific configuration.

## Quick Start
```
> rot track ~/.config/i3
INFO: tracking /home/enricozb/.config/i3 under i3
> rot repo git@github.com:enricozb/dotfiles
> rot push
```

## Install
Currently the easiest way to install rot is through `cargo`:
```
cargo install rot-dot
```

## Documentation
  - [commands](doc/commands.asciidoc): for info on the commands that rot accepts.
  - [config](doc/config.asciidoc): for the rot config specification.
  - [templates](doc/templates.asciidoc): for how to use templates and variables.

## Terminology
Throughout the codebase and this document the terms *remote* and *local* appear often.
*Remote* refers to files that are under the `$XDG_CONFIG_HOME/rot` directory. *Local*
files are those on the user's filesystem currently being used as configs.

## Todo
See [todo](todo.md).

## Why?
No idea.
