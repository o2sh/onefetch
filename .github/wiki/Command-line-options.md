```man
Usage: onefetch [OPTIONS] [INPUT]

Arguments:
  [INPUT]
          Run as if onefetch was started in <input> instead of the current working directory

Options:
      --ascii-input <STRING>
          Takes a non-empty STRING as input to replace the ASCII logo

          It is possible to pass a generated STRING by command substitution

          For example:

          '--ascii-input "$(fortune | cowsay -W 25)"'

  -a, --ascii-language <LANGUAGE>
          Which LANGUAGE's ascii art to print

  -c, --ascii-colors <X>...
          Colors (X X X...) to print the ascii art

  -d, --disabled-fields <FIELD>...
          Allows you to disable FIELD(s) from appearing in the output

  -i, --image <IMAGE>
          Path to the IMAGE file

      --image-protocol <IMAGE_PROTOCOL>
          Which image protocol to use

          [possible values: kitty, sixel, iterm]

      --color-resolution <VALUE>
          VALUE of color resolution to use with SIXEL backend

          [default: 16]
          [possible values: 16, 32, 64, 128, 256]

      --no-bold
          Turns off bold formatting

      --no-merges
          Ignores merge commits

      --no-color-palette
          Hides the color palette

      --no-title
          Hides the title

      --number-of-authors <NUM>
          Maximum NUM of authors to be shown

          [default: 3]

      --number-of-languages <NUM>
          Maximum NUM of languages to be shown

          [default: 6]

  -e, --exclude <EXCLUDE>...
          Ignore all files & directories matching EXCLUDE

      --no-bots [<REGEX>]
          Exclude [bot] commits. Use <REGEX> to override the default pattern

  -l, --languages
          Prints out supported languages

  -p, --package-managers
          Prints out supported package managers

  -o, --output <FORMAT>
          Outputs Onefetch in a specific format

          [possible values: json, yaml]

      --true-color <WHEN>
          Specify when to use true color

          If set to auto: true color will be enabled if supported by the terminal

          [default: auto]
          [possible values: auto, never, always]

      --show-logo <WHEN>
          Specify when to show the logo

          If set to auto: the logo will be hidden if the terminal's width < 95

          [default: always]
          [possible values: auto, never, always]

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

  -E, --email
          Show the email address of each author

      --include-hidden
          Count hidden files and directories

  -T, --type <TYPE>...
          Filters output by language type

          [default: programming markup]
          [possible values: programming, markup, prose, data]

      --generate <SHELL>
          If provided, outputs the completion file for given SHELL

          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
