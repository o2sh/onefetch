# How to Contribute

- [How to Contribute](#how-to-contribute)
  - [Contributing via Pull Requests](#contributing-via-pull-requests)
    - [Adding support for a new language](#adding-support-for-a-new-language)
    - [Adding support for a new package manager](#adding-support-for-a-new-package-manager)
    - [Adding translation for README.md](#adding-translation-for-readmemd)

## Contributing via Pull Requests

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

- link 1: https://github.com/XAMPPRocky/tokei#supported-languages
- link 2: https://github.com/github/linguist/blob/master/lib/linguist/languages.yml

### Adding support for a new package manager

To add a new package manager, make sure to follow these steps:

1. Add a new entry in the `define_package_managers!` macro in [package_manager.rs](src/info/deps/package_manager.rs).

**Example**:

`{ Cargo, "cargo", [ ("Cargo.toml", cargo) ] },`

The first item `Cargo` corresponds to the name of the package manager. The second item `cargo` is the display name. Then we have the name of the package manager file that will be parsed: `Cargo.toml` along with its parser `cargo` (cf. step 2), notice that the third item is an array of tuple in case the package manager has multiple parsers (e.g. pip).

2. In [package_manager.rs](src/info/deps/package_manager.rs): create a function that takes an input of type `&str` representing the content of the package manager's file, and returns a `usize` as its number of dependencies.

```rust
pub fn cargo(contents: &str) -> Result<usize> {
    let parsed = contents.parse::<Value>()?;
    let count = parsed.get("dependencies");

    match count {
        Some(val) => Ok(val.as_table().unwrap().len()),
        None => Ok(0),
    }
}
```

### Adding translation for README.md

In order to make Onefetch more accessible for non English-speakers, we are seeking the help of multilingual contributors willing to translate the README.md in their native tongue.

These translations will be accessible directly from the English README.md via hyperlinks.

<h2 align="center">Special Thanks to</h2>

- Every onefetch user, who contributed to the project by writing issues or PRs.
- [@spenserblack](https://github.com/spenserblack), [@CephalonRho](https://github.com/CephalonRho), [@yoichi](https://github.com/yoichi), [@HallerPatrick](https://github.com/HallerPatrick) and [@Byron](https://github.com/Byron) for maintaining this project.
- Everyone I forgot to mention here, but also influenced onefetch and helped it grow.

<p align="center">:heart::heart:</p>

<p align="center">
  <img src="https://contrib.rocks/image?repo=o2sh/onefetch" />
</p>

Made with [contributors-img](https://contrib.rocks).
