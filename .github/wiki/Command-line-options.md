```man
Usage: onefetch.exe [OPTIONS] [INPUT]

Arguments:
  [INPUT]
          Run as if onefetch was started in <input> instead of the current working directory

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

INFO:
  -d, --disabled-fields <FIELD>...
          Allows you to disable FIELD(s) from appearing in the output

      --no-title
          Hides the title

      --number-of-authors <NUM>
          Maximum NUM of authors to be shown

          [default: 3]

      --number-of-languages <NUM>
          Maximum NUM of languages to be shown

          [default: 6]

      --number-of-file-churns <NUM>
          Maximum NUM of file churns to be shown

          [default: 3]

      --churn-pool-size <NUM>
          Minimum NUM of commits from HEAD used to compute the churn summary

          By default, the actual value is non-deterministic due to time-based computation and will be displayed under the info title "Churn (NUM)"

  -e, --exclude <EXCLUDE>...
          Ignore all files & directories matching EXCLUDE

      --no-bots [<REGEX>]
          Exclude [bot] commits. Use <REGEX> to override the default pattern

      --no-merges
          Ignores merge commits

  -E, --email
          Show the email address of each author

      --include-hidden
          Count hidden files and directories

  -T, --type <TYPE>...
          Filters output by language type

          [default: programming markup]
          [possible values: programming, markup, prose, data]

TEXT FORMATTING:
  -t, --text-colors <X>...
          Changes the text colors (X X X...)

          Goes in order of title, ~, underline, subtitle, colon, and info

          For example:

          '--text-colors 9 10 11 12 13 14'

  -z, --iso-time
          Use ISO 8601 formatted timestamps

      --number-separator <SEPARATOR>
          Which thousands SEPARATOR to use

          [default: plain]
          [possible values: plain, comma, space, underscore]

      --no-bold
          Turns off bold formatting

ASCII:
      --ascii-input <STRING>
          Takes a non-empty STRING as input to replace the ASCII logo

          It is possible to pass a generated STRING by command substitution

          For example:

          '--ascii-input "$(fortune | cowsay -W 25)"'

  -c, --ascii-colors <X>...
          Colors (X X X...) to print the ascii art

  -a, --ascii-language <LANGUAGE>
          Which LANGUAGE's ascii art to print

      --true-color <WHEN>
          Specify when to use true color

          If set to auto: true color will be enabled if supported by the terminal

          [default: auto]
          [possible values: auto, never, always]

IMAGE:
  -i, --image <IMAGE>
          Path to the IMAGE file

      --image-protocol <PROTOCOL>
          Which image PROTOCOL to use

          [possible values: kitty, sixel, iterm]

      --color-resolution <VALUE>
          VALUE of color resolution to use with SIXEL backend

          [default: 16]
          [possible values: 16, 32, 64, 128, 256]

VISUALS:
      --no-color-palette
          Hides the color palette

      --no-art
          Hides the ascii art or image if provided

DEVELOPER:
  -o, --output <FORMAT>
          Outputs Onefetch in a specific format

          [possible values: json, yaml]

      --generate <SHELL>
          If provided, outputs the completion file for given SHELL

          [possible values: bash, elvish, fish, powershell, zsh]

OTHER:
  -l, --languages
          Prints out supported languages

  -p, --package-managers
          Prints out supported package managers
```
