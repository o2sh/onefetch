### Getting started

Follow the instructions for [installing onefetch from source](https://github.com/o2sh/onefetch/wiki/Installation#build-from-source).

> [!NOTE]
>
> Please be respectful and constructive in issues, pull requests, and reviews. Keep in mind that this project is maintained in contributors' free time.

### Adding support for a new language

Adding support for a new Language consists in adding a new entry to [language.yaml](./languages.yaml) and filling it in with the right data. [^1]

**Example**:

```yaml
CSharp: # required, this will be the name of the enum variant for the language as specified by tokei [1]
  type: programming # required, can be programming, data, markup, or prose as specified by linguist [2]
  # required, this is the logo. If it's not within 25x40 bounds, you will get a compiler error. Use `{i}` to color the ascii with `i` the color index.
  ascii: |
    {0}                 ++++++
    {0}              ++++++++++++
    {0}          ++++++++++++++++++++
    {0}       ++++++++++++++++++++++++++
    {0}    ++++++++++++++++++++++++++++++++
    {0} +++++++++++++{3}************{0}+++++++++++++
    {0}+++++++++++{3}******************{0}++++++++{2};;;
    {0}+++++++++{3}**********************{0}++{2};;;;;;;
    {0}++++++++{3}*********{0}++++++{3}******{2};;;;;;;;;;;
    {0}+++++++{3}********{0}++++++++++{3}**{2};;;{3}**{2};;;{3}**{2};;;
    {0}+++++++{3}*******{0}+++++++++{2};;;;;;{3}*********{2}::
    {0}+++++++{3}******{0}+++++++{2};;;;;;;;;;{3}**{2};;;{3}**{2};;;
    {0}+++++++{3}*******{0}+++{1}:::::{2};;;;;;;{3}*********{2};;
    {0}+++++++{3}********{1}::::::::::{3}**{2};;;{3}**{2};;;{3}**{2};;;
    {0}++++++++{3}*********{1}::::::{3}******{2};;;;;;;;;;;
    {0}++++++{1}:::{3}**********************{1}::{2};;;;;;;
    {0}+++{1}::::::::{3}******************{1}::::::::{2};;;
    {1} :::::::::::::{3}************{1}:::::::::::::
    {1}    ::::::::::::::::::::::::::::::::
    {1}       ::::::::::::::::::::::::::
    {1}          ::::::::::::::::::::
    {1}              ::::::::::::
    {1}                 ::::::
  colors:
    ansi: # required, a list of the ANSI colors used to colorize the logo
      - blue
      - magenta
      - magenta
      - white
    hex: # optional, alternative to basic colors for terminals that support true colour.
      - "#9B4F97"
      - "#67217A"
      - "#803788"
      - "#FFFFFF"
    chip: "#178600" # required, this is used for the language breakdown bar, its value can be found in linguist [3].
  icon: '\u{E648}' # optional, the UTF-16 code point of the nerd font icon if supported (link 3).
  serialization: c# # required only if the Enum name `CSharp` doesn't match the display name `C#`
```

[1]: https://github.com/XAMPPRocky/tokei#supported-languages
[2]: https://github.com/github/linguist/blob/master/lib/linguist/languages.yml
[3]: https://www.nerdfonts.com/cheat-sheet

- Match the original language logo as closely as possible so it is easyily recognizable.
- Prioritize clean proportions and strong shape symmetry.
- You may use AI or online ASCII-art tools to generate a starting point, but the final result must be polished by hand.
- Use existing ASCII art as inspiration and reference material where helpful, for example: https://ascii.co.uk/art

<h2 align="center">Special Thanks to</h2>

- Every onefetch user, who contributed to the project by writing issues or PRs.
- [@spenserblack](https://github.com/spenserblack) and [@Byron](https://github.com/Byron) for maintaining this project.
- Everyone I forgot to mention here, but also influenced onefetch and helped it grow.

<p align="center">:heart::heart:</p>

<p align="center">
  <img src="https://contrib.rocks/image?repo=o2sh/onefetch" />
</p>

Made with [contributors-img](https://contrib.rocks).
