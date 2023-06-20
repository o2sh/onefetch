<h3 align="center"><img src="../assets/onefetch.svg" height="130px"></h3>

<h5 align="center">Rust编写的一款命令行Git信息工具</h5>

<p align="center">
	<a href="https://crates.io/crates/onefetch"><img src="https://img.shields.io/crates/v/onefetch.svg" alt="cargo"></a>
	<a href="https://github.com/o2sh/onefetch/actions"><img src="https://github.com/o2sh/onefetch/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://onefetch.dev"><img src="../assets/language-badge.svg"></a>
	<a href="https://github.com/o2sh/onefetch/issues?q=is%3Aissue+is%3Aopen+label%3A%22%E2%9D%93+help+wanted%22"><img src="https://img.shields.io/github/issues/o2sh/onefetch/%E2%9D%93%20help%20wanted?color=green" alt="help wanted"></a>
	<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
	<img src="../assets/msrv-badge.svg">
</p>

<img src="../assets/screenshot-1.png" align="right" height="250px">

Onefetch 是一款由 `Rust` 编写的命令行 Git 信息工具，它将直接在终端中展示本地 Git 仓库的项目详情和代码统计等内容。工具完全离线可用（不需要网络连接)。<br><br>

默认情况下，仓库信息显示在主要语言 logo 的旁边，但是您还可以进一步配置 onefetch 以使用图像（在支持的终端上）、文本输入或什么都不展示。<br><br>

它会自动从文本中检测开源许可证，并为用户提供有价值的信息，例如代码分发、pending、依赖数量（使用包管理器）、主要贡献者（按提交次数）、占用磁盘大小、创建日期、LOC（代码行数）等。<br><br>

<img src="../assets/screenshot-2.png" align="right" height="250px">

Onefetch 可以通过命令行参数进行配置，以准确显示您想要的内容和方式：您可以自定义 ASCII/文本格式、禁用信息行、忽略文件和目录、以多种格式输出（Json、Yaml），等等。<br><br>

截至目前，onefetch 支持超过 100 种不同的编程语言；如果您选择的语言不受支持：创建一个 issue，我们将尽快适配。<br><br>

欢迎大家一起来维护项目！详见 [CONTRIBUTING](../CONTRIBUTING.md).<br><br>

### 更多: \[[Wiki](https://github.com/o2sh/onefetch/wiki)\] \[[安装](https://github.com/o2sh/onefetch/wiki/Installation)\] \[[开始使用](https://github.com/o2sh/onefetch/wiki/getting-started)\]
