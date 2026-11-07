Onefetch reads its settings from a JSONC file (JSON with comments and trailing commas). Options given on the command line override the file.

## Getting started

```sh
onefetch --gen-config        # minimal config: the list of modules
onefetch --gen-config-full   # every setting, with its current value
```

Both write `~/.config/onefetch/config.jsonc` (or `$XDG_CONFIG_HOME/onefetch/config.jsonc`) and refuse to overwrite an existing file; use `--gen-config-force` to overwrite it. Pass a path to write somewhere else, or `-` to print the config. The generated config includes the other options on the command line, so `onefetch --no-merges -E --gen-config-full` writes a config with those set.

Point your editor at the [JSON schema](https://github.com/o2sh/onefetch/raw/main/docs/json_schema.json) (the `$schema` key does this in VS Code and most editors) for completion and validation.

## Where the config is loaded from

Onefetch loads the first `config.jsonc` found in these directories, listed by `onefetch --list-config-paths`:

| Linux / macOS | Windows |
| --- | --- |
| `$XDG_CONFIG_HOME/onefetch/` or `~/.config/onefetch/` | `$XDG_CONFIG_HOME/onefetch/` or `%USERPROFILE%\.config\onefetch\` |
| `$XDG_CONFIG_DIRS/onefetch/` (default `/etc/xdg/onefetch/`) | `%APPDATA%\onefetch\` |
| `/etc/onefetch/` | `%LOCALAPPDATA%\onefetch\` |
| | `%ProgramData%\onefetch\` |

`--config <CONFIG>` loads another file instead. CONFIG is a path, or a file name looked up in those directories, with or without `.jsonc` (so `--config work` finds `~/.config/onefetch/work.jsonc`). `--config -` reads the config from stdin, and `--config none` loads no config file.

## Example

```jsonc
{
  "$schema": "https://github.com/o2sh/onefetch/raw/main/docs/json_schema.json",
  "logo": {
    "type": "builtin",   // auto, builtin, data, file, kitty, sixel, iterm, none
    "source": "rust",
    "color": { "1": 9, "2": 4 }
  },
  "display": {
    "color": { "keys": 12 },
    "numberSeparator": "comma",
    "nerdFonts": true
  },
  "general": {
    "noBots": true,
    "exclude": ["*.lock", "vendor/"]
  },
  "modules": [
    "title",
    "project",
    "version",
    "languages",
    { "type": "authors", "number": 5, "email": true },
    "lastchange",
    { "type": "activity", "months": 6 },
    "linesofcode",
    "license",
    "colors"
  ]
}
```

## Reference

### `logo`

`"logo": null` hides the logo, and `"logo": "go"` is short for `{"source": "go"}`.

| Key | Command line | Description |
| --- | --- | --- |
| `type` | | `auto` (default): the art of the main language, or of `source` if it is a language name; any other `source` is an image path. `builtin`: `source` is a language name. `data`: `source` is the ascii art. `file`: `source` is a text file with the ascii art. `kitty`, `sixel`, `iterm`: `source` is an image shown with that protocol. `none`: no logo. |
| `source` | `--ascii-language`, `--ascii-input`, `--image` | Language name, ascii art, or path, depending on `type` |
| `color` | `--ascii-colors` | Colors of the art, keyed `"1"` to `"9"`, from 0 to 15. Colors not set keep the language's. |
| `colorResolution` | `--color-resolution` | Sixel color resolution: 16, 32, 64, 128 or 256 |

### `display`

| Key | Command line | Description |
| --- | --- | --- |
| `color` | `--text-colors` | Object with `title`, `tilde`, `underline`, `keys`, `separator` and `output` colors, from 0 to 15. Colors not set keep their default. |
| `bold` | `--no-bold` | `false` turns off bold text |
| `numberSeparator` | `--number-separator` | `plain`, `comma`, `space` or `underscore` |
| `isoTime` | `--iso-time` | ISO 8601 timestamps |
| `trueColor` | `--true-color` | `auto`, `never` or `always` |
| `nerdFonts` | `--nerd-fonts` | Nerd Font icons |

### `general`

| Key | Command line | Description |
| --- | --- | --- |
| `exclude` | `--exclude` | Globs of files and directories to ignore |
| `noBots` | `--no-bots` | `true` to exclude bot commits, or a regex matching the authors to exclude |
| `noMerges` | `--no-merges` | Ignore merge commits |
| `includeHidden` | `--include-hidden` | Count hidden files and directories |
| `languageTypes` | `--type` | Types of languages to count: `programming`, `markup`, `prose`, `data` |

### `modules`

The info lines to show, in order: `project`, `description`, `head`, `pending`, `version`, `unreleased`, `created`, `languages`, `dependencies`, `authors`, `lastchange`, `activity`, `contributors`, `url`, `commits`, `churn`, `linesofcode`, `size`, `license`. A module left out is hidden.

`title` and `colors` (the color palette) show or hide the title and the palette. The title is always printed first and the palette last, wherever they are in the list.

A module is a name, or an object with a `type` and options:

| Module | Option | Command line |
| --- | --- | --- |
| `authors` | `number`, `email` | `--number-of-authors`, `--email` |
| `languages` | `number` | `--number-of-languages` |
| `churn` | `number`, `poolSize` | `--number-of-file-churns`, `--churn-pool-size` |
| `activity` | `months` (3, 6 or 12) | `--activity-months` |
| `url` | `http`, `hideToken` | `--http-url`, `--hide-token` |

`--disabled-fields` on the command line hides modules from the list.
