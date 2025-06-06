# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2.24.0 (2025-04-12)

### New Features

- add language support for Lean by @foxyseta in https://github.com/o2sh/onefetch/pull/1509
- add language support for Typst by @foxyseta in https://github.com/o2sh/onefetch/pull/1508
- add language support for Razor by @SrS2225a in https://github.com/o2sh/onefetch/pull/1521

### Chores

- more idiomatic way to fetch HEAD refs by @o2sh in https://github.com/o2sh/onefetch/pull/1515
- more idiomatic way to fetch repository remote URL by @o2sh in https://github.com/o2sh/onefetch/pull/1516
- update holyc language logo by @o2sh in https://github.com/o2sh/onefetch/pull/1543
- update wiki powershell-snippet by @FallenDeity in https://github.com/o2sh/onefetch/pull/1542
- add nix local setup @Sk7Str1p3 in https://github.com/o2sh/onefetch/pull/1549

## 2.23.1 (2025-01-01)

### Bug Fixes

- Fix version in man page

## 2.23.0 (2025-01-01)

### New Features

- add language support for OpenSCAD by @kenchou in https://github.com/o2sh/onefetch/pull/1502
- add language support for Modelica by @dietmarw in https://github.com/o2sh/onefetch/pull/1262
- add language support for ATS by @pykenny in https://github.com/o2sh/onefetch/pull/523
- add language support for CUDA by @jtmr05 in https://github.com/o2sh/onefetch/pull/940
- add missing nerd fonts icons for some languages by @ankddev in https://github.com/o2sh/onefetch/pull/1491

### Chores

- add Italian translation of README by @tlazzarin in https://github.com/o2sh/onefetch/pull/1435
- add Polish translation of README by @adamperkowski in https://github.com/o2sh/onefetch/pull/1444
- add Czech translation of READEME by @Amereyeu in https://github.com/o2sh/onefetch/pull/1439
- update russian README by @ankddev in https://github.com/o2sh/onefetch/pull/1478
- [onefetch.dev] migrate to Svelte v5 by @o2sh in https://github.com/o2sh/onefetch/pull/1455
- add script to preview/validate Nerd Fonts by @spenserblack in https://github.com/o2sh/onefetch/pull/1492
- add Powershell snippet to run onefetch automatically by @kiapanahi in https://github.com/o2sh/onefetch/pull/1453

## 2.22.0 (2024-09-20)

### New Features

- Add support for nerd font glyphs in languages info by @Localghost385 in https://github.com/o2sh/onefetch/pull/1395
- [onefetch.dev] Add nerdfont iconts to the preview by @Localghost385 in https://github.com/o2sh/onefetch/pull/1411
- Automate publishing crates to crates.io by @musicinmybrain in https://github.com/o2sh/onefetch/pull/1364

### Bug Fixes

- Show future commit dates without panicking by @MalteT in https://github.com/o2sh/onefetch/pull/1389

### Chores

- Re-generate the man page with --no-info by @musicinmybrain in https://github.com/o2sh/onefetch/pull/1376
- Drop unused shebangs from repo test fixture scripts by @musicinmybrain in https://github.com/o2sh/onefetch/pull/1375

## 2.21.0 (2024-05-08)

### New Features

- Add CLI option to force URL format to HTTP instead of SSH by @0spotter0 in https://github.com/o2sh/onefetch/pull/1314
- Add CLI flag to hide token from repository URL by @o2sh in https://github.com/o2sh/onefetch/pull/1319
- Make Lua logo more readable on dark terminal by @o2sh in https://github.com/o2sh/onefetch/pull/1337

### Bug Fixes

- Fix deadlock in Churn computation by @Nettifani in https://github.com/o2sh/onefetch/pull/1316
- Exclude bot commits from churn when `--no-bots` option is used by @o2sh in https://github.com/o2sh/onefetch/pull/1335

### Chores

- [onefetch.dev] switch to dark theme by @o2sh in https://github.com/o2sh/onefetch/pull/1297
- RUSTSEC-2024-0320: remove yaml-rust dependency by @Suyun114 in https://github.com/o2sh/onefetch/pull/1309
- Refactor `--no-bots` CLI option by @o2sh in https://github.com/o2sh/onefetch/pull/1340

## 2.20.0 (2024-03-17)

