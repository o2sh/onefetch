### Adding support for a new language

Adding support for a new Language consists in adding a new entry to [language.yaml](./languages.yaml) and filling it in with the right data.

**Example**:

```
CSharp: # required, this will be the name of the enum variant for the language as specified by tokei (link 1)
  type: programming # required, can be programming, data, markup, or prose as specified by linguist (link 2)
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
      - '#9B4F97'
      - '#67217A'
      - '#803788'
      - '#FFFFFF'
    chip: '#178600' # required, this is used for the language breakdown bar, its value can be found in linguist (link 2).
  serialization: c# # required only if the Enum name `CSharp` doesn't match the display name `C#`
```

_**NOTE**: An additional field, `line_types` can also be set on a language's attributes. It has been excluded because_
_it is not necessary for the majority of languages. By default, only a language's lines of code are counted, but this_
_field can be used to count other lines, too. For example, `line_types: [code, comments]`. This is useful in languages_
_like Markdown, where the significant lines are mostly comments. A list of available fields to be used can be found in_
_[tokei's documentation](https://docs.rs/tokei/latest/tokei/struct.Language.html#fields)._

- link 1: https://github.com/XAMPPRocky/tokei#supported-languages
- link 2: https://github.com/github/linguist/blob/master/lib/linguist/languages.yml

### Adding translation for README.md

In order to make Onefetch more accessible for non English-speakers, we are seeking the help of multilingual contributors willing to translate the README.md in their native tongue.

These translations will be accessible directly from the English README.md via hyperlinks.

<h2 align="center">Special Thanks to</h2>

- Every onefetch user, who contributed to the project by writing issues or PRs.
- [@spenserblack](https://github.com/spenserblack), [@CephalonRho](https://github.com/CephalonRho), [@yoichi](https://github.com/yoichi) and [@Byron](https://github.com/Byron) for maintaining this project.
- Everyone I forgot to mention here, but also influenced onefetch and helped it grow.

<p align="center">:heart::heart:</p>

<p align="center">
  <img src="https://contrib.rocks/image?repo=o2sh/onefetch" />
</p>

Made with [contributors-img](https://contrib.rocks).
