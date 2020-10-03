# T0D0

## Bugs
- `tittle sync` fails if a destination is in a deep-nonexistent directory
  - create parent directory of destination.

- `tittle edit` doesn't generate a commit after edit.

## Debt
- remove `repo` field from `config.json` and let the git repository handle it.

## Features
- add readme on initialization referencing tittle

- add a commit message option

- commits should reference the machine ID that made them.

- add a remove
  - maybe similar to edit where we see only the destinations, and we remove the
    ones we don't want to track.
  - doesn't remove local files

- tracking a file in a directory that was already tracked

- the *dest* field in the config should allow for some variables
```
{
  "i3/config": "$HOME/.config/i3/config.template"
}
```
- add `tittle edit a@b` to edit a specific machine's overrides.

- adding new files under a tracked directory is not detected?
  - walk each directory independently.

## Maybe Done
- tracking a file or directory with a deep name: `tittle track x -n a/b/c`
- added `clone` command.
