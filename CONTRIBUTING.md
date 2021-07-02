# How to Contribute

- [How to Contribute](#how-to-contribute)
  - [Contributing via Pull Requests](#contributing-via-pull-requests)
    - [Adding support for a new language](#adding-support-for-a-new-language)
      - [Ascii logo](#ascii-logo)
    - [Adding support for a new package manager](#adding-support-for-a-new-package-manager)
    - [Adding translation for README.md](#adding-translation-for-readmemd)

## Contributing via Pull Requests

### Adding support for a new language

Before anything, you need to make sure that the language you want to add is supported by ([tokei](https://github.com/XAMPPRocky/tokei#supported-languages)).

Adding support for a new Language consists in adding a new entry in the `define_language!` macro in [language.rs](https://raw.githubusercontent.com/o2sh/onefetch/main/src/language.rs) in filling it in with the right data.

**Example**:

` { CSharp, "csharp.ascii", define_colors!( [Color::Blue, Color::Magenta] ), "c#" }, `

The first item `CSharp` corresponds to the name of the language as defined in tokei. The second item `csharp.ascii` is the name of the file containing the ascii logo: this file has to be placed in the _./resources_ folder (more info below). Then we have the colors used to customize the look of the ascii logo when displayed to the screen. The last item `c#` is only required if the Enum name `CSharp` doesn't match the display name `C#` and is used as an input for `-a, --ascii-language <LANGUAGE>` - by default we take the Enum name in lowercase.

#### Ascii logo

```text
{0}                 ++++++
{0}              ++++++++++++
{0}          ++++++++++++++++++++
{0}       ++++++++++++++++++++++++++
{0}    ++++++++++++++++++++++++++++++++
{0} +++++++++++++{2}************{0}+++++++++++++
{0}+++++++++++{2}******************{0}++++++++{1}:::
{0}+++++++++{2}**********************{0}++{1}:::::::
{0}++++++++{2}*********{0}++++++{2}******{1}:::::::::::
{0}+++++++{2}********{0}++++++++++{2}**{1}:::{2}**{1}:::{2}**{1}:::
{0}+++++++{2}*******{0}+++++++++{1}::::::{2}*********{1}::
{0}+++++++{2}******{0}+++++++{1}::::::::::{2}**{1}:::{2}**{1}:::
{0}+++++++{2}*******{0}+++{1}::::::::::::{2}*********{1}::
{0}+++++++{2}********{1}::::::::::{2}**{1}:::{2}**{1}:::{2}**{1}:::
{0}++++++++{2}*********{1}::::::{2}******{1}:::::::::::
{0}++++++{1}:::{2}**********************{1}:::::::::
{0}+++{1}::::::::{2}******************{1}:::::::::::
{1} :::::::::::::{2}************{1}:::::::::::::
{1}    ::::::::::::::::::::::::::::::::
{1}       ::::::::::::::::::::::::::
{1}          ::::::::::::::::::::
{1}              ::::::::::::
{1}                 ::::::
```

Remarks:

- The ascii logo's dimensions must fall below `25*40` (height\*width); The CI will fail otherwise.
- Use `{i}` to color the ascii with `i` the color index from the `vec!` of colors defined in `define_language!`.
- Make sure to trim any unnecessary trailing whitespaces.
- Check [Convert image to ASCII art](https://github.com/o2sh/onefetch/wiki/image-to-ascii) to learn more about image to ascii conversion.
- Optionally, you may provide a `vec!` of colors in `rgb` format as an alternative to basic colors for terminals that support [true colour](https://gist.github.com/XVilka/8346728).

### Adding support for a new package manager

To add a new package manager, make sure to follow these steps:

1. Add a new entry in the `define_package_managers!` macro in [package_manager.rs](src/onefetch/package_managers/package_manager.rs).

**Example**:

`{ Cargo, "cargo", [ ("Cargo.toml", cargo) ] },`

The first item `Cargo` corresponds to the name of the package manager. The second item `cargo` is the display name. Then we have the name of the package manager file that will be parsed: `Cargo.toml` along with its parser `cargo` (cf. step 2), notice that the third item is an array of tuple in case the package manager has multiple parsers (e.g. pip).

2. in `src/onefetch/deps/package_parser.rs`: create a function that takes an input of type `&str` representing the content of the package manager's file, and returns a `usize` as its number of dependencies.

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
- [@spenserblack](https://github.com/spenserblack), [@CephalonRho](https://github.com/CephalonRho), [@ebroto](https://github.com/ebroto), [@erikgaal](https://github.com/erikgaal), [@yoichi](https://github.com/yoichi) and [@HallerPatrick](https://github.com/HallerPatrick) for maintaining this project.
- Everyone I forgot to mention here, but also influenced onefetch and helped it grow.

<p align="center">:heart::heart:</p>

<p align="center">
  <img src="https://contrib.rocks/image?repo=o2sh/onefetch" />
</p>

Made with [contributors-img](https://contrib.rocks).
