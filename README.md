<div align="center">

<img src="assets/onefetch.svg" height=120px>

<h5>Command-line Git information tool written in Rust</h5>

[![cargo](https://img.shields.io/crates/v/onefetch.svg)](https://crates.io/crates/onefetch)
[![Build Status](https://github.com/o2sh/onefetch/workflows/CI/badge.svg)](https://github.com/o2sh/onefetch/actions)
[![help wanted](https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green)](https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
![MSRV](assets/msrv-badge.svg)

<h3>

[Wiki](https://github.com/o2sh/onefetch/wiki) | [Installation](https://github.com/o2sh/onefetch/wiki/Installation) | [Getting Started](https://github.com/o2sh/onefetch/wiki/getting-started)

</h3>

</div>

---

<img src="assets/screenshot-1.png" align="right" height="280px">

Onefetch is a command-line Git information tool written in `Rust` that displays project information and code statistics for a local Git repository directly to your terminal. The tool is completely offline - no network access is required.

By default, the repo's information is displayed alongside the dominant language's logo, but you can further configure onefetch to instead use an image - on supported terminals -, a text input or nothing at all.

It automatically detects open source licenses from texts and provides the user with valuable information like language distribution, pending changes, number of dependencies (by package manager), top contributors (by number of commits), size on disk, creation date, LOC (lines of code), etc.

<img src="assets/screenshot-2.png" align="right" height="280px">

Onefetch can be configured via command-line flags to display exactly what you want, the way you want it to: you can customize ASCII/Text formatting, disable info lines, ignore files & directories, output in multiple formats (Json, Yaml), etc.

As of now, onefetch supports more than [100 different programming languages](https://onefetch.dev); if your language of choice isn't supported: Open up an issue and support will be added.

Contributions are very welcome! See [CONTRIBUTING](CONTRIBUTING.md) for more info.

_Artwork by @Kuvshinov_Ilya_
