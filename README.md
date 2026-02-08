<div align="center">
  
# Onefetch - Command-line Git information tool

<p><img src="assets/onefetch.svg" height=100px></p>

[![Crates.io Version](https://img.shields.io/crates/v/onefetch)](https://crates.io/crates/onefetch)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/o2sh/onefetch/ci.yml)](https://github.com/o2sh/onefetch/actions/workflows/ci.yml)
[![help wanted](https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green)](https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
![MSRV](assets/msrv-badge.svg)

<h3>

[Homepage](https://onefetch.dev/) | [Installation](https://github.com/o2sh/onefetch/wiki/Installation) | [Documentation](https://github.com/o2sh/onefetch/wiki/)

</h3>

</div>

---

|  |  |
|---|---|
| ![Screenshot 1](assets/screenshot-1.png) | ![Screenshot 2](assets/screenshot-2.png) |

Onefetch is a command-line Git information tool written in `Rust` that displays project information and code statistics for a local Git repository directly in your terminal.

The tool works completely offline with a focus on performance and customizability.

By default, repository information is shown alongside the dominant language’s ASCII logo, but you can configure onefetch to display an image on [supported terminals](https://github.com/o2sh/onefetch/wiki/images-in-the-terminal).

It automatically detects open-source licenses from their text and provides valuable information such as language distribution, pending changes, dependency counts (per package manager), top contributors (by number of commits), disk usage, creation date, lines of code (LOC), and more.

Onefetch can be customized via [command-line options](https://github.com/o2sh/onefetch/wiki/command-line-options) to display exactly what you want, the way you want it: adjust the text styling, disable info lines, ignore files and directories, output in multiple formats (JSON, YAML), etc.

Currently, onefetch supports more than [100 different programming languages](https://onefetch.dev); if your language of choice isn't supported: Open an issue and support will be added.

Contributions are very welcome! See [CONTRIBUTING](CONTRIBUTING.md) for more info.
