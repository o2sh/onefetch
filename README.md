<h3 align="center"><img src="assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Command-line Git information tool written in Rust</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22%E2%9D%93+help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/%E2%9D%93%20help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="assets/msrv-badge.svg">
</p>

<p align="center">
  <a href="docs/README.ja.md">日本語</a> | <a href="docs/README.fa.md">فارسی</a> |
  <a href="docs/README.cn.md">简体中文</a> | <a href="docs/README.ru.md">Русский</a> |
  <a href="docs/README.es.md">Español</a> | <a href="docs/README.fr.md">Français</a> |
  <a href="docs/README.kr.md">한국어</a> | <a href="docs/README.ar.md">العربية</a> |
  <a href="docs/README.tr.md">Türkçe</a>
</p>

<img src="assets/screenshot-1.png" align="right" height="250px">

Onefetch is a command-line Git information tool written in `Rust` that displays project information and code statistics for a local Git repository directly to your terminal. The tool is completely offline - no network access is required.

By default, the repo's information is displayed alongside the dominant language's logo, but you can further configure onefetch to instead use an image - on supported terminals -, a text input or nothing at all.

It automatically detects open source licenses from texts and provides the user with valuable information like code distribution, pending changes, number of dependencies (by package manager), top contributors (by number of commits), size on disk, creation date, LOC (lines of code), etc.

<img src="assets/screenshot-2.png" align="right" height="250px">

Onefetch can be configured via command-line flags to display exactly what you want, the way you want it to: you can customize ASCII/Text formatting, disable info lines, ignore files & directories, output in multiple formats (Json, Yaml), etc.

As of now, onefetch supports more than [100 different programming languages](https://onefetch.dev); if your language of choice isn't supported: Open up an issue and support will be added.

Contributions are very welcome! See [CONTRIBUTING](CONTRIBUTING.md) for more info.

### More: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[Installation](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[Getting Started](https://github.com/o2sh/onefetch/wiki/getting-started)\]
