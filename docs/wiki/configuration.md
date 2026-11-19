Onefetch can read its options from a config file, so you don't have to type them every time.

## Location

Onefetch reads `~/.config/onefetch/config.jsonc`, or `$XDG_CONFIG_HOME/onefetch/config.jsonc` if `XDG_CONFIG_HOME` is set. This is the same on Linux, macOS and Windows. Nothing happens if the file doesn't exist.

To use another file, pass `--config <PATH>`.

## Format

The file is a JSON object whose keys are the [command-line options](https://github.com/o2sh/onefetch/wiki/command-line-options), written with their long name and without the leading `--`. Comments (`//` and `/* */`) and trailing commas are allowed.

```jsonc
{
  // Show 5 authors instead of 3
  "number-of-authors": 5,
  "ignore-author": "^bors$",
  "no-merges": true,
  "exclude": ["*.lock", "vendor/"],
  "disabled-fields": ["churn", "url"],
  "text-colors": [9, 10, 11, 12, 13, 14],
  "number-separator": "comma",
  "nerd-fonts": true
}
```

Values work like on the command line:

| Option | Value | Example |
| --- | --- | --- |
| A flag, like `--no-merges` | `true` turns it on, `false` does nothing | `"no-merges": true` |
| An option with one value, like `--number-of-authors <NUM>` | a number or a string | `"number-of-authors": 5` |
| An option with several values, like `--exclude <EXCLUDE>...` | a list | `"exclude": ["*.lock"]` |

An option with one value also takes a list of strings, which are joined into lines. This keeps a custom logo readable:

```jsonc
"ascii-input": [
  "{0} /\\_/\\",
  "{0}( o.o )",
  "{0} > ^ <"
]
```

## Example

[`presets/cowsay.jsonc`](https://github.com/o2sh/onefetch/blob/main/presets/cowsay.jsonc) uses every option, with a cowsay cow as the logo. Try it with `onefetch --config presets/cowsay.jsonc`, or copy it to `~/.config/onefetch/config.jsonc` and keep the options you want.

## The command line wins

An option given on the command line replaces the same option in the file. With `"number-of-authors": 5` in the config, `onefetch --number-of-authors 2` shows 2 authors.

A flag turned on in the file can't be turned off from the command line. To run onefetch without the config, point `--config` at an empty file that contains `{}`.
