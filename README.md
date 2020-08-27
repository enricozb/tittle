# Rot - Dotfile Manager

Rot tracks your dotfiles under version control and supports fine-grained
machine-specific configuration.

## Quick Start
```
> rot repo git@github.com:enricozb/dotfiles
> rot track ~/.config/i3
INFO: tracking /home/enricozb/.config/i3 under i3
> rot sync
```

## Usage
See `rot help [SUBCOMMAND]` for details, though most of the commands are shown here
under the [Common Commands](#common-commands) section.

## Terminology
Throughout the codebase and this document the terms *remote* and *local* appear often.
*Remote* refers to files that are under the `$XDG_CONFIG_HOME/rot` directory. *Local*
files are those on the user's filesystem currently being used as configs.

## Common Commands

### `repo`  Setting an upstream repo for your dotfiles
This is done with `rot repo URL`. `URL` can be upstream repository url that `git` would
accept, for example, using an SSH url:
```
rot repo git@github.com:enricozb/dotfiles
```

### `sync`  Synchronizing your local and remote dotfiles
```
rot sync
```
This pulls from the upstream dotfile repository if any exists and then will copy
between remote and locally tracked files depending on which one was most recently
edited. This will also print out any sync operations that occur, for example:
```
INFO: sync /home/enricozb/.config/rot/i3/config.template <- /home/enricozb/.config/i3/config.template
```

### `edit`  Modify your Rot configuration
```
rot edit [MODE]
```
The optional positional argument `MODE` specifies which portion of the
rot config is being edited. This is one of
 - `me`  Edit this machine's specific overrides

### `track`  Tracking files

#### Tracking a directory
```
rot track ~/.config/i3/
```
This tracks all files under the entire `i3` directory and when synchronizing between
the remote and local files, it is entirely in a single direction depending on whether
the remote or the local files contain the oldest file.

#### Tracking a single file
```
rot track ~/.config/i3/config --name i3
```
Notice that when tracking single files we specify a name to track them under. The
`rot` directory would now look like:
```
/home/enricozb/.config/rot
└── i3
    └── config
```

## Overrides
Rot lets you specify machine-specific overrides in order to control:
 - variables inside templates
 - where tracked files end up
 - where templates are rendered to

### Templates
Rot supports templatizing configs so they can render differently on
each machine.  For example, say your terminal's config lives under
`~/.term.yml`, and it looks like this YAML file:
```yml
font:
  normal:
    family: Terminus
    style: Medium

  size: 10.0
```
Now, say you want `size` to be different on different machines. You would first create a
template file `~/.term.yml.template` (the actual extension is irrelevant and  this file
can be anywhere), and you would replace the machine-specific value with a *variable*
like so:

```yml
font:
  normal:
    family: Terminus
    style: Medium

  size: {{FONT_SIZE}}
```
Variables must be surrounded by double braces `{{...}}` in their template files. Then,
you would track this template file and tell Rot where it should render the template to:
```
rot track ~/.term.yml.template --name term --render_to ~/.term.yml
```
Lastly, we need to set the variables for this machine. We can do so by editing in *me*
mode:
```
rot edit me
```
A JSON file will appear in whatever `$EDITOR` is set, and we need to add some value
to our `FONT_SIZE` variable:
```
{
  "dest": {},
  "templates": {},
  "vars": {
    "FONT_SIZE": "10.0"
  }
}
```
Lastly, we do
```
rot render
```
to render all templates. If a variable is not substituted, `rot` will inform you.

### `dest` and `templates` Overrides
If you edit the current machine's override config:
```
rot edit me
```
You will see that the default override entries are
```
{
  "dest": {},
  "templates": {},
  "vars": {}
}
```
Each entry corresponds to:
  - `dest` and `templates`: allows you to override the `dest` and `templates` maps in
    the default config. See the default config with `rot edit`.
  - `vars`: discussed in the previous section