This version marks the completion of the transition from [`git2`](https://crates.io/crates/git2) to [`gitoxide`](https://crates.io/crates/gix). No more dependency to git2, onefetch is now fully oxidized!

### New Features

- Add svg language support by @Localghost385 in https://github.com/o2sh/onefetch/pull/1266
- lang: Adding Oz programming language by @luxluth in https://github.com/o2sh/onefetch/pull/1280

### Chores

- website: Filter entries by language type in onefetch.dev by @o2sh in https://github.com/o2sh/onefetch/pull/1227
- Use GitHub's alert syntax by @spenserblack in https://github.com/o2sh/onefetch/pull/1234
- Add german translation of `README.md` by @rdwz in https://github.com/o2sh/onefetch/pull/1236
- Use `gitoxide` to get pending changes by @Byron in https://github.com/o2sh/onefetch/pull/1285

## 2.19.0 (2023-11-29)

### New Features

- exclude files from churn by @o2sh in https://github.com/o2sh/onefetch/pull/1120
- add odin support by @spsandwichman in https://github.com/o2sh/onefetch/pull/1064
- New language: Arduino by @Sh4rk-Byte in https://github.com/o2sh/onefetch/pull/1176
- Right align authorship percentages by @lukehsiao in https://github.com/o2sh/onefetch/pull/1207
- Add Agda to languages.yaml by @Zoltan-Balazs in https://github.com/o2sh/onefetch/pull/1216

### Bug Fixes

- add a test for negative dates and see how onefetch handles it by @Byron in https://github.com/o2sh/onefetch/pull/1100

### Chores

- Group clap dependency updates by @spenserblack in https://github.com/o2sh/onefetch/pull/1101
- Group all NPM dependency updates by @spenserblack in https://github.com/o2sh/onefetch/pull/1110
- Added Turkish Translations by @4Chaffenel in https://github.com/o2sh/onefetch/pull/1135
- use workspace inheritance by @o2sh in https://github.com/o2sh/onefetch/pull/1142
- docs(contributing): Add syntax highlighting to YAML block by @spenserblack in https://github.com/o2sh/onefetch/pull/1172
- add release.yml file by @o2sh in https://github.com/o2sh/onefetch/pull/1177
- replace action-rs by @o2sh in https://github.com/o2sh/onefetch/pull/1191
- Resolve clippy warnings by @spenserblack in https://github.com/o2sh/onefetch/pull/1201
- Refactor and test info field styling by @spenserblack in https://github.com/o2sh/onefetch/pull/1214
- Refactoring git metrics module by @o2sh in https://github.com/o2sh/onefetch/pull/1217

### Dependencies

- upgrade to `gix` 0.53.1 by @Byron in https://github.com/o2sh/onefetch/pull/1166

## 2.18.1 (2023-06-25)

### Bug Fixes

- don't fail when computing diff on partial clones (#1093) @Byron @o2sh

### Features

- fetch banner info from github (#1094) @spenserblack @o2sh

## 2.18.0 (2023-06-20)

### Features

- add new info line called "Churn" which displays the files with the most modifications (commits) (#1071) @o2sh @Byron
- add Hlsl support (#1082) @progDes007

### Other

- add info builder pattern

### Chore

- performance: optimize case where repo has a commit-graph for massive performance gains (#1081) @Byron
- docs: add a cmd.exe version of the cd snippet (#1048) @mataha
- refacto: use the builder pattern to instantiate the `Info` struct (#1047) @o2sh @spenserblack
- improve bot regex (#1086) @o2sh @spenserblack

## 2.17.1 (2023-04-28)

### Other

- Improve code coverage of src/info/mod.rs (#1011) @changhc
- Improve code coverage of src/ui/mod.rs (#1012) @changhc
- Added fish git repository greeter script to wiki (#1021) @TheSast
- upgrade gitoxide to v0.44 (and incorporate #1023):x (#1024) @Byron @spenserblack

## 2.17.0 (2023-04-09)

### Other

- Disable line wrap (#983) @o2sh
- Add Pascal support (#989) @rchastain
- Add Coldfusion support (#971) @theemanofsteele
- Remove github token from url field (#996) @jim4067
- Changed Hashbang (#979) @gautamprikshit1
- Prevent conflicts in wiki action 39fe441 @spenserblack
- Fix typos (#992) @hezhizhen
- Group CLI options in sections (#995) @o2sh
- replace --show-logo with --no-art (#1002) @o2sh
- Set snapshot language to plain text (#1003) @spenserblack
- Better error message when human_time panics (#1010) @o2sh

### New Features

- remove github token from url field

## 2.16.0 (2023-02-24)

### Other

- Add GLSL language support #490 (#824) @sangsatori
- Fix Markdown / Jupyter markup not getting counted (#937) @spenserblack
- upgrade gix to 0.36.1 to avoid breakage. (#965) @Byron
- Fix path to language template (#939) @spenserblack
- Create the Arabic README file (Arabic translation) (#950) @anas-elgarhy
- Refactoring of info/langs/mod.rs (#948) @o2sh
- Remove country flags #928 @o2sh
- Upgrade git-repository 0.30 to gix 0.36 (#963) @Byron

## 2.15.1 (2023-01-19)

### Other

- Fix CD Github action @o2sh

## 2.15.0 (2023-01-19)

### Other

- Add --number-separator CLI flag #892 @o2sh
- Add Makefile language support #867 @ozwaldorf
- Vercel: add section links #922 @ozwaldorf
- Add gitpod.io configuration #881 @spenserblack
- Use human_panic #887 @o2sh
- Read license from manifest first #769 @o2sh
- Install cargo-insta in dev containers #909 @spenserblack
- Info struct to holds a Vec #911 @o2sh
- Add benchmark #912 @o2sh
- GH action to synchronize wiki with .github/wiki #926 @spenserblack @o2sh
- Clean up greeter and fix repository detection mechanism in wiki #927 @quazar-omega
- Turn AsciiArt.rs into its own crate #934 @o2sh
- Use ISO time for snapshot tests #908 @spenserblack
- Parse multi-byte unicode chars correctly + docs #936 @ozwaldorf

## 2.14.2 (2022-11-27)

### Other

- Include assets in crate a2f508a @o2sh
- Fix clap deps for onefetch-image crate 8cca7af
- Add description field to onefetch-image and onefetch-manifest crate 2888186 @o2sh

## 2.14.1 (2022-11-27)

### Other

- Fix CD 5085c5b @o2sh

## 2.14.0 (2022-11-27)

### Other

- Add description info line #851 @o2sh
- Add CLI flag to set the maximum number of languages to be shown 8159b34 @o2sh
- Add VisualBasic language support #867 @antonizyla
- Add manifest crate #851 @o2sh @spenserblack
- Move image_backends into its own crate 9ce17c1 @o2sh
- Add devcontainer/codespace config #857 @spenserblack
- Switch to Swatinem/rust-cache for caching 7592eb2 @o2sh
- Add README translation for Korean #869 @abiriadev
- add icon to windows exe 584574f @o2sh
- Fix typo in help message for -e (--exclude) #861 @skogseth

## 2.13.2 (2022-10-30)

### Other

- [fix] Repo without remote should not fail #841 @o2sh
- [chore] Add integration tests with snapshot testing for Info struct #827 @atluft
- [chore] Refactor test expressions #831 @saguywalker

## 2.13.1 (2022-10-22)

### Other

- [ci/cd] fix Snapcraft release
- [misc] fix Cargo.lock

## 2.13.0 (2022-10-21)

`onefetch` is now typically more than twice as fast when executing. This was achieved by reducing
allocations and switching to `gitoxide` for the most taxing tasks.

A new web interface [onefetch.dev](https://onefetch.dev) was developed where users can visualize an ASCII preview for all the programming languages currently supported by onefetch. Like the binary, the data is parsed from the `Languages.yaml` file.

### Other

- [chore] reducing allocations and switching to gitoxide from libgit2 #635 @Byron
- [docs] add README translation for Spanish #631 @JakeRoggenbuck @practicatto
- [docs] add Changelog generated using cargo-smart-release #637 @Byron
- [cli] add --completion option #657 @spenserblack
- [language] update PHP colors #664 @DenverCoder1
- [misc] switch to actions/stale #666 @spenserblack @o2sh
- [misc] add github issue forms #667 @spenserblack @o2sh
- [ci/cd] generate Windows installer from CD #668 @o2sh
- [ci/cd] create WinGet workflow for auto publishing #673 @russellbanks
- [language] update logo: shell #677 @fux0c1ety
- [docs] adding french documentation support #693 @Kaderovski
- [chore] extract language definitions into data file #699 @spenserblack
- [ci/cd] add codecov + tarpaulin in ci @o2sh
- [misc] create Vercel app for onefetch with ASCII preview #701 @spenserblack
- [docs] update the README in Russian #736 @AndreyKozhev
- [chore] turn InfoField into a trait (big refactoring) #755 @o2sh
- [language] Improve JSX ASCII logo #784 @alessandroasm
- [language] Improve TSX ASCII logo #785 @alessandroasm
- [language] added support for verilog #789 @atluft
- [language] improve ruby logo #786 @atluft
- [language] added support for xsl #798 @atluft
- [language] added support for systemverilog #797 @atluft
- [test] add unit tests to src/info/info_field.rs #810 @alessandroasm
- [ci/cd] automate publish to crates.io #800 @spenserblack
- [language] added support for ABNF #811 @atluft
- [test] add unit tests src/info/repo/commits.rs #813 @alessandroasm
- [test] add unit tests src/info/repo/contributors.rs #814 @alessandroasm
- [language] added support for ABAP #821@atluft
- [test] testing get_git_username using git-testtools for #812 @atluft
- [language] improve bash logo @o2sh
- [language] improve assembly logo @o2sh
- [test] add unit tests for author.rs #829 @gallottino @Oniryu95

### Fixes

- Commits replaced with `git replace` are now followed. This can be turned off by setting the
  `GIT_NO_REPLACE_OBJECTS` environment variable.
- Shallow clones are now detected and displayed as such. Previously it might have appeared that
  the commit count is the real even though it was truncated due to the shallow-ness of the
  repository.
  If a repository is shallow, `(shallow)` will appear after the commit count.

### Refactor

- git2 repository can now be owned by the `Repo` type
  Previously this wasn't possible as commits would be kept in `Repo`
  which would cause self-referential borrow check issues unless
  the git2 repository was kept outside.
- completely separate `Commits` and `Repo` structure
- put all commit-traversal related initialization into own struct
