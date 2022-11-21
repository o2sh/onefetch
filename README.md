<h3 align="center"><img src="assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Command-line Git information tool written in Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="assets/msrv-badge.svg">
</p>

<p align="center">
  <a href="docs/README.ja.md"><img title="日本語" alt="日本語" src="assets/flags/jp.svg" height="18"></a>&nbsp;
  <a href="docs/README.fa.md"><img title="فارسی" alt="فارسی" src="assets/flags/ir.svg" height="18"></a>&nbsp;
  <a href="docs/README.cn.md"><img title="简体中文" title="简体中文" src="assets/flags/cn.svg" height="18"></a>&nbsp;
  <a href="docs/README.ru.md"><img title="Русский" title="Русский" src="assets/flags/ru.svg" height="18"></a>&nbsp;
  <a href="docs/README.es.md"><img title="Español" title="Español" src="assets/flags/es.svg" height="18"></a>&nbsp;
  <a href="docs/README.fr.md"><img title="Français" title="Français" src="assets/flags/fr.svg" height="18"></a>
  <a href="docs/README.ko.md"><img title="한국어" alt="한국어" src="assets/flags/ko.svg" height="18"></a>
</p>

<img src="assets/screenshot-1.png" align="right" height="240px">

Onefetch is a command-line Git information tool written in `Rust` that displays project information and code statistics for a local Git repository directly to your terminal. The tool is completely offline - no network access is required.

By default, the repo's information is displayed alongside the dominant language's logo, but you can further configure onefetch to instead use an image - on supported terminals -, a text input or nothing at all.

It automatically detects open source licenses from texts and provides the user with valuable information like code distribution, pending changes, number of dependencies (by package manager), top contributors (by number of commits), size on disk, creation date, LOC (lines of code), etc.

<img src="assets/screenshot-2.png" align="right" height="240px">

Onefetch can be configured via command-line flags to display exactly what you want, the way you want it to: you can customize ASCII/Text formatting, disable info lines, ignore files & directories, output in multiple formats (Json, Yaml), etc.

As of now, onefetch supports more than [50 different programming languages](https://onefetch.dev); if your language of choice isn't supported: Open up an issue and support will be added.

Contributions are very welcome! See [CONTRIBUTING](CONTRIBUTING.md) for more info.

### More: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Getting Started](https://github.com/o2sh/onefetch/wiki/getting-started)\]
