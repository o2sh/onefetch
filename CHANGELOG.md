# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2.18.1 (2023-06-24)

### Bug Fixes

- don't fail when computing diff on partial clones (#1093) @Byron @o2sh

### Features

- fetch banner info from github (#1094) @spenserblack @o2sh

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 4 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#1093](https://github.com/o2sh/onefetch/issues/1093)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#1093](https://github.com/o2sh/onefetch/issues/1093)**
  - Don't fail when computing diff on partial clones ([`89dcc8f`](https://github.com/o2sh/onefetch/commit/89dcc8fa3e0d2c1f61deb1bcf911c34915df0061))
- **Uncategorized** - Rename fixture scripts ([`e35e106`](https://github.com/o2sh/onefetch/commit/e35e1065c5bf079dec1dce4b315127db4812ef53))
</details>

## 2.18.0 (2023-06-20)

### Features

- add new info line called "Churn" which displays the files with the most modifications (commits) (#1071) @o2sh @Byron
- add Hlsl support (#1082) @progDes007

### Chore

- performance: optimize case where repo has a commit-graph for massive performance gains (#1081) @Byron
- docs: add a cmd.exe version of the cd snippet (#1048) @mataha
- refacto: use the builder pattern to instantiate the `Info` struct (#1047) @o2sh @spenserblack
- improve bot regex (#1086) @o2sh @spenserblack

### Commit Statistics

<csr-read-only-do-not-edit/>

- 14 commits contributed to the release over the course of 40 calendar days.
- 53 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 4 unique issues were worked on: [#1047](https://github.com/o2sh/onefetch/issues/1047), [#1071](https://github.com/o2sh/onefetch/issues/1071), [#1081](https://github.com/o2sh/onefetch/issues/1081), [#1086](https://github.com/o2sh/onefetch/issues/1086)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#1047](https://github.com/o2sh/onefetch/issues/1047)**
  - Add info builder pattern ([`8609f92`](https://github.com/o2sh/onefetch/commit/8609f921ab43f0e64cb0aeb831d92f157a5b0ed3))
- **[#1071](https://github.com/o2sh/onefetch/issues/1071)**
  - Added File Churn Metric ([`1955153`](https://github.com/o2sh/onefetch/commit/1955153a9f998780b78bc391df1a94464c57cc80))
- **[#1081](https://github.com/o2sh/onefetch/issues/1081)**
  - Gix upgrade and optimizations ([`09c4dc9`](https://github.com/o2sh/onefetch/commit/09c4dc9df6e1201eeed54b01f3432f4097ff54dc))
- **[#1086](https://github.com/o2sh/onefetch/issues/1086)**
  - Improve bot regex ([`167b428`](https://github.com/o2sh/onefetch/commit/167b428794c86eb16a415c7da9869f209a3758e1))
- **Uncategorized** - Extract total_number_of_commits from author map ([`086ed9e`](https://github.com/o2sh/onefetch/commit/086ed9e1272f411172d3dd51824caa0285413907)) - Fix help message for churn_pool_size ([`3e19824`](https://github.com/o2sh/onefetch/commit/3e198246b5afb48d4a30be532a9bd5a67cf9af87)) - Add unit tests for break_sentence_into_lines ([`fd1d5f0`](https://github.com/o2sh/onefetch/commit/fd1d5f04bbf1fbf160506fad4a05dd2b1dd35fd9)) - Re-use info field value in display method ([`2c9706b`](https://github.com/o2sh/onefetch/commit/2c9706b21920330ae67d4b27b5663d5b9ed957f4)) - Compute info_field.value() only once to check if empty ([`b990786`](https://github.com/o2sh/onefetch/commit/b99078628dd4b24ff196703d6ae5889db2f1409d)) - Remove should_color() ([`bb85a9c`](https://github.com/o2sh/onefetch/commit/bb85a9cafe05e9b9610e390f53ee5b6855a9bed3)) - Rename variable ([`43e93fa`](https://github.com/o2sh/onefetch/commit/43e93fa27c66baa261c5e41b36fc6f947f99dce5)) - Cargo clippy pedantic ([`444dcaa`](https://github.com/o2sh/onefetch/commit/444dcaa8a4a48c0895baa5afc6f3d2d2c11c0f46)) - Cargo clippy pedantic ([`ca1db57`](https://github.com/o2sh/onefetch/commit/ca1db57fd3e60656dd83993dc0a7ba6e8f0a6469)) - Cargo clippy pedantic ([`0654eb9`](https://github.com/o2sh/onefetch/commit/0654eb957ffe50a4b1e12cf53569862a68294b35))
</details>

## 2.17.1 (2023-04-28)

### Other

- Improve code coverage of src/info/mod.rs (#1011) @changhc
- Improve code coverage of src/ui/mod.rs (#1012) @changhc
- Added fish git repository greeter script to wiki (#1021) @TheSast
- upgrade gitoxide to v0.44 (and incorporate #1023):x (#1024) @Byron @spenserblack

### Commit Statistics

<csr-read-only-do-not-edit/>

- 6 commits contributed to the release over the course of 19 calendar days.
- 19 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 3 unique issues were worked on: [#1011](https://github.com/o2sh/onefetch/issues/1011), [#1012](https://github.com/o2sh/onefetch/issues/1012), [#1024](https://github.com/o2sh/onefetch/issues/1024)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#1011](https://github.com/o2sh/onefetch/issues/1011)**
  - Improve code coverage of src/info/mod.rs ([`050568f`](https://github.com/o2sh/onefetch/commit/050568f1d346c4c398711ace93b9ee6759349892))
- **[#1012](https://github.com/o2sh/onefetch/issues/1012)**
  - Improve code coverage of src/ui/mod.rs ([`31b813f`](https://github.com/o2sh/onefetch/commit/31b813f695781e0d9820bbe833aec4a8613e3904))
- **[#1024](https://github.com/o2sh/onefetch/issues/1024)**
  - Upgrade gitoxide to v0.44 (and incorporate #1023):x ([`7be7ea9`](https://github.com/o2sh/onefetch/commit/7be7ea92a29117706af73f4ff9799a9a5a3d76b1))
- **Uncategorized** - Add unit tests to git.rs ([`793de44`](https://github.com/o2sh/onefetch/commit/793de44f5b1aa81bcc2a6e2f34ce3d2982b10556)) - Better property name ([`e38c6d0`](https://github.com/o2sh/onefetch/commit/e38c6d041d7fbec1034d56c41e9e6ac6e3e6f494)) - Cleaning code in git.rs ([`282e8d1`](https://github.com/o2sh/onefetch/commit/282e8d175655293e53ab9e4be20488b836bade56))
</details>

## 2.17.0 (2023-04-08)

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

### Commit Statistics

<csr-read-only-do-not-edit/>

- 10 commits contributed to the release over the course of 21 calendar days.
- 43 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 6 unique issues were worked on: [#1002](https://github.com/o2sh/onefetch/issues/1002), [#1010](https://github.com/o2sh/onefetch/issues/1010), [#983](https://github.com/o2sh/onefetch/issues/983), [#992](https://github.com/o2sh/onefetch/issues/992), [#995](https://github.com/o2sh/onefetch/issues/995), [#996](https://github.com/o2sh/onefetch/issues/996)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#1002](https://github.com/o2sh/onefetch/issues/1002)**
  - Replace --show-logo with --no-art ([`20f4390`](https://github.com/o2sh/onefetch/commit/20f43906ea316bfa4a5e1e80361fb4df14df4991))
- **[#1010](https://github.com/o2sh/onefetch/issues/1010)**
  - Better panic message ([`727cf70`](https://github.com/o2sh/onefetch/commit/727cf7008aab976136b3f29bf4b0782cf65f73b1))
- **[#983](https://github.com/o2sh/onefetch/issues/983)**
  - Disable line wrap ([`71ea9c1`](https://github.com/o2sh/onefetch/commit/71ea9c16b797d9ea7541b40eedabe27982bcadea))
- **[#992](https://github.com/o2sh/onefetch/issues/992)**
  - Fix typo ([`fa80f33`](https://github.com/o2sh/onefetch/commit/fa80f3308ff6f19a3c62233ecbc9767ffa7b9ac9))
- **[#995](https://github.com/o2sh/onefetch/issues/995)**
  - Add help sections ([`240fae7`](https://github.com/o2sh/onefetch/commit/240fae73575e3c0643292daa9602bba9b2c95e6d))
- **[#996](https://github.com/o2sh/onefetch/issues/996)**
  - Remove github token from url field ([`6d448ce`](https://github.com/o2sh/onefetch/commit/6d448ce89a30154411ee04e3fbd67485f1a7a667))
- **Uncategorized** - Update manpage ([`d7773c9`](https://github.com/o2sh/onefetch/commit/d7773c98a22ca3ab871696cf90f2fe55b6c8f611)) - Use pascal case for clap help sections ([`20ca8cd`](https://github.com/o2sh/onefetch/commit/20ca8cd6a2509ced9c766b5d5da5be00054082b4)) - Merge branch 'main' of github.com:o2sh/onefetch ([`e57965a`](https://github.com/o2sh/onefetch/commit/e57965a82e2c11a9b0108db52d37d288dc6de5cd)) - Move setup_panic at the top of the main function ([`406a844`](https://github.com/o2sh/onefetch/commit/406a844dfb77e10fc34ebd9d43577c69beaedc15))
</details>

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

### Commit Statistics

<csr-read-only-do-not-edit/>

- 10 commits contributed to the release over the course of 30 calendar days.
- 35 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 4 unique issues were worked on: [#937](https://github.com/o2sh/onefetch/issues/937), [#948](https://github.com/o2sh/onefetch/issues/948), [#963](https://github.com/o2sh/onefetch/issues/963), [#965](https://github.com/o2sh/onefetch/issues/965)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#937](https://github.com/o2sh/onefetch/issues/937)**
  - Fix Markdown / Jupyter markup not getting counted ([`5379ecd`](https://github.com/o2sh/onefetch/commit/5379ecd127d1217a7a9d469534490aa2d1961374))
- **[#948](https://github.com/o2sh/onefetch/issues/948)**
  - Refactoring of `info/langs/mod.rs` ([`fdf435d`](https://github.com/o2sh/onefetch/commit/fdf435d089487a7b8eea7c6875bf6a282db2e4a8))
- **[#963](https://github.com/o2sh/onefetch/issues/963)**
  - Upgrade `git-repository` 0.30 to `gix` 0.36 ([`7ec5240`](https://github.com/o2sh/onefetch/commit/7ec5240a4e746b7373c665bbd3f8f53eea8d5d1a))
- **[#965](https://github.com/o2sh/onefetch/issues/965)**
  - Upgrade `gix` to 0.36.1 to avoid breakage. ([`ce76a6d`](https://github.com/o2sh/onefetch/commit/ce76a6dc64e505eb660f090cd7b3070965d09aca))
- **Uncategorized** - #966, simplify \_\_stats_loc method ([`aa0412f`](https://github.com/o2sh/onefetch/commit/aa0412f0851dd2029f5a2da738cc73d2bff959e1)) - Cargo clippy ([`f142aa5`](https://github.com/o2sh/onefetch/commit/f142aa508cfd1251dcb41e89d323c2142ce28a64)) - Fix #966 ([`8947d43`](https://github.com/o2sh/onefetch/commit/8947d43d3e35d927716d829aa2dce354158e4973)) - Rename method ([`34a1717`](https://github.com/o2sh/onefetch/commit/34a1717e8772c300b885c6569601332018238559)) - Cargo clippy ([`0959021`](https://github.com/o2sh/onefetch/commit/095902190d4d6adf4548e203ffe3f5c680c08cd6)) - Simplify file structure ([`c7c30b4`](https://github.com/o2sh/onefetch/commit/c7c30b49d684ef0f933179ccd9188f644f41e29f))
</details>

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

### Commit Statistics

<csr-read-only-do-not-edit/>

- 15 commits contributed to the release over the course of 47 calendar days.
- 52 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 5 unique issues were worked on: [#892](https://github.com/o2sh/onefetch/issues/892), [#904](https://github.com/o2sh/onefetch/issues/904), [#907](https://github.com/o2sh/onefetch/issues/907), [#911](https://github.com/o2sh/onefetch/issues/911), [#934](https://github.com/o2sh/onefetch/issues/934)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#892](https://github.com/o2sh/onefetch/issues/892)**
  - Add `--number-separator` CLI flag ([`3cff2d2`](https://github.com/o2sh/onefetch/commit/3cff2d20bba4e2d7c3d09020d9df5cc7015aff1a))
- **[#904](https://github.com/o2sh/onefetch/issues/904)**
  - Bump git-repository from 0.29.0 to 0.30.2 ([`8806f26`](https://github.com/o2sh/onefetch/commit/8806f26ed482c8182c5db6a15da47348d890accf))
- **[#907](https://github.com/o2sh/onefetch/issues/907)**
  - Bump git-testtools ([`6cccb2c`](https://github.com/o2sh/onefetch/commit/6cccb2cf3a4efe42230208d778b74a14c2b044e7))
- **[#911](https://github.com/o2sh/onefetch/issues/911)**
  - Info struct to holds a Vec<InfoField> ([`4a3ef8c`](https://github.com/o2sh/onefetch/commit/4a3ef8c23d5ddcd264e9f421aaa8614bc6703d6d))
- **[#934](https://github.com/o2sh/onefetch/issues/934)**
  - Turn `AsciiArt.rs` into its own crate ([`1716519`](https://github.com/o2sh/onefetch/commit/17165192bf187fbd56298a53e34373d8329bfa3a))
- **Uncategorized** - Fix cli help text ([`71a2a87`](https://github.com/o2sh/onefetch/commit/71a2a87b057307afd44ed707c8796b109578e0d4)) - Cargo clippy ([`c6c6054`](https://github.com/o2sh/onefetch/commit/c6c60543c1d8b7cbe42f8ec48b3c84512585afa5)) - Remove test mod ([`b6e362e`](https://github.com/o2sh/onefetch/commit/b6e362e778fb6d07840f1b40b59940c3368149a3)) - Move function outside of Info struct ([`2f65efd`](https://github.com/o2sh/onefetch/commit/2f65efd4d70a3e079568366fd0b2fc05fd074459)) - Move no_bots logic inside git.rs ([`ff2c8f0`](https://github.com/o2sh/onefetch/commit/ff2c8f0ab170a3ad84c5a20ef50c9be9287e0cb5)) - Remove title from InfoType enum ([`31dc656`](https://github.com/o2sh/onefetch/commit/31dc656861751ec13a167a9af6aafe7575349a36)) - Cargo clippy ([`71bfde0`](https://github.com/o2sh/onefetch/commit/71bfde026b6aa2f3e9195fde12d6cff4afb4eff7)) - Remove useless function ([`3c876f6`](https://github.com/o2sh/onefetch/commit/3c876f64f1a232987018c97771b2a3f960972d91)) - #769 read license from manifest first ([`e2af4ec`](https://github.com/o2sh/onefetch/commit/e2af4eca3ee6556a5d5063ff80a64dc89cb974eb)) - Use human_panic #887 ([`975856d`](https://github.com/o2sh/onefetch/commit/975856d9215852f7687ab2bb25926ee14d6f0956))
</details>

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

### Commit Statistics

<csr-read-only-do-not-edit/>

- 15 commits contributed to the release over the course of 25 calendar days.
- 28 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 3 unique issues were worked on: [#851](https://github.com/o2sh/onefetch/issues/851), [#856](https://github.com/o2sh/onefetch/issues/856), [#861](https://github.com/o2sh/onefetch/issues/861)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#851](https://github.com/o2sh/onefetch/issues/851)**
  - Add manifest crate ([`ed96c45`](https://github.com/o2sh/onefetch/commit/ed96c4547d62e1660bd78663be428cad5dc57867))
- **[#856](https://github.com/o2sh/onefetch/issues/856)**
  - Upgrade gitoxide ([`78da9e8`](https://github.com/o2sh/onefetch/commit/78da9e812b674748c4c3e41d829752e0fbf46d1f))
- **[#861](https://github.com/o2sh/onefetch/issues/861)**
  - Fix typo in help message for -e (--exclude) ([`40ed5c3`](https://github.com/o2sh/onefetch/commit/40ed5c323cf3eba48fdb4a5b6a9a5fa74ac8e10c))
- **Uncategorized** - Add cli flag to set maximum number of languages to be shown #863 ([`8159b34`](https://github.com/o2sh/onefetch/commit/8159b34ce1eeb07b544f984ab43e35c449ae045f)) - Cargo clippy ([`73a3eb0`](https://github.com/o2sh/onefetch/commit/73a3eb00444ccf2f01a8e952c0810c5db47de00f)) - Move image_backends into its own crate ([`9ce17c1`](https://github.com/o2sh/onefetch/commit/9ce17c186f4b301b9af26fa2329ad54dda58e557)) - Remove unused imports ([`dcf49c7`](https://github.com/o2sh/onefetch/commit/dcf49c745ce8ef28b44b29df8e621e097f3a5e95)) - Remove flaky unit tests ([`fac95a6`](https://github.com/o2sh/onefetch/commit/fac95a6e103e5debcfa79d892a1b223271c79dd0)) - Try fix ci ([`e3c594c`](https://github.com/o2sh/onefetch/commit/e3c594c24b10f86dccbb82ade1f68a20cf5525e4)) - Cargo clippy ([`0fd014f`](https://github.com/o2sh/onefetch/commit/0fd014f8f92b9b60e0b8399254ba497a401cd534)) - Don't color info for authors and languages ([`2ec4f0c`](https://github.com/o2sh/onefetch/commit/2ec4f0c168b94e06625565a249850e12e12b181c)) - Replace with_context with context ([`6ce5677`](https://github.com/o2sh/onefetch/commit/6ce5677ead67a8b13acd3a6817cafe2a3536a8d7)) - Update signature of get_repo_name ([`32ab8d2`](https://github.com/o2sh/onefetch/commit/32ab8d25c4f7269efb76096f274a51b0abfc7f7e)) - Replace String::From with .into() ([`1ec711e`](https://github.com/o2sh/onefetch/commit/1ec711ed7a2a6145beb76fc4ec4c08657890623c)) - Fix help message for ascii-input ([`01ce421`](https://github.com/o2sh/onefetch/commit/01ce421c609c43a81b71f980b67815fd05b0b674))
</details>

## 2.13.2 (2022-10-30)

### Other

- [fix] Repo without remote should not fail #841 @o2sh
- [chore] Add integration tests with snapshot testing for Info struct #827 @atluft
- [chore] Refactor test expressions #831 @saguywalker

### Commit Statistics

<csr-read-only-do-not-edit/>

- 9 commits contributed to the release over the course of 7 calendar days.
- 8 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 3 unique issues were worked on: [#827](https://github.com/o2sh/onefetch/issues/827), [#831](https://github.com/o2sh/onefetch/issues/831), [#841](https://github.com/o2sh/onefetch/issues/841)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#827](https://github.com/o2sh/onefetch/issues/827)**
  - Add test info mod rs ([`a1f1987`](https://github.com/o2sh/onefetch/commit/a1f198712feddb693ebb5f4ba2985819ad2b2eff))
- **[#831](https://github.com/o2sh/onefetch/issues/831)**
  - Refactor test expressions ([`e94a273`](https://github.com/o2sh/onefetch/commit/e94a273c331712360b6f13c85b36710a5d83fccb))
- **[#841](https://github.com/o2sh/onefetch/issues/841)**
  - Repo without remote should not fail ([`e72f371`](https://github.com/o2sh/onefetch/commit/e72f371e504851f3f6edaab2b0dd73e99aa7fb18))
- **Uncategorized** - Cargo clippy ([`82ce8e9`](https://github.com/o2sh/onefetch/commit/82ce8e96b20474d99cfef5269075c280ef37ef24)) - Cargo clipppy ([`6b88ac7`](https://github.com/o2sh/onefetch/commit/6b88ac7abf0a190aeddf413fadb9259b7aaecace)) - Fix access modifiers ([`fdd17d3`](https://github.com/o2sh/onefetch/commit/fdd17d35a11ad89fcd7fb294b79e556bc48f7c60)) - Add comment ([`1f99437`](https://github.com/o2sh/onefetch/commit/1f99437d40b4414dbfd049bf2d10238fa602da0f)) - Move integration tests to separate folder ([`6b852c8`](https://github.com/o2sh/onefetch/commit/6b852c82fb5bfbb10990f31c97e7359f3dfed872)) - Merge branch 'main' of github.com:o2sh/onefetch ([`3a3b1c6`](https://github.com/o2sh/onefetch/commit/3a3b1c6c04907d5461deb6ed609a2543f5be3776))
</details>

## 2.13.1 (2022-10-22)

### Other

- [ci/cd] fix Snapcraft release
- [misc] fix Cargo.lock

## 2.13.0 (2022-10-21)

<csr-id-d43fa9acbbc93cfee2e59faf3652e7893de55ffa/>
<csr-id-5e4d02552beea1a998239360fe61b8465437884a/>
<csr-id-b6cd415d049b24348150e0e2088f2fdb5822e1cb/>
<csr-id-7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e/>
<csr-id-d00ab45d3cab26e6c8394c2952d7704dd58b8245/>

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

- <csr-id-b6cd415d049b24348150e0e2088f2fdb5822e1cb/> git2 repository can now be owned by the `Repo` type
  Previously this wasn't possible as commits would be kept in `Repo`
  which would cause self-referential borrow check issues unless
  the git2 repository was kept outside.
- <csr-id-7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e/> completely separate `Commits` and `Repo` structure
- <csr-id-d00ab45d3cab26e6c8394c2952d7704dd58b8245/> put all commit-traversal related initialization into own struct

### Commit Statistics

<csr-read-only-do-not-edit/>

- 87 commits contributed to the release over the course of 206 calendar days.
- 206 days passed between releases.
- 3 commits were understood as [conventional](https://www.conventionalcommits.org).
- 17 unique issues were worked on: [#657](https://github.com/o2sh/onefetch/issues/657), [#664](https://github.com/o2sh/onefetch/issues/664), [#680](https://github.com/o2sh/onefetch/issues/680), [#685](https://github.com/o2sh/onefetch/issues/685), [#699](https://github.com/o2sh/onefetch/issues/699), [#700](https://github.com/o2sh/onefetch/issues/700), [#705](https://github.com/o2sh/onefetch/issues/705), [#733](https://github.com/o2sh/onefetch/issues/733), [#752](https://github.com/o2sh/onefetch/issues/752), [#755](https://github.com/o2sh/onefetch/issues/755), [#791](https://github.com/o2sh/onefetch/issues/791), [#810](https://github.com/o2sh/onefetch/issues/810), [#812](https://github.com/o2sh/onefetch/issues/812), [#813](https://github.com/o2sh/onefetch/issues/813), [#814](https://github.com/o2sh/onefetch/issues/814), [#815](https://github.com/o2sh/onefetch/issues/815), [#829](https://github.com/o2sh/onefetch/issues/829)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic.

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#657](https://github.com/o2sh/onefetch/issues/657)**
  - Add `--completion` option ([`9aec77c`](https://github.com/o2sh/onefetch/commit/9aec77ca625d4f1114786d6bf4c9e20d5d311633))
- **[#664](https://github.com/o2sh/onefetch/issues/664)**
  - Update PHP colors ([`415c460`](https://github.com/o2sh/onefetch/commit/415c4601b786b699f0b67728b10101fbd96fb1f7))
- **[#680](https://github.com/o2sh/onefetch/issues/680)**
  - Fix for --ascii-colors and --ascii-input ([`9f9764d`](https://github.com/o2sh/onefetch/commit/9f9764db812b552365db96cb9e789e24df5cd132))
- **[#685](https://github.com/o2sh/onefetch/issues/685)**
  - Bump clap from 3.1.18 to 3.2.5 and use clap derive ([`2fa44e2`](https://github.com/o2sh/onefetch/commit/2fa44e2b2517cdcc9eef3fbd606d173f2c178a07))
- **[#699](https://github.com/o2sh/onefetch/issues/699)**
  - Extract language definitions into data file ([`d4e6cda`](https://github.com/o2sh/onefetch/commit/d4e6cdadc6c89efe994b65ead870af27e6468ee9))
- **[#700](https://github.com/o2sh/onefetch/issues/700)**
  - Add tests for `author.rs` ([`9cf8df4`](https://github.com/o2sh/onefetch/commit/9cf8df44196cfec2e5c8514f6616fcd5e8b4d77d))
- **[#705](https://github.com/o2sh/onefetch/issues/705)**
  - Gitoxide update ([`2bacf5a`](https://github.com/o2sh/onefetch/commit/2bacf5ad5d616c426cc621f59855b8988522016a))
- **[#733](https://github.com/o2sh/onefetch/issues/733)**
  - Switch to `terminal_size` ([`68fe7e4`](https://github.com/o2sh/onefetch/commit/68fe7e4d78f37f16eb8cb06b0b54cfe875b36fff))
- **[#752](https://github.com/o2sh/onefetch/issues/752)**
  - Update gitoxide to v0.21.1 ([`f52fed6`](https://github.com/o2sh/onefetch/commit/f52fed6423a6136d5af24900771b06238415fe0b))
- **[#755](https://github.com/o2sh/onefetch/issues/755)**
  - Turn InfoField into a trait (big refactoring) ([`066be27`](https://github.com/o2sh/onefetch/commit/066be27eb2128b1257e35d25f2a5decfab0dc955))
- **[#791](https://github.com/o2sh/onefetch/issues/791)**
  - Bump clap from 3.2.22 to 4.0.8 ([`9681164`](https://github.com/o2sh/onefetch/commit/968116428f35d0ec61ab90a96ac0f1c372fe8065))
- **[#810](https://github.com/o2sh/onefetch/issues/810)**
  - Adding test coverage to src/info/info_field.rs ([`3cb479a`](https://github.com/o2sh/onefetch/commit/3cb479aa475d9d6ccdbca3190261d4c301bb7752))
- **[#812](https://github.com/o2sh/onefetch/issues/812)**
  - Testing get_git_username using git-testtools for #700 ([`cbcd100`](https://github.com/o2sh/onefetch/commit/cbcd100e7014a1073b0ecf6c8d7d23f63c44c2d9))
- **[#813](https://github.com/o2sh/onefetch/issues/813)**
  - Improve code coverage of src/info/repo/commits.rs ([`c88541d`](https://github.com/o2sh/onefetch/commit/c88541d3162e32119ba0805e312e17382fb1c4b5))
- **[#814](https://github.com/o2sh/onefetch/issues/814)**
  - Improve code coverage of src/info/repo/contributors.rs ([`adee7d3`](https://github.com/o2sh/onefetch/commit/adee7d31b7baaebb7515ff09e87d99a8a34f6fa8))
- **[#815](https://github.com/o2sh/onefetch/issues/815)**
  - Convert line endings to LF ([`7f37977`](https://github.com/o2sh/onefetch/commit/7f37977f40b4a4dc3119f4ce741dc952873c5f9c))
- **[#829](https://github.com/o2sh/onefetch/issues/829)**
  - Add tests for `author.rs` ([`9cf8df4`](https://github.com/o2sh/onefetch/commit/9cf8df44196cfec2e5c8514f6616fcd5e8b4d77d))
- **Uncategorized** - Unit test author, assert with color ([`64a8148`](https://github.com/o2sh/onefetch/commit/64a8148d716b965c27b87dc9fe099975864e2c77)) - Use contains instead of exact match in title unit tests ([`cc7a59c`](https://github.com/o2sh/onefetch/commit/cc7a59cb9c8ede88ff574942832bde4736c93031)) - Merge branch 'main' of github.com:o2sh/onefetch ([`4c56fb9`](https://github.com/o2sh/onefetch/commit/4c56fb950de79d49ab820b65e26b76e16928e47f)) - Replace ansi_term with enable-ansi-support #747 ([`7cfbef6`](https://github.com/o2sh/onefetch/commit/7cfbef600bf39d9f3441e8cea88bd94c34e3d1c6)) - Remove serde feature from git-repository ([`42e6dc7`](https://github.com/o2sh/onefetch/commit/42e6dc76aa6c150b7c4150dd132e7ddb712d6bcd)) - Rustfmt ([`e0f122a`](https://github.com/o2sh/onefetch/commit/e0f122acc653ebdbc79732945f0352dd68776747)) - Rename git_info to title ([`2a3d995`](https://github.com/o2sh/onefetch/commit/2a3d995cdf24dca8859bfd6f791abe7057e71a3b)) - Cargo clippy ([`4afc056`](https://github.com/o2sh/onefetch/commit/4afc0567009b586899707f435bbdf55f875f4791)) - Add unit tests to author #700 ([`f8285fd`](https://github.com/o2sh/onefetch/commit/f8285fd1f90646fd618e43de154b09ba828e4b47)) - Add unit tests to head_refs #700 ([`0b7111a`](https://github.com/o2sh/onefetch/commit/0b7111a0e5f7d96acbf2158041ba54a62f829294)) - Add unit test to ascii_art #700 ([`8fe3e42`](https://github.com/o2sh/onefetch/commit/8fe3e420d08998d5fac4d4d30de825d5ab1ab1d6)) - Add unit tests to deps/mod #700 ([`e348771`](https://github.com/o2sh/onefetch/commit/e348771ded234ca13dbde0b33d2074fc9e966979)) - More unit tests for cli #700 ([`89d194f`](https://github.com/o2sh/onefetch/commit/89d194f3c9f9c9192fe70f37b01cedd68ebcd8aa)) - Add unit tests to cli #700 ([`f94f2e2`](https://github.com/o2sh/onefetch/commit/f94f2e2185271ffb838797b3dd8b13a93194f3d3)) - Setup unit tests for cli #700 ([`c8c70ba`](https://github.com/o2sh/onefetch/commit/c8c70bad744f8eb396045227a97b40ffa5b5b2e6)) - Add unit tests for package_manager #700 ([`2d57637`](https://github.com/o2sh/onefetch/commit/2d576372c5bd88e4481e745bdab94125efad32de)) - Remove rs extension from tera template ([`c395be8`](https://github.com/o2sh/onefetch/commit/c395be81b9b8283c46800213ee535d6618a1453e)) - Apply bot regex pattern after mailmap ([`264c48f`](https://github.com/o2sh/onefetch/commit/264c48f471d2f4692ff2f247547de8938d3cd2db)) - Update clap derive declarations ([`80c863c`](https://github.com/o2sh/onefetch/commit/80c863c55be4493a115a246b6c8d7a5aa19c0e42)) - Add unit test for info_field #700 ([`45d690a`](https://github.com/o2sh/onefetch/commit/45d690aa071804be96b664adab400f563f474cab)) - Impl from trait for InfoFieldOff ([`8e0450d`](https://github.com/o2sh/onefetch/commit/8e0450d8c109606fafc54d9675d4a4e475049b96)) - Add unit tests to ui/mod.rs #700 ([`d562e86`](https://github.com/o2sh/onefetch/commit/d562e86c32eec9bb8cd8fff8547b7fe99446a575)) - Add unit tests to text_colors #700 ([`26cb0f1`](https://github.com/o2sh/onefetch/commit/26cb0f132f2a33797d81da6fab80474031a31a02)) - Update unit tests for license.rs ([`f5d8fe7`](https://github.com/o2sh/onefetch/commit/f5d8fe72101a8e531c94cf468a61d09f6d841350)) - Add unit tests for license.rs #700 ([`edc27d8`](https://github.com/o2sh/onefetch/commit/edc27d84a925b3bcdaccf92a33c6e64145d1d025)) - Remove licenses folder ([`86e6136`](https://github.com/o2sh/onefetch/commit/86e61365febb81a4d9d24627ceb63b49375af561)) - Cargo clippy ([`70cea29`](https://github.com/o2sh/onefetch/commit/70cea29274b4af7a9c1cfca1371b23aacd1de2a4)) - Replace default regex for --no-bots ([`de37245`](https://github.com/o2sh/onefetch/commit/de3724565223339f6fac6d292b077957dfacbd38)) - Cargo fmt ([`779d7a0`](https://github.com/o2sh/onefetch/commit/779d7a065c5da41b0cad002abef5b9f92a191cb4)) - Remove break lined ([`595f7a1`](https://github.com/o2sh/onefetch/commit/595f7a1045fad54d25f413ead9f4bf097bf0c55c)) - Fix --no-bots option ([`93b7d68`](https://github.com/o2sh/onefetch/commit/93b7d68fb9e12c3d8bc86f0809e654e949cd4ed6)) - Fix serialization of language enum ([`15968d9`](https://github.com/o2sh/onefetch/commit/15968d99823c746290ff51a311fcb927c9214137)) - Allow upper case acronyms for language enum ([`e3c5f5d`](https://github.com/o2sh/onefetch/commit/e3c5f5d0d84687edcd15fcc7d362581e93c7f6c5)) - Remove unused derive attributes ([`3a4a2e8`](https://github.com/o2sh/onefetch/commit/3a4a2e80e1c427d03a61851d6949d9a3cbe42887)) - Simplify Repository instantiation ([`e394155`](https://github.com/o2sh/onefetch/commit/e39415524592f59d691dbeb9fe17c77465c16c0b)) - Allow to open worktrees with gitoxide ([`d8e3c19`](https://github.com/o2sh/onefetch/commit/d8e3c19bac6a13b6ed1f0cd7f9e1ace6b07225db)) - Fix indentation ([`103e719`](https://github.com/o2sh/onefetch/commit/103e719ee56d03058fb885a64328559b8c72f2ba)) - Upgrade to gitoxide 0.16 ([`39e905d`](https://github.com/o2sh/onefetch/commit/39e905dea4ccea4b945cdc2d353421c5a800a526)) - Merge pull request #635 from Byron/gitoxide-for-traversal ([`cc6f332`](https://github.com/o2sh/onefetch/commit/cc6f332e086a952092037c11c7f19585ccca6b07)) - Git2 repository can now be owned by the `Repo` type ([`b6cd415`](https://github.com/o2sh/onefetch/commit/b6cd415d049b24348150e0e2088f2fdb5822e1cb)) - Completely separate `Commits` and `Repo` structure ([`7b34b0a`](https://github.com/o2sh/onefetch/commit/7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e)) - Put all commit-traversal related initialization into own struct ([`d00ab45`](https://github.com/o2sh/onefetch/commit/d00ab45d3cab26e6c8394c2952d7704dd58b8245)) - Make expect("msg") more informative to help users file an issue ([`c6d7cba`](https://github.com/o2sh/onefetch/commit/c6d7cbae0835a3ee21aa88c353f17d885e8164ee)) - Remove additional deduplication of contributors by email ([`fb4d449`](https://github.com/o2sh/onefetch/commit/fb4d449134eff556aac1d14fb95ee9243b0e747e)) - Update to use gitoxide's built-in shallow clone detection ([`b9b65c7`](https://github.com/o2sh/onefetch/commit/b9b65c7eea79b30582be58541554ad35948c909f)) - Make clear that the commit count might be truncated due to shallow cloning ([`927815a`](https://github.com/o2sh/onefetch/commit/927815a5d46f9cc22261ba3796bdee515e139bb1)) - Don't peel references - this would resolve remotes/origin/HEAD to …main… ([`eb753f9`](https://github.com/o2sh/onefetch/commit/eb753f9ca38adcc3e4ad21196bfdd0d1a52c8aa6)) - Use email and name to identify contributors, similar to what git does ([`d3d20ed`](https://github.com/o2sh/onefetch/commit/d3d20eda4dd76720110f70b4992bb9c6483b1f12)) - Support for shallow clones ([`1a494a9`](https://github.com/o2sh/onefetch/commit/1a494a933856f4494749ecf66a58161c2a167535)) - Thanks clippy ([`3f94c51`](https://github.com/o2sh/onefetch/commit/3f94c5138809a67e7d77fd36795d506f896dfd8d)) - Compute contributor identity using emails, lowercase, only ([`397b4ae`](https://github.com/o2sh/onefetch/commit/397b4aec6e125b6a85dbb0fd85f9b7ed978aebdb)) - Improve unification of contributors by taking the lower-case email as identity ([`04ff547`](https://github.com/o2sh/onefetch/commit/04ff54742b8a58b61a6dd3a5c558187ea248bcb5)) - Tune the object cache based on science™️ ([`a5ab965`](https://github.com/o2sh/onefetch/commit/a5ab965a24b97cb0fe33e7b68a8e9fdf8af40701)) - Don't take risks when making assumptions about what branches point at ([`6817e48`](https://github.com/o2sh/onefetch/commit/6817e48be827d95efa1c3ad87cdc0bfaf3421cd4)) - Do three things in parallel, don't wait for `Repo::new()` ([`633f0ce`](https://github.com/o2sh/onefetch/commit/633f0ce6cad0a664e764627b3f193aa59eb212a6)) - Gather language statistics in parallel to everything else ([`d178a5c`](https://github.com/o2sh/onefetch/commit/d178a5c721cf2d7154548ab92f90beaabe2bcda3)) - Get worktree status in parallel ([`5394f3c`](https://github.com/o2sh/onefetch/commit/5394f3c0bc21bb7bb3249f8723986d2542634ba6)) - Refactor ([`4085053`](https://github.com/o2sh/onefetch/commit/4085053f6a10658b98592008d66f4b44987bfdea)) - Refactor ([`4fc3334`](https://github.com/o2sh/onefetch/commit/4fc333417165e969b93147224803e6a110165294)) - Assure short ids are not ambiguous ([`1942087`](https://github.com/o2sh/onefetch/commit/1942087b15d6aa4ef192724f71936cd085ac1340)) - Collect branches at current head-commit with gitoxide ([`f61761d`](https://github.com/o2sh/onefetch/commit/f61761d990f76ecaba82557da3259c9cb6731af7)) - Get most recent version with gitoxide ([`2c6016e`](https://github.com/o2sh/onefetch/commit/2c6016eba41caccddea3ac02a18ea5079b92298f)) - Retrieve all branches with `gitoxide` ([`615e071`](https://github.com/o2sh/onefetch/commit/615e0712ab0fba29ff309b3a01dfc9433cf1d988)) - Gitoxide for tags; Fix author name and email printing; avoid doing unnecessary work ([`c42a1ef`](https://github.com/o2sh/onefetch/commit/c42a1ef809bb122caf064741554fa9e7ca5482f3)) - Refactor ([`9b2774c`](https://github.com/o2sh/onefetch/commit/9b2774cf046523240af85787e8d0f2afcc874426)) - Use `gitoxide` for calculating repo size ([`2a67bb4`](https://github.com/o2sh/onefetch/commit/2a67bb4c8b19fd829ac0d1e96f439b4c882ead03)) - Refactor ([`28deadf`](https://github.com/o2sh/onefetch/commit/28deadfe3422c238e7838f13bbcba718bb54f712)) - Fix commit count ([`65da5be`](https://github.com/o2sh/onefetch/commit/65da5be0af20c38fdc3994cae88d3218d2ae6f49)) - Calculate authors on the fly as much as possible; don't store commits ([`0652bbe`](https://github.com/o2sh/onefetch/commit/0652bbeb488c51243aa3a2283dc0bc6994a4f37b)) - No cloning for `Sig` and `Author` by using BString directly ([`954de84`](https://github.com/o2sh/onefetch/commit/954de842f1f91dff19898158ba593a1a39f3e3a5)) - Obtain all but author information on the fly ([`8df0d19`](https://github.com/o2sh/onefetch/commit/8df0d1966642cfd77783b8222be8537f1fa31b7c))
</details>

## v2.12.0 (2022-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 30 commits contributed to the release over the course of 87 calendar days.
- 125 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 13 unique issues were worked on: [#560](https://github.com/o2sh/onefetch/issues/560), [#562](https://github.com/o2sh/onefetch/issues/562), [#585](https://github.com/o2sh/onefetch/issues/585), [#591](https://github.com/o2sh/onefetch/issues/591), [#602](https://github.com/o2sh/onefetch/issues/602), [#604](https://github.com/o2sh/onefetch/issues/604), [#609](https://github.com/o2sh/onefetch/issues/609), [#613](https://github.com/o2sh/onefetch/issues/613), [#620](https://github.com/o2sh/onefetch/issues/620), [#625](https://github.com/o2sh/onefetch/issues/625), [#626](https://github.com/o2sh/onefetch/issues/626), [#630](https://github.com/o2sh/onefetch/issues/630), [#632](https://github.com/o2sh/onefetch/issues/632)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#560](https://github.com/o2sh/onefetch/issues/560)**
  - Add Ren'Py language support ([`19e8d72`](https://github.com/o2sh/onefetch/commit/19e8d72944388f34bd540d48e4c30b81b00a2507))
- **[#562](https://github.com/o2sh/onefetch/issues/562)**
  - Bump clap from 2.34.0 to 3.0.1 ([`8cbb6d6`](https://github.com/o2sh/onefetch/commit/8cbb6d66fd4031991ad2106954649b5fef16ca3f))
- **[#585](https://github.com/o2sh/onefetch/issues/585)**
  - Add language bar ([`e9e20bb`](https://github.com/o2sh/onefetch/commit/e9e20bbc83d71ae74caf12e2cb7ae2099f991353))
- **[#591](https://github.com/o2sh/onefetch/issues/591)**
  - Bump clap from 3.0.14 to 3.1.1 ([`8c6236e`](https://github.com/o2sh/onefetch/commit/8c6236edc422a85cbb9d8bfa18fd8428af342658))
- **[#602](https://github.com/o2sh/onefetch/issues/602)**
  - Add Ceylon language support ([`96f8d61`](https://github.com/o2sh/onefetch/commit/96f8d61f891e16ad75c49620c30b51245072b0c1))
- **[#604](https://github.com/o2sh/onefetch/issues/604)**
  - Default to terminal foreground color for tilde, underline, colon and info ([`0ef5f58`](https://github.com/o2sh/onefetch/commit/0ef5f580619e66737423cf66e4d168901aa717a6))
- **[#609](https://github.com/o2sh/onefetch/issues/609)**
  - Add Wolfram language support ([`6d14f6e`](https://github.com/o2sh/onefetch/commit/6d14f6e5801cca390d676326a3f7bd61bfc8d959))
- **[#613](https://github.com/o2sh/onefetch/issues/613)**
  - Make time test relative to current time ([`2c1f2f0`](https://github.com/o2sh/onefetch/commit/2c1f2f0b2c666f6ce94af0299f88048dd1d83484))
- **[#620](https://github.com/o2sh/onefetch/issues/620)**
  - Replace colored with owo-colors ([`1773abe`](https://github.com/o2sh/onefetch/commit/1773abe613dad021433667e00809a85e112595bf))
- **[#625](https://github.com/o2sh/onefetch/issues/625)**
  - Remap White ANSI color to Default ([`7b89eff`](https://github.com/o2sh/onefetch/commit/7b89eff51d4fa282b4d7966b3a65bfb58c1b9327))
- **[#626](https://github.com/o2sh/onefetch/issues/626)**
  - Fix "other" language block hidden in background ([`dd2b6c5`](https://github.com/o2sh/onefetch/commit/dd2b6c5e3e90f6d3b383c385ea8bf7581d0d8fff))
- **[#630](https://github.com/o2sh/onefetch/issues/630)**
  - Match circle color with github linguist ([`03139c9`](https://github.com/o2sh/onefetch/commit/03139c90b5000c1ff25bc83ff89798ddc5b45f5c))
- **[#632](https://github.com/o2sh/onefetch/issues/632)**
  - Add language support for VHDL ([`1018d9a`](https://github.com/o2sh/onefetch/commit/1018d9a907d9325cc000ed9430f306b9ac259769))
- **Uncategorized** - Use gitoxide in all methods related to commits ([`e3b29b0`](https://github.com/o2sh/onefetch/commit/e3b29b0aee7391baa39f613aa8798b0c137e386a)) - Get_logs() with gitoxide ([`1bb11e6`](https://github.com/o2sh/onefetch/commit/1bb11e649e2c09d0601409274094081ce3a26803)) - Make info.text_colors private ([`17047db`](https://github.com/o2sh/onefetch/commit/17047db0089b227b5a1ca22634f8010d640e6530)) - Rename function ([`b228e46`](https://github.com/o2sh/onefetch/commit/b228e4615e1457d55a60c2d97a9612f48c8ac5dc)) - Refacto styling of info lines ([`452ad6e`](https://github.com/o2sh/onefetch/commit/452ad6eb55997a0b883809ee18b4ba4bd013ba73)) - Remove blank space ([`06dbe1a`](https://github.com/o2sh/onefetch/commit/06dbe1a32b3754fb8fbe679c756927e542fba47d)) - Rename function in ascii_art ([`6fbcfb5`](https://github.com/o2sh/onefetch/commit/6fbcfb58fc1fa8bafa1581b5155013099c56e086)) - Improve readability get_language_field ([`0f6765c`](https://github.com/o2sh/onefetch/commit/0f6765c9d29635c27f6733afb2df60d2247410a5)) - Merge branch 'main' of github.com:o2sh/onefetch ([`9df7631`](https://github.com/o2sh/onefetch/commit/9df7631b548d6366ebc971a7093276285ee8aae9)) - Cargo clippy ([`51e65ac`](https://github.com/o2sh/onefetch/commit/51e65ac1344c0752274ff8ed94bba0133995cacd)) - Bump image dependency ([`29e1056`](https://github.com/o2sh/onefetch/commit/29e1056ad78a7e086332d170816263d7c3a06a10)) - Update error message ([`bc09db5`](https://github.com/o2sh/onefetch/commit/bc09db57a03bbe8c4947f52b870a54dc2e707ffe)) - Skip first condition in is_valid check, #588 ([`43511dc`](https://github.com/o2sh/onefetch/commit/43511dc2b022c79bd73a12cf9540892fb2c6b3f7)) - Update c# ascii logo ([`0c20aff`](https://github.com/o2sh/onefetch/commit/0c20aff77809b5a992453d6c10a0e87bd2db893d)) - Update c ascii logo ([`412a662`](https://github.com/o2sh/onefetch/commit/412a66240f8c2218cf4f20e58565c58a103254d0)) - Merge branch 'main' of github.com:o2sh/onefetch ([`f00c8b3`](https://github.com/o2sh/onefetch/commit/f00c8b308c7bb2a616821fc31d8fc34a6c705f6d)) - Update c++ ascii logo ([`e19b106`](https://github.com/o2sh/onefetch/commit/e19b1062ce0a2e969ebe473d2fd94fe983006f15))
</details>

## v2.11.0 (2021-11-23)

<csr-id-b85dda815ed3a9275d5c4d6b858f23d6ccad15e1/>

### New Features

- <csr-id-be7787ea25e4dfeb7dae813d37dfbc6033041640/> support AutoHotKey
  - feat: support AutoHotKey

* improvement: improve ahk logo
* fix alphabetical order of languages

### Other

- <csr-id-b85dda815ed3a9275d5c4d6b858f23d6ccad15e1/> switch to weekly schedule

### Commit Statistics

<csr-read-only-do-not-edit/>

- 37 commits contributed to the release over the course of 141 calendar days.
- 142 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 17 unique issues were worked on: [#465](https://github.com/o2sh/onefetch/issues/465), [#466](https://github.com/o2sh/onefetch/issues/466), [#491](https://github.com/o2sh/onefetch/issues/491), [#495](https://github.com/o2sh/onefetch/issues/495), [#496](https://github.com/o2sh/onefetch/issues/496), [#497](https://github.com/o2sh/onefetch/issues/497), [#498](https://github.com/o2sh/onefetch/issues/498), [#499](https://github.com/o2sh/onefetch/issues/499), [#501](https://github.com/o2sh/onefetch/issues/501), [#505](https://github.com/o2sh/onefetch/issues/505), [#506](https://github.com/o2sh/onefetch/issues/506), [#507](https://github.com/o2sh/onefetch/issues/507), [#508](https://github.com/o2sh/onefetch/issues/508), [#509](https://github.com/o2sh/onefetch/issues/509), [#513](https://github.com/o2sh/onefetch/issues/513), [#514](https://github.com/o2sh/onefetch/issues/514), [#519](https://github.com/o2sh/onefetch/issues/519)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#465](https://github.com/o2sh/onefetch/issues/465)**
  - Count hidden files and directories ([`8a9cb2d`](https://github.com/o2sh/onefetch/commit/8a9cb2dede5efbdf4b2ba629c593caea3cdd6d06))
- **[#466](https://github.com/o2sh/onefetch/issues/466)**
  - Better error handling ([`9466f26`](https://github.com/o2sh/onefetch/commit/9466f26925efbc11707c7bcf08d69bfa8b89fbf7))
- **[#491](https://github.com/o2sh/onefetch/issues/491)**
  - Add sql support ([`a3455d4`](https://github.com/o2sh/onefetch/commit/a3455d494a48d0d9dc986deda7855c9cb1788e8b))
- **[#495](https://github.com/o2sh/onefetch/issues/495)**
  - Add WebAssembly language support ([`81e3a03`](https://github.com/o2sh/onefetch/commit/81e3a033ff9f9ef23787a128e2f599123b5efb9b))
- **[#496](https://github.com/o2sh/onefetch/issues/496)**
  - Add TOML language support ([`484b9e4`](https://github.com/o2sh/onefetch/commit/484b9e44577cfb5c984cde565df1020dd6d9aca8))
- **[#497](https://github.com/o2sh/onefetch/issues/497)**
  - Removed json dependency ([`94a100a`](https://github.com/o2sh/onefetch/commit/94a100aa66da14cf76a6e5a573a2588a5caf2d54))
- **[#498](https://github.com/o2sh/onefetch/issues/498)**
  - Add Jsonnet language support ([`f77f376`](https://github.com/o2sh/onefetch/commit/f77f376be571f7c831e6a5ce9337289c17af4548))
- **[#499](https://github.com/o2sh/onefetch/issues/499)**
  - Add YAML language support ([`de015f0`](https://github.com/o2sh/onefetch/commit/de015f03c097fdedf6ee88ec6277430ad0a0ead1))
- **[#501](https://github.com/o2sh/onefetch/issues/501)**
  - Add Solidity language support ([`01e06df`](https://github.com/o2sh/onefetch/commit/01e06df1e71b280191aa1980171f927c70bd2a00))
- **[#505](https://github.com/o2sh/onefetch/issues/505)**
  - Add support for json ([`35796b7`](https://github.com/o2sh/onefetch/commit/35796b71b7272667774fb4e5914920103cb33438))
- **[#506](https://github.com/o2sh/onefetch/issues/506)**
  - Add SASS language support ([`7aa883f`](https://github.com/o2sh/onefetch/commit/7aa883f99154318c4b715f03c41818b9fa0281cc))
- **[#507](https://github.com/o2sh/onefetch/issues/507)**
  - Add LLVM language support ([`07e8929`](https://github.com/o2sh/onefetch/commit/07e8929ca21bc6a51bb419d3bfa104e8abde1836))
- **[#508](https://github.com/o2sh/onefetch/issues/508)**
  - Support AutoHotKey ([`be7787e`](https://github.com/o2sh/onefetch/commit/be7787ea25e4dfeb7dae813d37dfbc6033041640))
- **[#509](https://github.com/o2sh/onefetch/issues/509)**
  - Change Ruby logo to red ([`f11777c`](https://github.com/o2sh/onefetch/commit/f11777cbdf417665f28b91167b0f248876a57f0f))
- **[#513](https://github.com/o2sh/onefetch/issues/513)**
  - Add language type ([`b98a26c`](https://github.com/o2sh/onefetch/commit/b98a26c684f02e08dcc66f948315320c7bd6f8b8))
- **[#514](https://github.com/o2sh/onefetch/issues/514)**
  - Add Coq support ([`5074a16`](https://github.com/o2sh/onefetch/commit/5074a16f693212f212a04de1db398c94d917ae77))
- **[#519](https://github.com/o2sh/onefetch/issues/519)**
  - Add support for fortran legacy ([`4ad136a`](https://github.com/o2sh/onefetch/commit/4ad136ac45ab792cba2092d22afe80d55eb7895c))
- **Uncategorized** - Merge pull request #533 from HallerPatrick/main ([`ee6ef95`](https://github.com/o2sh/onefetch/commit/ee6ef95ab2430f858c1f1c80a78c8677391b8533)) - Update repo.rs ([`742c979`](https://github.com/o2sh/onefetch/commit/742c979b3b07dda2f486734daf6489ad0e170fc7)) - Adding tests for display time in output ([`3d7b14b`](https://github.com/o2sh/onefetch/commit/3d7b14b856300b145f2b2dfec2ca62cb5c0785e8)) - #526 Remove chrono as dependency and swiched out with time-rs and time-humanize, refactored code to display creaton of repo time ([`6ceb13f`](https://github.com/o2sh/onefetch/commit/6ceb13f36d589643f016009cad1485e4fb256d93)) - Fix some language serialization name ([`f86070b`](https://github.com/o2sh/onefetch/commit/f86070b6a102f483b0b58b20fd6ec96d1343664a)) - Update rustfmt ([`f34303b`](https://github.com/o2sh/onefetch/commit/f34303ba2de35ef8334ef2f270d11efe1ea93b51)) - Change language type for yaml ([`6788f02`](https://github.com/o2sh/onefetch/commit/6788f0273751dc8e8b66296ed02c1257d038f691)) - Add langs module ([`8699d26`](https://github.com/o2sh/onefetch/commit/8699d26444607eb71b1faf8748be223d1d460a90)) - Merge branch 'main' of github.com:o2sh/onefetch ([`3363357`](https://github.com/o2sh/onefetch/commit/33633573fe2d1d9412229b87f9be8beb0f057212)) - Fix cargo clippy ([`ce74501`](https://github.com/o2sh/onefetch/commit/ce74501b141acd907da1b34e44172d3afac9c2c3)) - Enable wrap_help feature for clap ([`8499b2a`](https://github.com/o2sh/onefetch/commit/8499b2ae649c15f0a1e1e71fc93dd7330830aa07)) - Remove error-chain recursion limit ([`dfe5646`](https://github.com/o2sh/onefetch/commit/dfe5646a83f07019306d4199c1a1767e9be80eb5)) - #467, fix coloring protobuf ([`ce07578`](https://github.com/o2sh/onefetch/commit/ce07578a980e30bf5a9989abd61c34e53f2b46e8)) - Merge pull request #468 from HallerPatrick/main ([`2bc490f`](https://github.com/o2sh/onefetch/commit/2bc490fe0b4cb7a78b89534bc80acfea2f2e562f)) - New language support for protobuf with ascii image ([`04dd387`](https://github.com/o2sh/onefetch/commit/04dd3876e26d599927162594008fd065bf468def)) - Fix exclude option ([`4cf3600`](https://github.com/o2sh/onefetch/commit/4cf3600822496cf43b0eaa18900ad0fd7fc3bb07)) - Change cli name from --show-email to --email ([`6d032da`](https://github.com/o2sh/onefetch/commit/6d032da6376958e5617939e764ab403581c5f183)) - Remove unnecessary logic in exclude pattern ([`968009e`](https://github.com/o2sh/onefetch/commit/968009e43c406d9650930284a7d6cdc3b9ccf111)) - Fix text coloring ([`23e6d53`](https://github.com/o2sh/onefetch/commit/23e6d53adefa6e9350f66884a0c8c781b4d12967)) - Refacto info formatter ([`d890933`](https://github.com/o2sh/onefetch/commit/d8909335ed0909678f740d3661d0dd38c755d4fe))
</details>

## v2.10.2 (2021-07-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 1 commit contributed to the release.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Fix text coloring ([`0515730`](https://github.com/o2sh/onefetch/commit/0515730f9170ca1474a4cde1c9520bc7165b40fb))
</details>

## v2.10.1 (2021-07-03)

## v2.10.0 (2021-07-03)

### New Features

- <csr-id-b88abf9042732e36380e06aae7418f81a3aee56a/> add Svelte support
  - feat(languages): add Svelte support

* Svelte, no need to override the serialized name

### Commit Statistics

<csr-read-only-do-not-edit/>

- 34 commits contributed to the release over the course of 165 calendar days.
- 168 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 8 unique issues were worked on: [#394](https://github.com/o2sh/onefetch/issues/394), [#412](https://github.com/o2sh/onefetch/issues/412), [#435](https://github.com/o2sh/onefetch/issues/435), [#436](https://github.com/o2sh/onefetch/issues/436), [#438](https://github.com/o2sh/onefetch/issues/438), [#452](https://github.com/o2sh/onefetch/issues/452), [#454](https://github.com/o2sh/onefetch/issues/454), [#456](https://github.com/o2sh/onefetch/issues/456)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#394](https://github.com/o2sh/onefetch/issues/394)**
  - Add haxe language support ([`356f71e`](https://github.com/o2sh/onefetch/commit/356f71e634a48d900377ff35d24560bfc1436b18))
- **[#412](https://github.com/o2sh/onefetch/issues/412)**
  - Add Scheme language ([`ae2cc1b`](https://github.com/o2sh/onefetch/commit/ae2cc1b35c876f8b092a1c7eb9fd9021354930a0))
- **[#435](https://github.com/o2sh/onefetch/issues/435)**
  - Added Ada support ([`f8bf292`](https://github.com/o2sh/onefetch/commit/f8bf29206e624b2a8b64752456e00a561122a5bf))
- **[#436](https://github.com/o2sh/onefetch/issues/436)**
  - Add Svelte support ([`b88abf9`](https://github.com/o2sh/onefetch/commit/b88abf9042732e36380e06aae7418f81a3aee56a))
- **[#438](https://github.com/o2sh/onefetch/issues/438)**
  - Add HCL Support ([`1bfd60a`](https://github.com/o2sh/onefetch/commit/1bfd60a39293b8cbb1a55ac7266732a2302d2d87))
- **[#452](https://github.com/o2sh/onefetch/issues/452)**
  - Add CLI option for displaying author emails ([`0130a25`](https://github.com/o2sh/onefetch/commit/0130a253289449fb2bd56b910a553b28a5a5cc40))
- **[#454](https://github.com/o2sh/onefetch/issues/454)**
  - Add support for powershell ([`5244828`](https://github.com/o2sh/onefetch/commit/5244828a369d2d6844249a994910b44d545bd7e1))
- **[#456](https://github.com/o2sh/onefetch/issues/456)**
  - Display the number of contributors ([`4028f8c`](https://github.com/o2sh/onefetch/commit/4028f8c51543ca27cc94a9b3e954e45b6890e9db))
- **Uncategorized** - Merge pull request #453 from o2sh/feature/use-mailmap ([`12df1fa`](https://github.com/o2sh/onefetch/commit/12df1fac6fbe126eed64baa1f863a68b9ca883b3)) - Fix json/yaml serializer ([`9b1aa16`](https://github.com/o2sh/onefetch/commit/9b1aa1691607212c187ebe88b65e8e931bd0c211)) - Merge branch 'main' into feature/use-mailmap ([`6d84c2d`](https://github.com/o2sh/onefetch/commit/6d84c2d1057654a85516d111201c27c097b664c2)) - Reorganize file structure ([`f352e98`](https://github.com/o2sh/onefetch/commit/f352e98f2068a00661c57657c8690f88b46e2ed5)) - Reorganize project file structure ([`7a37b01`](https://github.com/o2sh/onefetch/commit/7a37b01442692ae8658ea27bb341d3394a083229)) - Remove from language macro in favor tokei fmt ([`da375be`](https://github.com/o2sh/onefetch/commit/da375be6f7810f4ef00b83fd21043bb0fe22be4d)) - Replace master to main ([`228f385`](https://github.com/o2sh/onefetch/commit/228f3858d918f336fee4d4e6b6c0b4629f0d1e9e)) - Fix symmetry on jupyter ascii logo ([`1e0b6c9`](https://github.com/o2sh/onefetch/commit/1e0b6c90092afdc634f15b852cc29b62eaf82101)) - Fallback to commit.author() if commit.author_with_mailmap() fails ([`e031a88`](https://github.com/o2sh/onefetch/commit/e031a88ce7718330174b8a691a5921d4917ac5eb)) - Cargo clippy ([`67ea4c7`](https://github.com/o2sh/onefetch/commit/67ea4c7a092f750be19692bd3d008f9dad86be9b)) - Merge authors by signature ~ username + email ([`98f5756`](https://github.com/o2sh/onefetch/commit/98f57566ce05b91294dea51af72102a1bd37f114)) - Use mailmap bindings ([`9ba9533`](https://github.com/o2sh/onefetch/commit/9ba9533ba179744fc3f2ba2c81b73a6c8bf8ac98)) - Fix warning: derive helper attribute is used before it is introduced ([`2e39715`](https://github.com/o2sh/onefetch/commit/2e397155b3ab2ed6f15ea8ec2f3e84fd07dd20f7)) - Fmt cli flags declaration ([`28a1c13`](https://github.com/o2sh/onefetch/commit/28a1c13347c36242c29db2c7c144b5dd3e2b47a7)) - Cargo clippy ([`bff472f`](https://github.com/o2sh/onefetch/commit/bff472fa40f5b1b3336e6cd3004f7c62c2c102d9)) - Merge pull request #448 from o2sh/feature/ignore-bot-commits ([`a611fe4`](https://github.com/o2sh/onefetch/commit/a611fe48154b1f8d61f60c1ce9ed776b3cc229bc)) - CR, ignore bot commits ([`eb526b2`](https://github.com/o2sh/onefetch/commit/eb526b29c5ae87aaf0bfffe06fa440ac40635110)) - No-bots with optional pattern ([`63d2234`](https://github.com/o2sh/onefetch/commit/63d22345fa679a96a69a694a874ad481802e6260)) - Ignore bots in commits ([`76e665f`](https://github.com/o2sh/onefetch/commit/76e665f77a0dcfec674e7040904186d06a9a4ac1)) - Shorter CLI flags ([`aa80dc9`](https://github.com/o2sh/onefetch/commit/aa80dc96cd9a94940ccbba5f1160ca3adc69523b)) - Round up instead of truncate in perc. of contribution ([`6821f96`](https://github.com/o2sh/onefetch/commit/6821f96f48160f52093f9e68d74f84fc43a86b75)) - #428, add support for GDScript ([`f4b2833`](https://github.com/o2sh/onefetch/commit/f4b2833b9625dabd90ad47ecd07d2031983812d8)) - Revert better basic coloring for haxe ([`6047ad8`](https://github.com/o2sh/onefetch/commit/6047ad8b4c53f39ba611ccd74442219987ec4fd1)) - Better basic coloring for haxe ([`a17981f`](https://github.com/o2sh/onefetch/commit/a17981f34dcb94d596d1755fa6fe7b1b4696a4a7)) - Fix haxe alphabetical order ([`bb0c6ac`](https://github.com/o2sh/onefetch/commit/bb0c6ac4f4ecd13ab100a5dae4b438aa37c38359)) - Fix #377: fix type Typo --> now ago when using to_text_en with accuracy rough ([`ce4fe5b`](https://github.com/o2sh/onefetch/commit/ce4fe5bbbee68feacd8521c309eda4ad0976e105))
</details>

## v2.9.1 (2021-01-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 4 commits contributed to the release over the course of 1 calendar day.
- 2 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 2 unique issues were worked on: [#375](https://github.com/o2sh/onefetch/issues/375), [#376](https://github.com/o2sh/onefetch/issues/376)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#375](https://github.com/o2sh/onefetch/issues/375)**
  - Update dependencies and fix serde related build failure ([`0c36fe6`](https://github.com/o2sh/onefetch/commit/0c36fe6f41c355680a04353f9cce8903c52cb091))
- **[#376](https://github.com/o2sh/onefetch/issues/376)**
  - Fixed spelling mistake ([`5b10724`](https://github.com/o2sh/onefetch/commit/5b10724039f35a62d92a8c2fa586258bef27cb0f))
- **Uncategorized** - Reorder --true-color flag ([`5445659`](https://github.com/o2sh/onefetch/commit/5445659ce58c724b128f647f0cef5c1f5ed0a54e)) - Reorder -z flag ([`171c099`](https://github.com/o2sh/onefetch/commit/171c09976e4988d27c8816ec3d76183c02c7e081))
</details>

## v2.9.0 (2021-01-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 17 commits contributed to the release over the course of 27 calendar days.
- 27 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 3 unique issues were worked on: [#362](https://github.com/o2sh/onefetch/issues/362), [#370](https://github.com/o2sh/onefetch/issues/370), [#373](https://github.com/o2sh/onefetch/issues/373)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#362](https://github.com/o2sh/onefetch/issues/362)**
  - Add qml support ([`f28c3ce`](https://github.com/o2sh/onefetch/commit/f28c3ce20ca4aca96e391f75e49f1129aba81b12))
- **[#370](https://github.com/o2sh/onefetch/issues/370)**
  - Add YAML output (-y) and ISO 8601 date format option (-z) ([`3e9cd24`](https://github.com/o2sh/onefetch/commit/3e9cd2466007cf12df0629872d0e428831297594))
- **[#373](https://github.com/o2sh/onefetch/issues/373)**
  - Add CLI option to switch true colors on/off ([`503d82f`](https://github.com/o2sh/onefetch/commit/503d82f81f6311002e106c5c19bdc5d129817ddc))
- **Uncategorized** - Append new line when printing in json/yaml format ([`e500690`](https://github.com/o2sh/onefetch/commit/e500690c2b2b48e9b9e91d62bd143444c1fecbf2)) - Rustfmt and update README ([`fca326b`](https://github.com/o2sh/onefetch/commit/fca326b748d912a1aea7f3ed44c6e45e5aa5120c)) - Merge pull request #353 from geeseven/graphql ([`dd2c43b`](https://github.com/o2sh/onefetch/commit/dd2c43b0915f29f2a0bdf9f41a06850e65553d57)) - Add support for GraphQL ([`a5f58ca`](https://github.com/o2sh/onefetch/commit/a5f58ca985c5a7935eea6fd673ac5acbfe63f4d8)) - Merge pull request #352 from geeseven/emojicode ([`2967891`](https://github.com/o2sh/onefetch/commit/29678913006cc0bc1568692074cb81245010d697)) - Brighter colors for emojicode ([`7728237`](https://github.com/o2sh/onefetch/commit/772823722d732b52a071428f591bf205e3498759)) - Add support for Emojicode ([`7f1ae19`](https://github.com/o2sh/onefetch/commit/7f1ae192806b43a6e478a927e59bc0c999056b23)) - Fix rustfmt ([`c44489a`](https://github.com/o2sh/onefetch/commit/c44489a35fddcef7d9013f780b44f0c7c143bd7a)) - Rename deps folder ([`ff6dadc`](https://github.com/o2sh/onefetch/commit/ff6dadcb6c846d3c05ab1d2a9bc680f7c1ed371d)) - Merge pull request #350 from o2sh/feature/macro_package_managers ([`7de7cc2`](https://github.com/o2sh/onefetch/commit/7de7cc26bf5e2079f16eb36f91807c3ca47b8ce0)) - Add support for multiple parsers per package manager ([`43206c4`](https://github.com/o2sh/onefetch/commit/43206c42648db9f7879dde909a296446e3ca0268)) - Fix clippy warn ([`a7055f1`](https://github.com/o2sh/onefetch/commit/a7055f15145d04b1370d8f98864f65ee1243db1b)) - Create define_package_managers macro ([`dbb88e8`](https://github.com/o2sh/onefetch/commit/dbb88e8e8ac0889fd469bdc25829278058166406)) - Use tuple deconstruction for get_git_info_field ([`966f598`](https://github.com/o2sh/onefetch/commit/966f598748fdf0ee80716628e83f3ca0789db8e8))
</details>

## v2.8.0 (2020-12-16)

<csr-id-39e6f61ed251cce1cd7b2350d20f2283f9605263/>

### Refactor

- <csr-id-39e6f61ed251cce1cd7b2350d20f2283f9605263/> get rid of duplicate code

### New Features

- <csr-id-add14cfcb25873733af99b952bf351c719fd0c99/> hide logo if terminal size is too wide
  a new CLI option to configure whether the logo should be shown
  if configued to auto, Onefetch will detect the terminal size and
  hide/show the logo based off that.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 43 commits contributed to the release over the course of 24 calendar days.
- 24 days passed between releases.
- 2 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Fix get_git_info_field when version or username is empty ([`b6705bb`](https://github.com/o2sh/onefetch/commit/b6705bbf625646887d7135696546db7013f1aa72)) - Merge pull request #346 from HallerPatrick/master ([`8fd575a`](https://github.com/o2sh/onefetch/commit/8fd575a2a11fdc4ea1b90e1ebd1ff757f0456659)) - Added pub as another package manager for dart projects ([`1e46f4f`](https://github.com/o2sh/onefetch/commit/1e46f4f0031815ba7646ae98064d14352beaff06)) - Fix JSON serializer if Git is not installed ([`e01bc38`](https://github.com/o2sh/onefetch/commit/e01bc38142fb69c06abd2cbf735641d2485f47ec)) - Update readme ([`0da1c02`](https://github.com/o2sh/onefetch/commit/0da1c02af49835f2aea6cfbe1a57072a8d80cd22)) - Rename format flag to output and other changes ([`667897e`](https://github.com/o2sh/onefetch/commit/667897e63be8f15f7d32775052240cb502049e4f)) - Merge pull request #341 from HallerPatrick/master ([`80424f4`](https://github.com/o2sh/onefetch/commit/80424f4f32cb572513bd50c305f0c8616e295a58)) - Code style change ([`ae68b53`](https://github.com/o2sh/onefetch/commit/ae68b53aa23de8e303ea7abffad333d9b472a17f)) - Don't panic when commit not found in get_logs ([`9207c6c`](https://github.com/o2sh/onefetch/commit/9207c6c79d71548a36682871cc15ca5bf27f0daf)) - Multiple changes linked to git2-rs migration ([`e99c56f`](https://github.com/o2sh/onefetch/commit/e99c56f0fe9b03c92e1f3234a322bba6599a4a31)) - Removed time sorting for faster RT ([`826ac93`](https://github.com/o2sh/onefetch/commit/826ac93e68e24eb1769dc95044be1cb7c520784e)) - Minor refactoring, now using a more 'safe' conversion of autho mail and name due to invalid utf8 ([`3943231`](https://github.com/o2sh/onefetch/commit/3943231cc23d1d8f60606ebfafd9952180156125)) - Put all git call functions into a GitClient impl, minor refactoring ([`d673169`](https://github.com/o2sh/onefetch/commit/d673169cafdcbac59871c233d1bd51eda26c480f)) - Replaced os git command with git2 ([`cc1f29c`](https://github.com/o2sh/onefetch/commit/cc1f29ca4a2624fba7eaaed7d61067e85adb2803)) - Merge pull request #337 from HallerPatrick/master ([`4be57b5`](https://github.com/o2sh/onefetch/commit/4be57b5249aeda8ec017d1d81c399b1bfefad231)) - Merge branch 'master' of https://github.com/HallerPatrick/onefetch into master ([`ffcb73e`](https://github.com/o2sh/onefetch/commit/ffcb73e7b790f98ba5f14c606cb5e7c01e262a01)) - Using possible_values instead of validator for format arg ([`2e3760a`](https://github.com/o2sh/onefetch/commit/2e3760ac6ec918e0db3485a20510306b105d49eb)) - Update src/onefetch/cli.rs ([`9cb465c`](https://github.com/o2sh/onefetch/commit/9cb465cd4777e25a69f57b5d6c903fa64b443b15)) - Removed serde_test, use of format flag instead of json flag, chang of string concat to format macro in git_utils ([`219190b`](https://github.com/o2sh/onefetch/commit/219190ba1963dabd841cfee3e46e024c3c53ab09)) - Custom impl of serializer for Info struct, also some refactoring of git_utils functions for more granual information for json ([`33ec3cd`](https://github.com/o2sh/onefetch/commit/33ec3cdb3b6a0f923a693583374c340a3d172b04)) - Commit of missing changes of last commit ([`9993dcb`](https://github.com/o2sh/onefetch/commit/9993dcb0ec503f3a5729a7bd6401299f8aace111)) - Implementation of json flag, wich allows to print out a json representation of onefetch output ([`65c7a54`](https://github.com/o2sh/onefetch/commit/65c7a545108c7443e5086aafcfb895d8bd50115b)) - Reorder cli flags ([`2b5b5ef`](https://github.com/o2sh/onefetch/commit/2b5b5ef5d754b65063ea0b2a4f55739b43afd32b)) - Inverse logic for hide-logo flag #330 ([`33ad7de`](https://github.com/o2sh/onefetch/commit/33ad7dede9b8d05191f11878fce89bcfb0d1cbce)) - Merge pull request #330 from Luke-zhang-04/master ([`a4698dc`](https://github.com/o2sh/onefetch/commit/a4698dc83325b53692991db5aa634f2535d195f8)) - Remove never option for hide-logo flag ([`2979a77`](https://github.com/o2sh/onefetch/commit/2979a7756450450eb2109f860f2cf85ce9ee8461)) - Auto, always, never options for hide-logo with always as default ([`51a7a4b`](https://github.com/o2sh/onefetch/commit/51a7a4bd58f741fd9fb025e863d7d2201b3f2192)) - Modified CLI (See full commit msg for details) ([`c72dbbe`](https://github.com/o2sh/onefetch/commit/c72dbbe116d038cc455ba0c94df57f5ff95eeefb)) - Cargo fmt ([`5762b15`](https://github.com/o2sh/onefetch/commit/5762b151703bdf1ba1f68896ff498315fb3babfd)) - Remove `off` option, allow user to define termianl width ([`711e41e`](https://github.com/o2sh/onefetch/commit/711e41eab1194b050f782e481f775e75ae5fd6a3)) - Use `std::eprintln` ([`bce8f07`](https://github.com/o2sh/onefetch/commit/bce8f07932c63c8ae0d9848dbe1fef2acb8d59fe)) - Get rid of duplicate code ([`39e6f61`](https://github.com/o2sh/onefetch/commit/39e6f61ed251cce1cd7b2350d20f2283f9605263)) - Default to auto ([`9616e79`](https://github.com/o2sh/onefetch/commit/9616e799f0e3e7d7bba1e4284df4b194c4f3d209)) - Fix typo in help message ([`d4ce353`](https://github.com/o2sh/onefetch/commit/d4ce3532fca2cd77d7a80edd4d1cf3cd172843c7)) - Hide logo if terminal size is too wide ([`add14cf`](https://github.com/o2sh/onefetch/commit/add14cfcb25873733af99b952bf351c719fd0c99)) - Move check for valid Git repo further down #329 ([`72e3682`](https://github.com/o2sh/onefetch/commit/72e36822bc00095c1eb7a19976eb770a2ef86642)) - Merge pull request #326 from Luke-zhang-04/master ([`fb8a387`](https://github.com/o2sh/onefetch/commit/fb8a38714dcc3007699be9a2ec6bb65acaa2ddef)) - Clean processing ascii logo ([`423c357`](https://github.com/o2sh/onefetch/commit/423c357eff5e2871e43fd8a8dba18a15222e37c4)) - Merge pull request #324 from geeseven/vala ([`8730e15`](https://github.com/o2sh/onefetch/commit/8730e15e91b4c26c35051e835c351a86675f5fb5)) - Add processing support ([`bc51dd5`](https://github.com/o2sh/onefetch/commit/bc51dd5b67b70c27c1dc197259abab37504967cc)) - Add support for Vala ([`b596d6c`](https://github.com/o2sh/onefetch/commit/b596d6c89de5b310195607ce3eb0eaf3c7c39622)) - Merge pull request #322 from geeseven/vimscript ([`d546f20`](https://github.com/o2sh/onefetch/commit/d546f20d245414215063c75961b8f930ed5b36b0)) - Add support for VimScript #321 ([`8d03557`](https://github.com/o2sh/onefetch/commit/8d03557c930d16209b3938d06fdf91a5d39fed5e))
</details>

## v2.7.3 (2020-11-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 1 commit contributed to the release.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - If user.name is not set print unknown instead of crashing ([`6ff85ef`](https://github.com/o2sh/onefetch/commit/6ff85efb5f227cf73b43a37210a5b8f6087bc4dc))
</details>

## v2.7.2 (2020-11-21)

<csr-id-39e6f61ed251cce1cd7b2350d20f2283f9605263/>

### New Features

- <csr-id-add14cfcb25873733af99b952bf351c719fd0c99/> hide logo if terminal size is too wide
  a new CLI option to configure whether the logo should be shown
  if configued to auto, Onefetch will detect the terminal size and
  hide/show the logo based off that.

### Refactor

- <csr-id-39e6f61ed251cce1cd7b2350d20f2283f9605263/> get rid of duplicate code

### Commit Statistics

<csr-read-only-do-not-edit/>

- 37 commits contributed to the release over the course of 13 calendar days.
- 13 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Fix image backend detection ([`7d92b48`](https://github.com/o2sh/onefetch/commit/7d92b48ff0431f3a84c06f0c1229be87f6c6e2f7)) - Merge pull request #318 from o2sh/dependabot/cargo/image-0.23.12 ([`ed1359c`](https://github.com/o2sh/onefetch/commit/ed1359c186acb7b2acf6dbd59a261c5969e4b47f)) - Cargo fmt ([`8b3cd82`](https://github.com/o2sh/onefetch/commit/8b3cd82502d8f973dc0114c24ec3d194aa5b8b81)) - Fix deprecated call to_rgba --> to_rgba8 ([`0428be2`](https://github.com/o2sh/onefetch/commit/0428be2d4961b294f589fee235c8d6757f58bbf9)) - Add check for empty rep_name and repo_url ([`43e540f`](https://github.com/o2sh/onefetch/commit/43e540f7b4f2f01958a02f88632271089b69b89e)) - Remove UnknownField from InfoField ([`2d17efa`](https://github.com/o2sh/onefetch/commit/2d17efa6c16c8d10fbb7d5a2e3ce43ebce28fc1e)) - Refacto info_fields ([`e121796`](https://github.com/o2sh/onefetch/commit/e1217962e95de6c5bab5a995aa2050667038fe1b)) - Better coloring for typescript and lua ([`050e693`](https://github.com/o2sh/onefetch/commit/050e693c187ea60899500375c3563b794660c001)) - Better coloring for c, c# and c++ ([`e50723e`](https://github.com/o2sh/onefetch/commit/e50723e3523628fb2c96d5c213f7de6db8212790)) - Merge pull request #313 from Ferryistaken/holyc-support ([`0af1dcf`](https://github.com/o2sh/onefetch/commit/0af1dcf3187061f1ac0684874bb8fc20731b238f)) - Clean holyC ascii design ([`1bbbcaa`](https://github.com/o2sh/onefetch/commit/1bbbcaa8d8cecee2d05593a2d64d299c468e7cae)) - Fix raku ascii logo ([`f3cbcf7`](https://github.com/o2sh/onefetch/commit/f3cbcf70b96e4c4d9b77a32a0a029b642bf30795)) - Fix cargo clippy ([`cad3ab3`](https://github.com/o2sh/onefetch/commit/cad3ab31ae2d71a4a876bb611aad4b0db556ece8)) - Fix max-width CI tests ([`b4fb7a6`](https://github.com/o2sh/onefetch/commit/b4fb7a6a12a7c72c4608e59f78d16fdc5c3203b8)) - Fix xaml ascii logo #317 ([`f3283ed`](https://github.com/o2sh/onefetch/commit/f3283edec81bd373b2cda37d12a707b57b69cced)) - Add support for Xaml #317 ([`05fec5a`](https://github.com/o2sh/onefetch/commit/05fec5af40ecc223591ff344f03b4b6e87252aef)) - Merge pull request #315 from o2sh/replace-git-sys-calls ([`cff720a`](https://github.com/o2sh/onefetch/commit/cff720a7f442ca31cd911d8c7ab11cdbed51eff4)) - Extract remaining git sys calls into seperate file ([`b8859a4`](https://github.com/o2sh/onefetch/commit/b8859a490753d36126bc5d103711a7688ca46b76)) - Move Printer into its own file ([`a87dc8f`](https://github.com/o2sh/onefetch/commit/a87dc8f5842a406d9b7fafeb7b4b87e108e5b2a3)) - Move is_git_installed in cli_utils.rs ([`96e3a55`](https://github.com/o2sh/onefetch/commit/96e3a551c40a7780ab7eaa896f22e50c269773b6)) - Move get_git_version in cli_utils.rs ([`1fecaea`](https://github.com/o2sh/onefetch/commit/1fecaea0eb4c023e317e964182c05047cbddd417)) - Extract get_ascii_colors from info.rs ([`0c9fcbb`](https://github.com/o2sh/onefetch/commit/0c9fcbb3122e5b4249151e9249fc6d6a40b0db27)) - Fix underflow on get_number_of_branches ([`427029f`](https://github.com/o2sh/onefetch/commit/427029f32a2c96b1039dd379d553257835e3d739)) - Update src/onefetch/repo.rs ([`d159306`](https://github.com/o2sh/onefetch/commit/d15930630b2a01ffeb724efbee15258ecc23454c)) - Update src/onefetch/repo.rs ([`597814a`](https://github.com/o2sh/onefetch/commit/597814a131c1912e53af5737594d8c85bbf56291)) - Migrate get_number_of_tags_branches ([`e0dd9ef`](https://github.com/o2sh/onefetch/commit/e0dd9ef950ee46031e31d8b1e9765f391094a385)) - Filter out tags from HEAD refs ([`c28404d`](https://github.com/o2sh/onefetch/commit/c28404d53e4b57c5d47acaaecc42b399d73de824)) - Remove releases_number ([`59cb0d5`](https://github.com/o2sh/onefetch/commit/59cb0d5e0e5548817a6c0672c8b42b2e7e45ea5d)) - Cargo fmt ([`b491f3a`](https://github.com/o2sh/onefetch/commit/b491f3a655e06be7b5ede856ed1d964ea9a3adcb)) - Create Repo struct and migrate get_version ([`f303b2a`](https://github.com/o2sh/onefetch/commit/f303b2a5558eecca5123e872c0684cb2255d6b29)) - Migrate get_pending_changes ([`c995f3e`](https://github.com/o2sh/onefetch/commit/c995f3e6b97945202ab739d4db6daa90fc611f85)) - Get_repo_name_and_url defaults to remote.origin.url #314 ([`a183a32`](https://github.com/o2sh/onefetch/commit/a183a32daae1b2ada1f52a21a734f950dea614f0)) - New HolyC logo ([`fa8d175`](https://github.com/o2sh/onefetch/commit/fa8d1752d32f021a608d29b78cf71b1b1d7fe4ac)) - Extract som git related fn into separate file ([`e1b7027`](https://github.com/o2sh/onefetch/commit/e1b7027f9b02b7214dce59e483e4f62d54159767)) - Cargo fmt and cargo clippy ([`4f221f7`](https://github.com/o2sh/onefetch/commit/4f221f7179cbdc2bc0b0952314a6c1fe09a757bd)) - Add is_valid_repo check in main ([`a3e0a5d`](https://github.com/o2sh/onefetch/commit/a3e0a5da1f5be974dea40fcd72e90ca29b4c5ef9)) - Added Support for HolyC ([`ed4c5f1`](https://github.com/o2sh/onefetch/commit/ed4c5f1e6d2ba4bc8171e5b39c6d1d5851586863))
</details>

## v2.7.1 (2020-11-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 1 commit contributed to the release.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Remove language::unknown ([`9849aad`](https://github.com/o2sh/onefetch/commit/9849aad6b39f5a8d18797b5829c2d19e01994cc7))
</details>

## v2.7.0 (2020-11-07)

<csr-id-0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88/>
<csr-id-6fdb61d9187308f568833227ac2a8a1c358d66b9/>
<csr-id-a19ced85921a17a42ad2d76b744ba765f1038341/>

### Refactor

- <csr-id-0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88/> use `map.contains_key()` instead of iterating
  using map.contains_key is easier to understand and probably has better
  lookup time
- <csr-id-6fdb61d9187308f568833227ac2a8a1c358d66b9/> move `is_package_file` to Detector impl`

- <csr-id-a19ced85921a17a42ad2d76b744ba765f1038341/> simplify regex in package parsers

### New Features

- <csr-id-f5ea1f7c4befcd5d0c9983b887761c24a0fc59de/> add support for Cargo
- <csr-id-deefcb53631635fa76d0f331d852b5c18ba3a487/> add support for yarn
- <csr-id-d79f447a47cbc4e4e7d591471cb217807ffc8827/> add support for pip
- <csr-id-582b24609fde4a1a568cb85ee60e475d0ea8ede9/> add support for go modules
- <csr-id-1c627037f007ab217504d39e89bb076749de8960/> add dependency insights field

### Documentation

- <csr-id-5bb1e05899962b3edf704d69e6c18caa5c23c8cd/> adding a new package manager

### Bug Fixes

- <csr-id-ba97550c74cd87cff4b86da730ae1137d71e55fc/> check for `=>` in go.mod
- <csr-id-1d6d95b2d42fffa297a19552c193f7d59220d850/> handle Cargo.toml without dependency field
- <csr-id-9311d49cd4637508c19bccd2c10883fb45b3de84/> detect `yarn.lock` with absolute directory

### Commit Statistics

<csr-read-only-do-not-edit/>

- 29 commits contributed to the release over the course of 2 calendar days.
- 3 days passed between releases.
- 11 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Better var and fn naming ([`d9e8184`](https://github.com/o2sh/onefetch/commit/d9e81846d8c2b66a999b4c351ecf2d3c377018c7)) - Add print-package-mangers cli option ([`6c4f409`](https://github.com/o2sh/onefetch/commit/6c4f4094ff54357ecd6d9b13346b18c60f89df56)) - Fix cargo clippy and go module parser and remove yarn logic ([`9415090`](https://github.com/o2sh/onefetch/commit/9415090a1cce3afd3a9ef34ffe9908ccfc1ab8d6)) - Merge pull request #304 from Luke-zhang-04/master ([`1a31c98`](https://github.com/o2sh/onefetch/commit/1a31c988d1830efd7517ee714e9c306a8d317907)) - Merge pull request #311 from yoichi/remove-unnecessary-newline ([`25babe8`](https://github.com/o2sh/onefetch/commit/25babe8c5f464b743b02e868ad1711a9fae463a2)) - Check for `=>` in go.mod ([`ba97550`](https://github.com/o2sh/onefetch/commit/ba97550c74cd87cff4b86da730ae1137d71e55fc)) - Merge branch 'master' of https://github.com/o2sh/onefetch Update branch for merging ([`aa62542`](https://github.com/o2sh/onefetch/commit/aa6254291a9796bd344c55099e63c542a61ee3df)) - Handle Cargo.toml without dependency field ([`1d6d95b`](https://github.com/o2sh/onefetch/commit/1d6d95b2d42fffa297a19552c193f7d59220d850)) - Add a comment ([`3d94170`](https://github.com/o2sh/onefetch/commit/3d94170377545529a6db0b5c50dfa59c71cca621)) - Use `map.contains_key()` instead of iterating ([`0b8caa0`](https://github.com/o2sh/onefetch/commit/0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88)) - Move `is_package_file` to Detector impl` ([`6fdb61d`](https://github.com/o2sh/onefetch/commit/6fdb61d9187308f568833227ac2a8a1c358d66b9))
    - Catch dependencies instead of `.unwrap()` ([`d60ecfe`](https://github.com/o2sh/onefetch/commit/d60ecfe3dda5b74531d77aaff6b648eac6be7799))
    - Simplify regex in package parsers ([`a19ced8`](https://github.com/o2sh/onefetch/commit/a19ced85921a17a42ad2d76b744ba765f1038341))
    - Detect `yarn.lock` with absolute directory ([`9311d49`](https://github.com/o2sh/onefetch/commit/9311d49cd4637508c19bccd2c10883fb45b3de84))
    - Change i32 in `package_parsers` to uint ([`afc7ef0`](https://github.com/o2sh/onefetch/commit/afc7ef086bcf24687735969d69afa12fa9ef090d))
    - Split deps into multiple files and replace Option with Result for better error handling ([`369506c`](https://github.com/o2sh/onefetch/commit/369506c03c3989962f635a1c6c731cafb40991d5))
    - Fix trailing white space in LOC ([`a466b06`](https://github.com/o2sh/onefetch/commit/a466b06032883be38d57678a21b8e3448f2137cc))
    - Colon is white by default ([`50f2b31`](https://github.com/o2sh/onefetch/commit/50f2b3113bb9b882298b513cf8407f898cda11e1))
    - Don't output unnecessary newline in supported() methods ([`44e47eb`](https://github.com/o2sh/onefetch/commit/44e47eb4cb69ae8ebef062939e4b13baf8d63e9c))
    - Cargo fmt ([`b966cc5`](https://github.com/o2sh/onefetch/commit/b966cc5125ecf2b66a5028d1278c6716e3c20997))
    - Add support for Cargo ([`f5ea1f7`](https://github.com/o2sh/onefetch/commit/f5ea1f7c4befcd5d0c9983b887761c24a0fc59de))
    - Add support for yarn ([`deefcb5`](https://github.com/o2sh/onefetch/commit/deefcb53631635fa76d0f331d852b5c18ba3a487))
    - Rust fmt ([`23042b4`](https://github.com/o2sh/onefetch/commit/23042b48811e54f6b8323c3901cb336b34abac72))
    - Merge branch 'master' of https://github.com/o2sh/onefetch Upstream was ahead of origin, and new .rustfmt.toml was added ([`774d2e5`](https://github.com/o2sh/onefetch/commit/774d2e56e503dc7307a7e2fdee767c1892cd8a3e))
    - Rust fmt ([`bd98e3a`](https://github.com/o2sh/onefetch/commit/bd98e3a42918e3d9edaef5937bafd3b355075a6f))
    - Add support for pip ([`d79f447`](https://github.com/o2sh/onefetch/commit/d79f447a47cbc4e4e7d591471cb217807ffc8827))
    - Add support for go modules ([`582b246`](https://github.com/o2sh/onefetch/commit/582b24609fde4a1a568cb85ee60e475d0ea8ede9))
    - Add dependency insights field ([`1c62703`](https://github.com/o2sh/onefetch/commit/1c627037f007ab217504d39e89bb076749de8960))
    - Editorconfig did stuff ([`230088d`](https://github.com/o2sh/onefetch/commit/230088d5a72010ccf43abea949e136eeb93e839f))
</details>

## v2.6.0 (2020-11-04)

<csr-id-0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88/>
<csr-id-6fdb61d9187308f568833227ac2a8a1c358d66b9/>
<csr-id-a19ced85921a17a42ad2d76b744ba765f1038341/>

### Documentation

- <csr-id-5bb1e05899962b3edf704d69e6c18caa5c23c8cd/> adding a new package manager

### New Features

- <csr-id-f5ea1f7c4befcd5d0c9983b887761c24a0fc59de/> add support for Cargo
- <csr-id-deefcb53631635fa76d0f331d852b5c18ba3a487/> add support for yarn
- <csr-id-d79f447a47cbc4e4e7d591471cb217807ffc8827/> add support for pip
- <csr-id-582b24609fde4a1a568cb85ee60e475d0ea8ede9/> add support for go modules
- <csr-id-1c627037f007ab217504d39e89bb076749de8960/> add dependency insights field
- <csr-id-10fd491eec3eefee0ebcbfaceff95a29783cc192/> add zsh and bash support

### Bug Fixes

- <csr-id-9311d49cd4637508c19bccd2c10883fb45b3de84/> detect `yarn.lock` with absolute directory

### Refactor

- <csr-id-0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88/> use `map.contains_key()` instead of iterating
  using map.contains_key is easier to understand and probably has better
  lookup time
- <csr-id-6fdb61d9187308f568833227ac2a8a1c358d66b9/> move `is_package_file` to Detector impl`

- <csr-id-a19ced85921a17a42ad2d76b744ba765f1038341/> simplify regex in package parsers

### Commit Statistics

<csr-read-only-do-not-edit/>

- 90 commits contributed to the release over the course of 15 calendar days.
- 15 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Fix colors on java logo and add truecolors to jupyter ([`b55cc16`](https://github.com/o2sh/onefetch/commit/b55cc16295315a386c8fa943c40b493690427dcf)) - Merge pull request #309 from o2sh/remove_tokio ([`bdd3867`](https://github.com/o2sh/onefetch/commit/bdd386799b24d784c518da71ad016591361dc8f6)) - Remove async/await ([`fad08bd`](https://github.com/o2sh/onefetch/commit/fad08bd68f370db9b1742bbc1a7d606bb6029a4f)) - Fix #307: remove CheckSupportedBackend and Result type for add_image ([`f2bb5b0`](https://github.com/o2sh/onefetch/commit/f2bb5b0f211061bc9059649b0bf2f8a9d8d1ff6b)) - Fix clippy warnings and rename iterm2 to iterm ([`ce00e62`](https://github.com/o2sh/onefetch/commit/ce00e62a5dcf22623edaf9c5d34a3cf37abd049b)) - Add rustfmt.toml ([`71230ce`](https://github.com/o2sh/onefetch/commit/71230ce111fa2763b268fd39be979099407ed674)) - Merge pull request #305 from yoichi/iterm2-support ([`492cb5f`](https://github.com/o2sh/onefetch/commit/492cb5fc0b989ce13cb13c763aa108e7b3b7d029)) - Cargo fmt ([`47dbab3`](https://github.com/o2sh/onefetch/commit/47dbab33654a0037db45c63c53683c23e2b822dd)) - Don't cause error if --image-backend=iterm2 is specified ([`d821bc3`](https://github.com/o2sh/onefetch/commit/d821bc350841b1c6fe3b683ba0eaae5a2b947c80)) - Fix error message ([`832e4c5`](https://github.com/o2sh/onefetch/commit/832e4c50b1bbcb5441f52882bb19f983bb92cf9c)) - Cargo fmt ([`a0f10cd`](https://github.com/o2sh/onefetch/commit/a0f10cdbb813caf3724ee25b62263007e414268d)) - Check for KITTY_WINDOW_ID for kitty support ([`b6b5f49`](https://github.com/o2sh/onefetch/commit/b6b5f496fe424e322455b9aadcba324753c00d7e)) - Fix typo ([`6581979`](https://github.com/o2sh/onefetch/commit/65819792268ead7ab5f834bdb8827ea4d322e098)) - Rename dependencies --> deps ([`596f2cf`](https://github.com/o2sh/onefetch/commit/596f2cfb92caa5d8a9b97faeea9b3477bf30d5f8)) - Match on Option after parsing number of deps ([`cd63dae`](https://github.com/o2sh/onefetch/commit/cd63dae64ff5da27089294326bca6c05eb2d8570)) - Fix get_deps() implementation --WIP ([`1c65108`](https://github.com/o2sh/onefetch/commit/1c65108270c195a79b2997c9dcc4e1d6be93a4ef)) - Add iTerm2 image protocol support ([`9b10bf6`](https://github.com/o2sh/onefetch/commit/9b10bf6e7ff7e53969da14d189b4fcf21ab6d0b5)) - Begin dependency feature ([`5a9ec69`](https://github.com/o2sh/onefetch/commit/5a9ec6946f5ac677fb9a9a027a40b23f276ea6cf)) - Add truecolors to dart and haskell ([`9011596`](https://github.com/o2sh/onefetch/commit/9011596022e797b854b5eeb07b945868078cce8b)) - Clean java ascii logo ([`e92ae3a`](https://github.com/o2sh/onefetch/commit/e92ae3a0a3c5252688ec99d72fa3a6993e258ead)) - Merge pull request #303 from Cogitri/raku ([`7a08f9b`](https://github.com/o2sh/onefetch/commit/7a08f9b26fcadf16cbf9d6611531478df5f5f9d1)) - Clean raku ascii + use of truecolors ([`b4b4dd1`](https://github.com/o2sh/onefetch/commit/b4b4dd1813232c7f20783c8397c6069042f5260f)) - Add Roku (Perl 6) ASCII logo ([`ebb0229`](https://github.com/o2sh/onefetch/commit/ebb02297832b8ea6f2ea428bc670c5a832b7d79f)) - Clean scala ascii logo ([`2685c69`](https://github.com/o2sh/onefetch/commit/2685c6976783d229dae22343d2f0262f797a3d9f)) - Improved help section ([`249b5b3`](https://github.com/o2sh/onefetch/commit/249b5b3a32c86d074e074e231aa441d84a05088a)) - Merge pull request #300 from o2sh/refacto-info-display ([`690965b`](https://github.com/o2sh/onefetch/commit/690965bac528ce94a75beca472e6b3f7dd2be93b)) - Tuple deconstruction in get_author_field ([`a51b2f7`](https://github.com/o2sh/onefetch/commit/a51b2f7c8248adee664b66f8042408240c97298b)) - Simplify get_formatted_subtitle_label ([`acc68d9`](https://github.com/o2sh/onefetch/commit/acc68d99fcc70069ed75f6cff7324ef10975f45a)) - Improve get_language_field ([`aaa80b1`](https://github.com/o2sh/onefetch/commit/aaa80b131076fc8533f764417fa0dd308806a9bf)) - Refacto info::display ([`5e8867a`](https://github.com/o2sh/onefetch/commit/5e8867aef9618900db45bfb9a6877cbb6137a458)) - Add man page ([`689f936`](https://github.com/o2sh/onefetch/commit/689f936efb210cd8a141eadb6246998e1b407f75)) - Clean rust, javascript, typescript ascii logos ([`6c4620a`](https://github.com/o2sh/onefetch/commit/6c4620a3d6fcc3745586ba3436c4e01b9f4ae7a3)) - Merge pull request #295 from rogercyyu/feature/text-coloring ([`a1cdcad`](https://github.com/o2sh/onefetch/commit/a1cdcad14a6795b3ed0925154876c9caa3b0e8b3)) - Addressed PR changes ([`a23926c`](https://github.com/o2sh/onefetch/commit/a23926c41e6ba24805dc6cc9e9825a577ae94cbf)) - Merge pull request #296 from o2sh/dependabot/cargo/image-0.23.11 ([`0df388c`](https://github.com/o2sh/onefetch/commit/0df388c9c9a88f5877f391c76fa0cb982b09c1cf)) - Merge pull request #299 from o2sh/ci-fail-on-deprecated ([`e14437f`](https://github.com/o2sh/onefetch/commit/e14437f02818a08130c17d49ea6444912ad1c948)) - Rust fmt ([`f6e2430`](https://github.com/o2sh/onefetch/commit/f6e24301fc2cc4f31f638dea99aa5319f67ee53a)) - Fixed incorrect scope for custom_color ([`a8af107`](https://github.com/o2sh/onefetch/commit/a8af107b3e0325972ee8ca62591b0559669edf60)) - Make build fail on deprecated ([`b542311`](https://github.com/o2sh/onefetch/commit/b5423111e7d7934a372131aed72a706e8e046ff9)) - Fixed colors for default use ([`aee65ec`](https://github.com/o2sh/onefetch/commit/aee65ec0e52777832ce271897fe5a0e301d490b7)) - Use color_quant::NeuQuant ([`e8cfcfe`](https://github.com/o2sh/onefetch/commit/e8cfcfe8112e34f782a4537ff6214762c8744b4a)) - Cleaner c c++ c# ascii logos ([`c58d6a6`](https://github.com/o2sh/onefetch/commit/c58d6a6211185eae57047c209878bd4c2ecae77f)) - Fixed color bug in langauges ([`419f917`](https://github.com/o2sh/onefetch/commit/419f917262dd25514a03e3fb1b389b52c84a32b4)) - Add feature: text coloring ([`f9dac47`](https://github.com/o2sh/onefetch/commit/f9dac4751357cbaeef3fef4b3ae0e0b497b887d5)) - Formatting ([`3c89824`](https://github.com/o2sh/onefetch/commit/3c89824e4a10f9939398003a8c0a18ada066480e)) - Initial text coloring for review ([`29d41f8`](https://github.com/o2sh/onefetch/commit/29d41f84f3af9ba4b0e24b3a7c39b8af3f238ed3)) - Initial coloring work ([`50b9808`](https://github.com/o2sh/onefetch/commit/50b98086d818b74ea7b85b162b61c976877a0664)) - Merge pull request #284 from Luke-zhang-04/master ([`0180a36`](https://github.com/o2sh/onefetch/commit/0180a36a15c4695c69b807226055616c475d20cd)) - Merge pull request #289 from o2sh/remove-panic-from-info-fmt ([`ab2e9f3`](https://github.com/o2sh/onefetch/commit/ab2e9f369a32c532569780a7d8f5994b06fbec4f)) - Merge branch 'master' of https://github.com/o2sh/onefetch This branch is behind on same changes. ([`c92c3d4`](https://github.com/o2sh/onefetch/commit/c92c3d45983ef4d3b2b2ba63fef210946efaa408)) - Make JSX yellow ([`2773c32`](https://github.com/o2sh/onefetch/commit/2773c324a3ef8fe600cea979c3d28a4a2bfe3cf9)) - Merge branch 'master' into remove-panic-from-info-fmt ([`949db39`](https://github.com/o2sh/onefetch/commit/949db3971a89461ba8f24c76ae4382138a52f7cc)) - Add requires param to color-resolution flag ([`2b2aeaa`](https://github.com/o2sh/onefetch/commit/2b2aeaa225b4075b2969db737a416f5e6427d8b8)) - Merge branch 'master' into remove-panic-from-info-fmt ([`4fcc458`](https://github.com/o2sh/onefetch/commit/4fcc45867a96be8102c64f91113a2c30027cbc40)) - Rust fmt ([`1c07cfb`](https://github.com/o2sh/onefetch/commit/1c07cfb497c690c31ae7f55190eaf5608a2d5494)) - Fix useless break line ([`61596a6`](https://github.com/o2sh/onefetch/commit/61596a6f096395ee8ee4ef80643b9ea8bfd83c9b)) - Better clap arg for --color-resolution ([`42a64dd`](https://github.com/o2sh/onefetch/commit/42a64dd2b6278626856a53c56504fed359dceacf)) - Rust fmt ([`11f86be`](https://github.com/o2sh/onefetch/commit/11f86be42871f91ea68f7b97565f6c60e2998050)) - Merge pull request #271 from yoichi/sixel-more-color ([`69306bd`](https://github.com/o2sh/onefetch/commit/69306bd1ef30657b18567d8df41ce78a11ee451d)) - Merge branch 'master' into sixel-more-color ([`3648aaa`](https://github.com/o2sh/onefetch/commit/3648aaa13409eae472837eed7135d5cae3ccbc88)) - Fix --off flag after bad merge ([`c2883d1`](https://github.com/o2sh/onefetch/commit/c2883d1c2de8b31d369d10e8ae9c523bb725543e)) - Merge pull request #288 from akrantz01/disable-ascii-art ([`522514d`](https://github.com/o2sh/onefetch/commit/522514d04c65c615f5ae458bca94b59202179c45)) - Merge branch 'master' into disable-ascii-art ([`3bf51c5`](https://github.com/o2sh/onefetch/commit/3bf51c5b3d989803c17177da6eeec0e279a9fe72)) - Fix cargo clippy warn in language.rs ([`0b37765`](https://github.com/o2sh/onefetch/commit/0b37765138a5bd9d2423ae567bd227add7c0e62c)) - Merge pull request #291 from o2sh/refacto/printer ([`a72dab5`](https://github.com/o2sh/onefetch/commit/a72dab5b32c1ceabf51b1b7553205b6b2481ed69)) - Update src/onefetch/cli_utils.rs ([`16db642`](https://github.com/o2sh/onefetch/commit/16db642c21f5253543538691ee84b6ef0c6ec071)) - Refacto info.rs by adding printer ([`59a2033`](https://github.com/o2sh/onefetch/commit/59a203348e34ed15661ef39ad4e76820f6b7b1eb)) - Implement suggestions ([`f705b01`](https://github.com/o2sh/onefetch/commit/f705b010ee3651858ca9351ba44460712dfddc87)) - Fix windows build ([`d8d0ff7`](https://github.com/o2sh/onefetch/commit/d8d0ff78da8e66ca3e9e8e34b4e2861a9e65c75b)) - Rust fmt ([`84895ab`](https://github.com/o2sh/onefetch/commit/84895abe6ca21c34048dab3449d2af0d394ae958)) - If windows: no supported image backend ([`84cb871`](https://github.com/o2sh/onefetch/commit/84cb871bd72c49f819f9702c8ea335d9798e20bc)) - Return Err when image but no image backend ([`8035b97`](https://github.com/o2sh/onefetch/commit/8035b9758733af51f3a85ecfd4c63ec5ee3230f0)) - Add CLI option to disable image ([`0d4aa2f`](https://github.com/o2sh/onefetch/commit/0d4aa2f49fd468c8a424f17082e5d81ad871b27f)) - Makes the grammar among the option arg help more consistent ([`0a3eadb`](https://github.com/o2sh/onefetch/commit/0a3eadbee806f6839728df4daef5396d61f69678)) - Merge branch 'master' of https://github.com/o2sh/onefetch ([`4c559ab`](https://github.com/o2sh/onefetch/commit/4c559ab1b86c0b1842236f235b11144a29d25fd8)) - Add support for JSX and TSX ([`6d84935`](https://github.com/o2sh/onefetch/commit/6d84935fa2c2b24dfdcbf86981d4d310f3a7e96b)) - Restrict values of image colors ([`1226801`](https://github.com/o2sh/onefetch/commit/122680113ee60fb529b2c43c31eed92223a95c66)) - Merge pull request #279 from KaindlJulian/ascii-flag ([`1ba074a`](https://github.com/o2sh/onefetch/commit/1ba074a8d58fb4d98c6c94bfc9e692bb0ae62abe)) - Merge declaration of logo_lines with test on ascii_input ([`46dea25`](https://github.com/o2sh/onefetch/commit/46dea2502ebcf6bb32319afd46df591f324e4bcc)) - Add validations and long help for ascii flag ([`cbfe831`](https://github.com/o2sh/onefetch/commit/cbfe8313cfe840432caecbe8f4d559ebb1844d5c)) - Avoid unnecessary call ([`a72b1ae`](https://github.com/o2sh/onefetch/commit/a72b1aeb33a5e3e14eefb4fb7a4acba0a8a32aed)) - Fix go logo and bash ([`ac74497`](https://github.com/o2sh/onefetch/commit/ac74497aadb21a6e21734b2c93f5cdc87f6df94c)) - Merge pull request #276 from Luke-zhang-04/master ([`0e11538`](https://github.com/o2sh/onefetch/commit/0e11538250cb27bd8050fd0b45ea03e6cc1d91b3)) - Merge with upstream ([`d8506af`](https://github.com/o2sh/onefetch/commit/d8506afc84c712604ba241243ae21eadd49a6a38)) - Make ZSH it's own language and add ASCII ([`6d50660`](https://github.com/o2sh/onefetch/commit/6d50660cfbdb6009787c09d1329b20ac730762ed)) - Merge pull request #252 from atluft/235-truecolor-define-color ([`7f0c08c`](https://github.com/o2sh/onefetch/commit/7f0c08c8c5374f391819a9d69cbed94cefcdcc24)) - Make BASH it's own language and add ASCII ([`a7b872c`](https://github.com/o2sh/onefetch/commit/a7b872c8595bf400d9df8efc67246ba8f779f018)) - Add ascii flag ([`2e48d98`](https://github.com/o2sh/onefetch/commit/2e48d98e0923fa0233066c90831937ef40afd401)) - Merge branch 'master' into 235-truecolor-define-color ([`c465eeb`](https://github.com/o2sh/onefetch/commit/c465eeb02fa5ced3050c7344001c5689e3ad80f7)) - Update src/onefetch/language.rs ([`1d73e42`](https://github.com/o2sh/onefetch/commit/1d73e42c5c9f97eb7d8b2014eb806789c4219c2a))
</details>

## v2.5.0 (2020-10-19)

### New Features

- <csr-id-aebf7434302d6884532f3427ec6b6021e2fe4adb/> show number of tags and branches
  This simply counts and displays the number of branches and tags that the repository has locally.
- <csr-id-10fd491eec3eefee0ebcbfaceff95a29783cc192/> add zsh and bash support

### Commit Statistics

<csr-read-only-do-not-edit/>

- 83 commits contributed to the release over the course of 14 calendar days.
- 15 days passed between releases.
- 2 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Add zsh and bash support ([`10fd491`](https://github.com/o2sh/onefetch/commit/10fd491eec3eefee0ebcbfaceff95a29783cc192)) - Rust fmt ([`b18d459`](https://github.com/o2sh/onefetch/commit/b18d459d656f5204ff2bdb0dda1d03d819b61c20)) - Add number validator for --authors flag ([`2e6f308`](https://github.com/o2sh/onefetch/commit/2e6f308d02b964c6299023ad06064fd0dc8a1b99)) - Merge pull request #268 from nguyenquannnn/master ([`b4cf5ec`](https://github.com/o2sh/onefetch/commit/b4cf5eca043f04f9bfabec70fd521c92195dcb2e)) - #235 accepted all review suggestions ([`36a9e9c`](https://github.com/o2sh/onefetch/commit/36a9e9c0f28905efc24f375c45bdb778c6a1c708)) - Merge pull request #270 from yoichi/fix-layout-with-sixel ([`37e6fa3`](https://github.com/o2sh/onefetch/commit/37e6fa34e80a15a65fe88c025f0172fd835a2d53)) - Add third color for Go ASCII logo ([`f2a5767`](https://github.com/o2sh/onefetch/commit/f2a5767979b5474d7b6f467fc196e1e0964e5eb9)) - Merge branch 'master' of github.com:o2sh/onefetch ([`6aea8a1`](https://github.com/o2sh/onefetch/commit/6aea8a1af71185b9a708f25093b37518545553b9)) - 233 Update Go ascii art ([`2ad5ca7`](https://github.com/o2sh/onefetch/commit/2ad5ca7736b4d191625ec94e6c43640f648e5cae)) - New option --image-colors to specify colors used in image backends ([`83459a8`](https://github.com/o2sh/onefetch/commit/83459a886530e42ff488aed191c823735dfd782f)) - Move is_true_color_terminal() in cli.rs ([`e8fdd42`](https://github.com/o2sh/onefetch/commit/e8fdd422cdf503a3e4cfebddd32500a0e0c746ac)) - #235 moves use into test module for build ([`e667b23`](https://github.com/o2sh/onefetch/commit/e667b2320648a35f31651cb38b5e0ca4f0bc19e5)) - #235 adds tests for equal array size and basic colors ([`fca7e74`](https://github.com/o2sh/onefetch/commit/fca7e7461ac7802241f8ead78b2a1b2463d51d08)) - Use more color (16 -> 128) in sixel graphics ([`7ccc949`](https://github.com/o2sh/onefetch/commit/7ccc9493fcc5d25028a310f52b32945693ff0440)) - Avoid calculation mistakes in full screen display ([`ce36353`](https://github.com/o2sh/onefetch/commit/ce36353bfce4a5ebb42234cd99213066317d6d49)) - Avoid moving too much upwards ([`c370cb4`](https://github.com/o2sh/onefetch/commit/c370cb4139a6aeca8834798208a0ec36250c06ce)) - Refacto info.rs ([`8bcab7b`](https://github.com/o2sh/onefetch/commit/8bcab7b09dc1dc4612bfdad15446cd06835e15d6)) - Remove future::join! inside get_number_of_tags_branches + exclude non async func from future::join! #269 ([`43db0dc`](https://github.com/o2sh/onefetch/commit/43db0dc95064dda88a13f02053586f0a27ccc6a0)) - Merge pull request #267 from o2sh/enable-ascii-size-test ([`69e6452`](https://github.com/o2sh/onefetch/commit/69e6452a0b9d9c2d1773e0037037a33482d92d37)) - #235 fix ([`410b849`](https://github.com/o2sh/onefetch/commit/410b849e4bc3069457cea7ba4a5f3c1cb87cd8b1)) - #235 improving colorterm test by separating into a function ([`c0735e1`](https://github.com/o2sh/onefetch/commit/c0735e1b38135ca764aa739799d2d32387db75f8)) - #235 define colors from define languages invocation ([`074e238`](https://github.com/o2sh/onefetch/commit/074e238026c7fa5a96b957005c49e628865f682d)) - \#233 Update Go ascii art ([`6b300af`](https://github.com/o2sh/onefetch/commit/6b300af8a5bb348ab822fa02ef366f789e566c19)) - Merge pull request #266 from yoichi/enable-image-backend-on-macos ([`792850e`](https://github.com/o2sh/onefetch/commit/792850eb6cb30847dc507d17ed4f827db1c38093)) - Replace condition: target_os = "windows" -> windows ([`0cf7398`](https://github.com/o2sh/onefetch/commit/0cf7398bdb84dd281d888f970e88a47ace4bbebf)) - Enable-ascii-size-test ([`6f638a0`](https://github.com/o2sh/onefetch/commit/6f638a02b5cee900bd1963075f9683ad0b877950)) - Reduce size of Prolog ASCII logo, fix #261 ([`85fc5e3`](https://github.com/o2sh/onefetch/commit/85fc5e3eb1d1c952ce8f015aff1f6d890ab0e292)) - Improve ASCII logo for javascript, typescript, lisp, python ([`49d11f6`](https://github.com/o2sh/onefetch/commit/49d11f6d3d541821b0ca8bb0f94df978fdcc02aa)) - Merge pull request #265 from o2sh/fix/markdown-jupyter-stats ([`2acd7e9`](https://github.com/o2sh/onefetch/commit/2acd7e93d219ffebf20aa19eaf6147e262b2db80)) - Refacto total == 0 condition ([`9bd6e89`](https://github.com/o2sh/onefetch/commit/9bd6e89ceac828d4c66d2fd971ee5cf2cc670105)) - Enable image backends on macOS ([`f82011e`](https://github.com/o2sh/onefetch/commit/f82011e76af9210c1c55b7e202df25eb22389942)) - Small refacto ([`149ede0`](https://github.com/o2sh/onefetch/commit/149ede083b7926c0df4b43be72142f343ff2efaa)) - Fix calculation of language distribution ([`e41f33e`](https://github.com/o2sh/onefetch/commit/e41f33ed077cdfa0ac9f42044d70c65163452047)) - Takes into account language.children ([`0a39044`](https://github.com/o2sh/onefetch/commit/0a390442d523e091f905709709dd9255a4dcca58)) - Renable -l flag ([`462e63a`](https://github.com/o2sh/onefetch/commit/462e63a8f9c5ea1b14d5e42985c6a7e3f647b58e)) - Merge pull request #262 from nguyenquannnn/master ([`fdb6729`](https://github.com/o2sh/onefetch/commit/fdb6729a246451fbe549ee2a1501b3d9fcdb47e9)) - 234 Update Rust ascii art ([`d9d9a87`](https://github.com/o2sh/onefetch/commit/d9d9a877f373cc312472f1c566e7f0fb5e73de32)) - Merge pull request #253 from o2sh/refacto-main ([`a2b0bfd`](https://github.com/o2sh/onefetch/commit/a2b0bfd5112a7baad0fa1ab449325231eaba9188)) - Use CARGO_MANIFEST_DIR instead of relative path ([`a8f345d`](https://github.com/o2sh/onefetch/commit/a8f345d4092e5f234e8b821d1b3322a2c6b5a00e)) - Simplify no_bold logic ([`be96551`](https://github.com/o2sh/onefetch/commit/be96551501d4db0324ee937aa827e7663ecaa532)) - Merge Options with cli ([`8419edd`](https://github.com/o2sh/onefetch/commit/8419eddffe62c97fd3ed6d17b4a51dd55c890aee)) - Extract info_fields into its own module ([`5e595a5`](https://github.com/o2sh/onefetch/commit/5e595a5ddfea01ca850c58d8bb60eebaaa379d63)) - Fix possible values for -a flag ([`4d69c11`](https://github.com/o2sh/onefetch/commit/4d69c11a3d028c14adf27db89f5a4c94ec280bfd)) - Rearrange files ([`7fd3b56`](https://github.com/o2sh/onefetch/commit/7fd3b565462799555f1771f0df47304aceee11f7)) - Rustfmt ([`f9e86a0`](https://github.com/o2sh/onefetch/commit/f9e86a0be402dfbb489e142a0b7509353abb42c0)) - Use error_chain for error handling ([`444f3b2`](https://github.com/o2sh/onefetch/commit/444f3b299842afa2b0c20b085c5aee7c51503108)) - Fix error handling ([`24a3666`](https://github.com/o2sh/onefetch/commit/24a36668790b8a052c403cd4ba9b42d8ea426051)) - Try fix build ([`a2350af`](https://github.com/o2sh/onefetch/commit/a2350af350b1280daed3adfc3e7217dabfd94403)) - Further refactoring of main.rs ([`2b9f425`](https://github.com/o2sh/onefetch/commit/2b9f425bdaa5556d72909119c9b70bf6e1d4dd12)) - Refacto declaration of option struct ([`71787c3`](https://github.com/o2sh/onefetch/commit/71787c3883e33d44ce46d194b4eac6c3832e1f46)) - Add possible values for ascii_colors ([`038b3c2`](https://github.com/o2sh/onefetch/commit/038b3c2d501a25604354bca5b336f2ac553a60ba)) - Split/refacto main module ([`62697c4`](https://github.com/o2sh/onefetch/commit/62697c41aeba26bbcfa60eb593164c52b0403aa5)) - #235 true colors using color define macro ([`bee2bf7`](https://github.com/o2sh/onefetch/commit/bee2bf7957e05932405e20a68239fe9634163c2b)) - Merge pull request #246 from o2sh/hotfix/fix-number-of-branches ([`e334176`](https://github.com/o2sh/onefetch/commit/e3341760b9e2c93a1d0f3ad1e02ffbc57e0e0062)) - Fix underflow when no remote branch ([`fa63290`](https://github.com/o2sh/onefetch/commit/fa63290295548565bcb77da2cd020239ba971020)) - Merge pull request #245 from o2sh/hotfix/remote-branch ([`ec8a9e4`](https://github.com/o2sh/onefetch/commit/ec8a9e43c19bd10d14053d28168ffda92dba2a2c)) - Match on literal . ([`75def6c`](https://github.com/o2sh/onefetch/commit/75def6c74e0a2e0595e78f776a7a694e887690b7)) - #235 true color using define_colors macro from define_language ([`56dd1c1`](https://github.com/o2sh/onefetch/commit/56dd1c178b3c86af5f3bbe91b08d95518256ce5a)) - Switch order tags/branches ([`5afe7b2`](https://github.com/o2sh/onefetch/commit/5afe7b245b3caa476f9499e0e9b955033017eb70)) - Fix number of branches to ignore HEAD ([`cc122c4`](https://github.com/o2sh/onefetch/commit/cc122c41cb5c49391e63f2a2c1ff90cb41000d9a)) - Regex matching for remote.url ([`c94ebe9`](https://github.com/o2sh/onefetch/commit/c94ebe9737d18b73e34e82026b9fd48c56a3174a)) - Fix rust fmt ([`e710018`](https://github.com/o2sh/onefetch/commit/e710018c0badb98386cef567226df3f42771d963)) - Merge pull request #243 from yoichi/identify-author-by-email ([`ca58871`](https://github.com/o2sh/onefetch/commit/ca588716efdd4c73151b18ad6a230fa766422910)) - Clarify variable names ([`c68e021`](https://github.com/o2sh/onefetch/commit/c68e021046d468777cb424dbad36ad40e880307d)) - Identify author by email ([`5044179`](https://github.com/o2sh/onefetch/commit/5044179251365426b3cb28465ee160ee94ef0821)) - Small refactoring of #237 ([`13b9727`](https://github.com/o2sh/onefetch/commit/13b9727814d575456b374f3142c72dae2fd31200)) - Merge pull request #237 from reime005/feat-show-tags-branches ([`55d0498`](https://github.com/o2sh/onefetch/commit/55d0498f5d2709e54cc37fcf0e6250373d28b746)) - Show number of tags and branches ([`aebf743`](https://github.com/o2sh/onefetch/commit/aebf7434302d6884532f3427ec6b6021e2fe4adb)) - Reduce to 2 languages per line ([`812283b`](https://github.com/o2sh/onefetch/commit/812283b4a41431d3639f19f148f5505e4446535c)) - White spheres in jupyter-notebook ASCII logo ([`020409b`](https://github.com/o2sh/onefetch/commit/020409b2aa541de7a9e044403d559a48f4d252d8)) - Merge pull request #231 from maash3r/ascii/jupyter ([`db92791`](https://github.com/o2sh/onefetch/commit/db9279138a28f2c23fa2375f4822e275fcd912d9)) - Changed jupyter ASCII color from black to blue ([`920958d`](https://github.com/o2sh/onefetch/commit/920958dcb7c252402ac8ba297ae3b9fdd91a5275)) - Merge pull request #232 from rootEnginear/master ([`1f09bcc`](https://github.com/o2sh/onefetch/commit/1f09bcc8cde94d58bbf8e5755fda6766d4b76f6e)) - Edit ASCII modifier, Update PHP pallette ([`b194754`](https://github.com/o2sh/onefetch/commit/b194754a58e12d73ddbc8d8dd171243e643d15fa)) - Change PHP logo ([`802d883`](https://github.com/o2sh/onefetch/commit/802d8835f681dcbf3ee37ab38d6ceeaf1a0ddf5d)) - Fixed color for Jupyter in language.rs ([`8af3fd3`](https://github.com/o2sh/onefetch/commit/8af3fd34b5c67cf866701d2b2a71d83e3f086a22)) - Updated colors for Jupyter in language.rs ([`143d066`](https://github.com/o2sh/onefetch/commit/143d06669b46829ec87631ef2366d0a45a42a24e)) - Refacto get_repo_name function ([`2719c63`](https://github.com/o2sh/onefetch/commit/2719c6373fc1d9979f99335bd03224ba0795d237)) - Refacto get_repo_name function ([`87453b2`](https://github.com/o2sh/onefetch/commit/87453b28e7713c3494a4a6bf124192dfe9a50093)) - Fix get_repo_name when cloning from url that ends with forward slash ([`82cc9a2`](https://github.com/o2sh/onefetch/commit/82cc9a29d7fface8c828f4bef81d1f58b1ab2dd9)) - Trim ASCII logo lines before checking for max width ([`8c9ccca`](https://github.com/o2sh/onefetch/commit/8c9ccca614092847df4ce4ca0bf092749e139b95)) - Merge pull request #218 from maash3r/asciis ([`064f7e1`](https://github.com/o2sh/onefetch/commit/064f7e117303406aac9b269e47d5bb9cb2ab689b)) - Updated language.rs for Lua having 2 colors ([`b2fb340`](https://github.com/o2sh/onefetch/commit/b2fb34017a16ad61f306baa5497e3b28aa2d1412))
</details>

## v2.4.0 (2020-10-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 33 commits contributed to the release over the course of 42 calendar days.
- 61 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Merge pull request #216 from MarkusPettersson98/lang/crystal ([`879e57c`](https://github.com/o2sh/onefetch/commit/879e57c1cbc33307345d26a6f9c6d20d77eb6ce1)) - Inverse black and white in Crystal ascii art Increase readability in a black terminal. ([`9927d95`](https://github.com/o2sh/onefetch/commit/9927d95b63c795db96576391db8182c11eaa06a3)) - Add support for the Crystal programming language. ([`a6665f6`](https://github.com/o2sh/onefetch/commit/a6665f60565b7b3e351cab248111d5eb0e4e9c7d)) - Better naming for some functions ([`96ef592`](https://github.com/o2sh/onefetch/commit/96ef592df952a2a280b209e136da875bce042cc7)) - Merge pull request #211 from o2sh/feature/single-git-log ([`5482a10`](https://github.com/o2sh/onefetch/commit/5482a105eb8deb2f1fb08f90b742f905646526aa)) - Apply suggestions by spenserblack ([`ede6c92`](https://github.com/o2sh/onefetch/commit/ede6c92b31166c721b8b101f687f1719a5763706)) - Re-use git history in get_last_change() ([`91ce698`](https://github.com/o2sh/onefetch/commit/91ce69838af0f721e45028bb2ad357aa35b5415c)) - Use git log once ([`f3d7335`](https://github.com/o2sh/onefetch/commit/f3d733577da0709e63b62cdc72980f709a2900b9)) - Merge pull request #206 from o2sh/feature/language-def-macro ([`7cafe85`](https://github.com/o2sh/onefetch/commit/7cafe853e6eeffb6c6ff12dfd162e4f38ec1051a)) - Cargo fmt ([`5d438f8`](https://github.com/o2sh/onefetch/commit/5d438f81fda7ade99c6d8fbbf197f1aa70e01117)) - Resolve Cargo.lock conflict ([`ba8a001`](https://github.com/o2sh/onefetch/commit/ba8a001c0b0743f54109c0b8ca28b60c2f0c4237)) - Ignore ASCII size assertions by default ([`152678c`](https://github.com/o2sh/onefetch/commit/152678c4c53ac193db7ccf99683a0a8379a67ab8)) - Enforce ASCII size with tests ([`fc3d8a1`](https://github.com/o2sh/onefetch/commit/fc3d8a13f34fb105af763c520d9cfd146be60d2e)) - Define available languages with macro ([`100d770`](https://github.com/o2sh/onefetch/commit/100d770825b08f9adbbb36d78ae77320e4626f09)) - Define language colors in macro ([`536bf76`](https://github.com/o2sh/onefetch/commit/536bf76b08eb36ec5e94a8119129e8c17a546f41)) - Enforce trailing comma ([`e76b8ec`](https://github.com/o2sh/onefetch/commit/e76b8ec85bd44ac1475aaf629a30cff26c6c5c5f)) - Define languages with macro ([`800d998`](https://github.com/o2sh/onefetch/commit/800d998abf5b2c451ec5ce8a22b3b7e0cd3c9744)) - Merge pull request #199 from o2sh/feature/tokio-command ([`ebe5b5b`](https://github.com/o2sh/onefetch/commit/ebe5b5bca9ee200a27c5d84169d300345e3bcf34)) - Remove unused use ([`b1c831d`](https://github.com/o2sh/onefetch/commit/b1c831df1b877d9f478ea021435b179eada945bb)) - Info ctor marked as async ([`1f0bb64`](https://github.com/o2sh/onefetch/commit/1f0bb64c38da3f34513524c264b2fae0487f903a)) - Tokio command for non blocking sys call ([`e9d7d7c`](https://github.com/o2sh/onefetch/commit/e9d7d7c0f6530525fed52906008cef2108793e1f)) - Fix cargo clippy warnings ([`a5c3c93`](https://github.com/o2sh/onefetch/commit/a5c3c938a7d9b691fa200b53cf03ef50d352e1fc)) - Merge pull request #193 from o2sh/dependabot/cargo/colored-2.0.0 ([`873fc45`](https://github.com/o2sh/onefetch/commit/873fc453957f5fa7ae7631b0760bd9b5ac49b272)) - Merge branch 'master' into dependabot/cargo/colored-2.0.0 ([`2f9587f`](https://github.com/o2sh/onefetch/commit/2f9587f0b860bbba7a4989447e8c08620a4b9c48)) - Merge pull request #187 from o2sh/dependabot/cargo/strum-0.19.2 ([`547bf25`](https://github.com/o2sh/onefetch/commit/547bf25201794adafcaf60c28097e4e880ba3a83)) - Fix strum breaking changes ([`366e26d`](https://github.com/o2sh/onefetch/commit/366e26dc77c856fac4a75f176cbeffb1d0fe97e4)) - Get strum_macros as feature ([`ef75b49`](https://github.com/o2sh/onefetch/commit/ef75b497ff86999592e4e9c1424694ceab9cdf68)) - Force colored to attempt colorization in test ([`39af779`](https://github.com/o2sh/onefetch/commit/39af779554c2c544297f142079c65ed7babd5232)) - Merge pull request #186 from o2sh/feature/async-await ([`b1d9c22`](https://github.com/o2sh/onefetch/commit/b1d9c22dc8a2d5721e7940a95928cda8c1348877)) - Async block ruturns Result for better error handling ([`1160e88`](https://github.com/o2sh/onefetch/commit/1160e88222c0850a047ec83959e9e4bd8f058616)) - Refacto use stmts and replace get_info_lines with async block ([`bcb4e64`](https://github.com/o2sh/onefetch/commit/bcb4e649d30f65d75e39cc33fbb8135feb191cf3)) - Async get_info_line ([`42f56c1`](https://github.com/o2sh/onefetch/commit/42f56c16d00dd5dbd620d2cb9449647c1514aaaf)) - Bump to edition 2018 to enable async/await syntax #185 ([`99bff66`](https://github.com/o2sh/onefetch/commit/99bff6636c915d115ef2fd9faff024056223fbe2))
</details>

## v2.3.0 (2020-08-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 15 commits contributed to the release over the course of 169 calendar days.
- 211 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Remove Bright Colors #179 ([`8de4835`](https://github.com/o2sh/onefetch/commit/8de48350aa33491af226a138a946b248d7c2b6bb)) - Fix trailing slash in exclude pattern #178 ([`b415cc2`](https://github.com/o2sh/onefetch/commit/b415cc2597d0ee007b501d57676c995ca62aa981)) - Fix exclude OPTION to work wiht absolute paths #178 ([`c8cf756`](https://github.com/o2sh/onefetch/commit/c8cf756fe55d7d26b8540fa0fb4d6fbf1693fe68)) - Fix typo in cli help ([`f7023d7`](https://github.com/o2sh/onefetch/commit/f7023d7d96a9ea5ffb5f1621e36a34ee02a7dc46)) - Replace directory OPTIONS with input ARGS ([`a9b775a`](https://github.com/o2sh/onefetch/commit/a9b775afefa2d7c024a551f555c145b4fa087dd7)) - Jupyter notebook support #151 ([`65e6ac4`](https://github.com/o2sh/onefetch/commit/65e6ac4c25c297277277ab10a8ddc2da382d08c2)) - Remove is_root bool from get_language ([`a217f1f`](https://github.com/o2sh/onefetch/commit/a217f1f452a9d21ccc93dff319e6cb3471420b3b)) - Add --exlude option ([`6003e04`](https://github.com/o2sh/onefetch/commit/6003e0454e563f34d3557dc1f90670cda41cc6a1)) - Add possible values to ascii colors option ([`81e5334`](https://github.com/o2sh/onefetch/commit/81e53347b7a43f23bd7043591220819e9579e308)) - Merge branch 'master' of https://github.com//o2sh/onefetch ([`a7f25a9`](https://github.com/o2sh/onefetch/commit/a7f25a9385163d73213de12173e89912f384af56)) - Better descriptions for flags and options ([`5cfdb2a`](https://github.com/o2sh/onefetch/commit/5cfdb2aae108c09445a373cfd7147ab3cc7a6888)) - Merge pull request #177 from ebroto/licence ([`65fe707`](https://github.com/o2sh/onefetch/commit/65fe707c713a969a9db0e9f1b5ddac0581f284a2)) - Accept LICENCE... as license file name ([`52dedba`](https://github.com/o2sh/onefetch/commit/52dedba48e246e3fcd20b23b0be0040588c097b7)) - Fix detection of Racket language #174 ([`a64ad56`](https://github.com/o2sh/onefetch/commit/a64ad567dfbbe4d9ef3f304ae02ae0b028cdf11d)) - Add support for DockerFile #173 ([`159b934`](https://github.com/o2sh/onefetch/commit/159b934d9b613f86d09d65c143d7b29a482e591a))
</details>

## v2.2.0 (2020-01-04)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 21 commits contributed to the release over the course of 55 calendar days.
- 55 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Merge pull request #169 from Sh3mm/master ([`fb0ad81`](https://github.com/o2sh/onefetch/commit/fb0ad81a6472845b10e13c8c5d1efcd5a0ec1b2d)) - Merge pull request #168 from Sh3mm/patch-1 ([`640057d`](https://github.com/o2sh/onefetch/commit/640057d3e3b970eac44c771d4425f5dbc475cbea)) - Changed the php logo to a better one ([`a93bb98`](https://github.com/o2sh/onefetch/commit/a93bb98e71ad7435b5a883ace5ab50c576654825)) - Addition of an authors-number parameter ([`cae296c`](https://github.com/o2sh/onefetch/commit/cae296c8ee0f488e2dfb5e2b3af2658fbcf89490)) - Merge pull request #167 from Sh3mm/patch-1 ([`a275398`](https://github.com/o2sh/onefetch/commit/a2753986e9ab84a834fe65a06dad4b2e4f5e7d97)) - Changed the color to real ones ([`cc6d005`](https://github.com/o2sh/onefetch/commit/cc6d00533fe6da5ec940fa424afad674d965bc8f)) - Trim leading spaces in Pending info_name ([`753885c`](https://github.com/o2sh/onefetch/commit/753885c3cfc681d194e09d5c1f3526f925eed7a2)) - Update language.rs ([`b6f3f7f`](https://github.com/o2sh/onefetch/commit/b6f3f7f7074c440b6492895a01e435cb7ab0cc40)) - Merge pull request #166 from axdiamond/show-pending-changes ([`28cab16`](https://github.com/o2sh/onefetch/commit/28cab1649df4f6476fd5c0808b264bfa83180899)) - Dont show zeros ([`4a3c136`](https://github.com/o2sh/onefetch/commit/4a3c136f3d80e83255d2c1455e7c9a05d58a7ced)) - Add another missing case for renamed files ([`876181e`](https://github.com/o2sh/onefetch/commit/876181e665680d3af0da178085c97647d40524d2)) - Add missing match ([`6c74e8f`](https://github.com/o2sh/onefetch/commit/6c74e8fa86eca44b85a9798902c89972225a3426)) - Show file level changes ([`ffabfdb`](https://github.com/o2sh/onefetch/commit/ffabfdb33f6ee7bef7fa7c1c09a21f18a6fef346)) - Move pending under Head in output ([`887b65d`](https://github.com/o2sh/onefetch/commit/887b65da959226748075e34431a9e04226324ff6)) - Change 'changes' to pending ([`0cd4e35`](https://github.com/o2sh/onefetch/commit/0cd4e3518d415d8d534fb9923e7ad0f869d96685)) - Add changes line ([`30b5d01`](https://github.com/o2sh/onefetch/commit/30b5d01e080d79e8bb0c5e03aaa6cb27e6d06638)) - Add support for Groovy #163 ([`90bdc4e`](https://github.com/o2sh/onefetch/commit/90bdc4ecb48d71e23871e4266a96e5d381c7d226)) - Update color profile for cpp ([`869d4f1`](https://github.com/o2sh/onefetch/commit/869d4f1830351d5a75823929e2ee1d5e6de8ae5a)) - Merge pull request #158 from Phundrak/master ([`7ed1c6a`](https://github.com/o2sh/onefetch/commit/7ed1c6a545900d936b58bb3f3b57844209b82a51)) - Removed strum line that shouldn't have been there ([`36fcc86`](https://github.com/o2sh/onefetch/commit/36fcc8624ce8789f1c54e583e34deabe65b6a8cb)) - Merge branch 'master' of github.com:o2sh/onefetch ([`2aef81f`](https://github.com/o2sh/onefetch/commit/2aef81f83caa95a88c40bb8df09c4dc1261ad283))
</details>

## v2.1.0 (2019-11-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 19 commits contributed to the release over the course of 1 calendar day.
- 3 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Added support for Nix ([`4475bf4`](https://github.com/o2sh/onefetch/commit/4475bf4a935f352d0a6c90f80457a6f9cb9f1074)) - Added CMake support ([`7c58540`](https://github.com/o2sh/onefetch/commit/7c58540514e8f5083b1229185da551928ec4f886)) - Added support for the fish shell language ([`029cc0c`](https://github.com/o2sh/onefetch/commit/029cc0cab14f16c3b4360dbdc9712f731655515b)) - Fixed issue with option `-a emacslisp`, moved elisp.ascii to emacslisp.ascii ([`0639324`](https://github.com/o2sh/onefetch/commit/06393244c9bcbb416da93a6b2afd9d7b68500989)) - Merge pull request #157 from ebroto/update-to-askalono-0.4.0 ([`a24c22b`](https://github.com/o2sh/onefetch/commit/a24c22b63032ed56d8d1fc0612b359a335e5079a)) - [strum(serialize = org-mode)] for Org ([`e451104`](https://github.com/o2sh/onefetch/commit/e45110411b3407c1de714c62949e3bd863ea8856)) - Fixed display name for Org ([`096ced1`](https://github.com/o2sh/onefetch/commit/096ced1cabc23e3cb559c48b501c94f25577e2f5)) - Added Emacs Lisp support ([`6ac0b5f`](https://github.com/o2sh/onefetch/commit/6ac0b5fb18999881a6351d3268cd4e358d5839af)) - Merge pull request #155 from Phundrak/master ([`8e4369a`](https://github.com/o2sh/onefetch/commit/8e4369a053197f39c1f005bd6271cc5a9ea766c5)) - Update to askalono 0.4.0 and use a more strict confidence threshold ([`6ae318c`](https://github.com/o2sh/onefetch/commit/6ae318c8222ce03a71ab037e6a56354ec0002df0)) - Merge pull request #154 from CephalonRho/sixel-backend ([`6b145a5`](https://github.com/o2sh/onefetch/commit/6b145a5a2aeef585ba583a5b55bd1a9a0f30ca94)) - Added org-mode support ([`c7d82e4`](https://github.com/o2sh/onefetch/commit/c7d82e422e1cfdc8fa1d6361ca19304995a39751)) - Fix sixel support detection ([`745982d`](https://github.com/o2sh/onefetch/commit/745982dbc19924398177836ec5c324e03d0cb80f)) - Fix windows build ([`1194f71`](https://github.com/o2sh/onefetch/commit/1194f71a867b665bca3ac6cb46c79a75cf823073)) - Add --no-color-blocks flag #153 ([`f74f741`](https://github.com/o2sh/onefetch/commit/f74f7410a8661ce675cb47d8d459c5d56d55dcc7)) - Add image-backend argument ([`c5a1e2c`](https://github.com/o2sh/onefetch/commit/c5a1e2c97013f7ab70f85a410061c099667b8cec)) - Fix color introducer string ([`cb8225f`](https://github.com/o2sh/onefetch/commit/cb8225f63db6f88169c47e9601226445474bcaf7)) - Add sixel backend ([`dc4e360`](https://github.com/o2sh/onefetch/commit/dc4e360256c330e2cac6e7a8fa66e9f0a131c509)) - Fix the kitty backend reading more bytes than it should ([`a64db30`](https://github.com/o2sh/onefetch/commit/a64db30f1b8fbc35371a46a107af4d35ab21e30c))
</details>

## v2.0.1 (2019-11-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Merge pull request #152 from ebroto/bugfix/isc-license-not-recognised ([`575b3ed`](https://github.com/o2sh/onefetch/commit/575b3ed1f01ad31e9b8dbcb8170b37ad390d9710)) - Use a newer version of askalono so ISC is detected ([`4dfc4e5`](https://github.com/o2sh/onefetch/commit/4dfc4e55d70b0b734694e8ced760ccb37b9fe752))
</details>

## v2.0.0 (2019-11-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 47 commits contributed to the release over the course of 6 calendar days.
- 7 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Limit language stat to one decimal ([`8b710bd`](https://github.com/o2sh/onefetch/commit/8b710bd558cf3aa2a789021a2ecf9147f512eea2)) - Merge pull request #150 from KasraF/ocaml_support ([`e44d736`](https://github.com/o2sh/onefetch/commit/e44d736b76b76e48fe586a4af36b50eb32d2bd7c)) - Merge branch 'master' into ocaml_support ([`3ba0020`](https://github.com/o2sh/onefetch/commit/3ba00208a419f8ffe60c03ae77e4bef2de8e1049)) - Add [strum(serialize = fortran)] for FortranModern ([`a133771`](https://github.com/o2sh/onefetch/commit/a133771d5c16754939f7be794e0aa428a8f5feb0)) - Merge pull request #149 from ebroto/feature/license-with-askalono ([`a2f6352`](https://github.com/o2sh/onefetch/commit/a2f63528648e02c303a5364b8e54f5c0036958d4)) - Merge pull request #142 from ZapAnton/fix_language_name_mismatch ([`ce4756e`](https://github.com/o2sh/onefetch/commit/ce4756e462d6fd0b07c091845209b72adf97b270)) - Removed the default value for the 'ascii_language' argument ([`e419f93`](https://github.com/o2sh/onefetch/commit/e419f93a655352a81d326de9dd498c84b958b9eb)) - Fixed whitespace ([`7909622`](https://github.com/o2sh/onefetch/commit/790962244fb96584f8421a8d5341dfb3f8fedaa1)) - Added support for the OCaml language ([`77febb2`](https://github.com/o2sh/onefetch/commit/77febb2c4eb623250f5d4de439a4beb295a454c9)) - Made the '-a' flag accept language names with special characters ([`13928fa`](https://github.com/o2sh/onefetch/commit/13928fa86f43eabdcf0a703369f8c32f9d8539ff)) - Generate license cache at build time ([`a1243fa`](https://github.com/o2sh/onefetch/commit/a1243fa038d7aad501a8ff4049e6192849751b8d)) - Rework license module to avoid loading cache for each license ([`dd1e480`](https://github.com/o2sh/onefetch/commit/dd1e480d643272675cc59f401f71543daabe93b5)) - Replace license crate with askalono ([`4d35e5a`](https://github.com/o2sh/onefetch/commit/4d35e5a965696f9255f5917b1674728df77c45bd)) - Missing ref #148 ([`493a07f`](https://github.com/o2sh/onefetch/commit/493a07fc55ca5bdf5946c64646d1ae08afaed1ad)) - Add support for markdown #148 ([`a698aaa`](https://github.com/o2sh/onefetch/commit/a698aaa24646582a5a081e2d3ab2461962e120ef)) - Update fortran asset ([`1bbc83f`](https://github.com/o2sh/onefetch/commit/1bbc83f7b4040be56e6fa625b5889ca516031590)) - Add support for Fortran 90 #138 ([`01fd813`](https://github.com/o2sh/onefetch/commit/01fd813edf2c88c5b35162d027412739a549ea38)) - Moved the possible language values from the 'help' method to the 'possible_values' method ([`46e70b3`](https://github.com/o2sh/onefetch/commit/46e70b33a3494e661a2f52d678977adc92ef485d)) - Merge pull request #139 from rockisch/master ([`9382549`](https://github.com/o2sh/onefetch/commit/93825493fbc17b9b4e4a56ae32215cfe7f84ec62)) - Improve functions ordering ([`f5adb95`](https://github.com/o2sh/onefetch/commit/f5adb958dc317dde363758f98f19d66577f7222f)) - Improve running command from subfolder ([`790a85c`](https://github.com/o2sh/onefetch/commit/790a85c2b1ca5f16de04c3598e8e51fdb237b231)) - Add support for D #145 ([`7ce913e`](https://github.com/o2sh/onefetch/commit/7ce913e3d68e62ce028e9d31b6024e9ffade5c4b)) - Added the possible language list to the '--ascii_language' flag help message ([`a9e6184`](https://github.com/o2sh/onefetch/commit/a9e61843f7ede08ee81faae8c8b0e59f1e365bcf)) - Changed the string representation of the languages , that contain special symbols ([`c110481`](https://github.com/o2sh/onefetch/commit/c11048131c3fc2dc46a4cd560fbe3df76e6760ad)) - Merge pull request #141 from o2sh/feature/no-merges ([`654a726`](https://github.com/o2sh/onefetch/commit/654a726a5d8ba05e26c7a6a28a6eeb5fdd45dfaa)) - Add -n flag for no-merges ([`f09d556`](https://github.com/o2sh/onefetch/commit/f09d5569a8da4d813e9fb58ff17cdd7e73fd8293)) - Merge remote-tracking branch 'origin/master' into feature/no-merges ([`f6df4dc`](https://github.com/o2sh/onefetch/commit/f6df4dc67c977564f2c45ce8d345070d09759dee)) - Merge pull request #140 from ZapAnton/fix_clippy_warnings ([`1e10bd2`](https://github.com/o2sh/onefetch/commit/1e10bd2648734fdfc4d0718fda52ee945b8a8703)) - Prevent merge author count on --no-merges ([`5672098`](https://github.com/o2sh/onefetch/commit/5672098e4a19e70d8cfe448aea7600065d0c978f)) - Add --no-merges flag for total commit count ([`efe9d70`](https://github.com/o2sh/onefetch/commit/efe9d704c65196801a343f8afe280fd5888b9de2)) - Applied 'cargo fmt' ([`615eecd`](https://github.com/o2sh/onefetch/commit/615eecdc91220b5c55784a93fc134cc96455689e)) - Fixed the 'needless_collect' clippy warning ([`0ea794b`](https://github.com/o2sh/onefetch/commit/0ea794b5c7f3da1b9015952b5d3794ddde7d3d8c)) - Fixed the 'clone_on_copy' clippy warning ([`cc59305`](https://github.com/o2sh/onefetch/commit/cc593054154d131b8973f1ed603e1682160e3c6f)) - Fixed the redundant_pattern_matching clippy warning ([`8e85b53`](https://github.com/o2sh/onefetch/commit/8e85b53503507c9e0ca8c3cba89d42875cff1805)) - Fixed the 'len_zero' clippy warning ([`b214dc2`](https://github.com/o2sh/onefetch/commit/b214dc2cb31390abf6397b316ee37a2c3d4361ab)) - Fixed the 'identity_conversion' clippy warning ([`090493e`](https://github.com/o2sh/onefetch/commit/090493e54a266f839892e1df9def6e12ec2ca97e)) - Fixed the needless_lifetimes clippy warning ([`95b5b75`](https://github.com/o2sh/onefetch/commit/95b5b755437ebc7ba42b1b648d69790736c760cb)) - Fixed the 'single_char_pattern' clippy warning ([`ddb100c`](https://github.com/o2sh/onefetch/commit/ddb100c4587545ce400b49bb48dcf7922b09c63c)) - Fixed the 'ptr_arg' clippy warning ([`065d65a`](https://github.com/o2sh/onefetch/commit/065d65a2191c987dcfb4aba8d5008eae7e2324b6)) - Fixed the 'write_literal' clippy warning ([`d868275`](https://github.com/o2sh/onefetch/commit/d8682752aca78bc5d596916f9d464e9835225ebb)) - Fixed the 'op_ref' clippy warning ([`cf815c0`](https://github.com/o2sh/onefetch/commit/cf815c029b1e8c3fdb7b9939e7eb450f290b34a6)) - Fixed the 'cast_lossless' clippy warning ([`3582a88`](https://github.com/o2sh/onefetch/commit/3582a88210c986b332b960eb4291bf886ab30f85)) - Fixed the 'char_lit_as_u8' clippy warning ([`54d9951`](https://github.com/o2sh/onefetch/commit/54d99513c7f78487137cde35df0e81e708a77ece)) - Fixed the 'redundant field names in struct initialization' clippy warning. ([`cb97ca1`](https://github.com/o2sh/onefetch/commit/cb97ca13f5748a347e7cc128eb7683b318d1e84d)) - Add support for Julia #136 ([`0815e8a`](https://github.com/o2sh/onefetch/commit/0815e8a138f6ed1bc69c0572a06f17da880f1799)) - Merge pull request #132 from rockisch/master ([`110811a`](https://github.com/o2sh/onefetch/commit/110811aaaa0caeeaa65af12c3060403dc715ca80)) - Allow command to run from subfolder ([`2dd7ff3`](https://github.com/o2sh/onefetch/commit/2dd7ff30ec0187d9e6fcb9fd582ee4081d40ccf0))
</details>

## v1.7.0 (2019-10-29)

<csr-id-f0285a06a1779fc1871345d3d1d3fad090ca004c/>

### Documentation

- <csr-id-73bdadb1488e73a1e2360b01297a86780e9aa63a/> update

### Other

- <csr-id-f0285a06a1779fc1871345d3d1d3fad090ca004c/> fixing missing comma

### Commit Statistics

<csr-read-only-do-not-edit/>

- 105 commits contributed to the release over the course of 17 calendar days.
- 19 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Merge pull request #127 from portgasd666/master ([`06d58db`](https://github.com/o2sh/onefetch/commit/06d58db7d9fa6e0575d110381c95fcf22e9e7335)) - Trim _all_ trailing blanklines ([`3bfe441`](https://github.com/o2sh/onefetch/commit/3bfe441925caec730f84b40a39b130b3117b13fc)) - Bump license from 0.7.1 to 0.8.1 Use new from_text_ext API instead of the Kind Enum ([`fd5f13c`](https://github.com/o2sh/onefetch/commit/fd5f13c97ab8e3be55df50ad3b060124afaafc2f)) - Target_os instead of targt ([`9769425`](https://github.com/o2sh/onefetch/commit/97694253d550a4fc0cb3aa5e123f244f3cfa6f36)) - Target_os instead of target, fix #86 ([`941a830`](https://github.com/o2sh/onefetch/commit/941a830661effd6d73b8ecda48ef844fc2a13c33)) - Set color override to true if enabled ([`d548783`](https://github.com/o2sh/onefetch/commit/d5487830b5cd646116fb824f92db8bb3bd60d5ac)) - Merge pull request #124 from astynax/master ([`99ff814`](https://github.com/o2sh/onefetch/commit/99ff814585e41168a5eb16f9011513c78a0e1891)) - Add Racket logo ([`b2d528c`](https://github.com/o2sh/onefetch/commit/b2d528c4c0ad2d8403b84f8a877c79b1ab6c03be)) - Merge pull request #113 from CephalonRho/display-image ([`d66e76f`](https://github.com/o2sh/onefetch/commit/d66e76f875eae6c7981d4f6626d321d8dd064dda)) - Merge pull request #114 from CephalonRho/limit-lang ([`08d9130`](https://github.com/o2sh/onefetch/commit/08d913062549f2c7d69d76a4bd457d48e7ef5389)) - Remove unused mut ([`9f080e4`](https://github.com/o2sh/onefetch/commit/9f080e4fb8b20da4130e901c4c39875c8822878c)) - Always show other language stat last ([`3dabf7b`](https://github.com/o2sh/onefetch/commit/3dabf7b150bcd72c0adabec7a768ea916bac4456)) - Avoid unnecessary allocation ([`710f1eb`](https://github.com/o2sh/onefetch/commit/710f1ebd15edacbaffc8aa81264934bd1632b478)) - Limit shown languages to 6 ([`8f07135`](https://github.com/o2sh/onefetch/commit/8f07135299545cb233456e93c9a22172e6a77bb7)) - Fix missing function for non-Linux targets ([`c8ed05f`](https://github.com/o2sh/onefetch/commit/c8ed05f8334ee3983dde6c41dd999887e228740f)) - Fix center pad sometimes being added multiple times ([`7d25da0`](https://github.com/o2sh/onefetch/commit/7d25da049fece279b855bb2c7420200abeb0c549)) - Add support for displaying a custom image instead of ascii art ([`dad9449`](https://github.com/o2sh/onefetch/commit/dad94491984d31e0bfa2235d82eb8d10424c69b6)) - Fixed regression - order languages by loc ([`f5296f2`](https://github.com/o2sh/onefetch/commit/f5296f2bd338c9060017438bef5ca28b7823142a)) - Merge pull request #108 from Emanon42/master ([`c527767`](https://github.com/o2sh/onefetch/commit/c5277677d67304c2759fc8c07dbd7ee872696853)) - Update language.rs ([`4ed8a3e`](https://github.com/o2sh/onefetch/commit/4ed8a3e5fe13c1f5634cb6dd613cd8cf00ad117e)) - Merge branch 'master' of https://github.com/o2sh/onefetch ([`768e86f`](https://github.com/o2sh/onefetch/commit/768e86feca3da665d4fb24a4a4485ad23037bf21)) - Slight refacto of #107 and change separator ([`c0d1689`](https://github.com/o2sh/onefetch/commit/c0d168992405af6c04aae8902e5949e230880293)) - Merge pull request #94 from GooseDB/prolog ([`4121af5`](https://github.com/o2sh/onefetch/commit/4121af5f6eee78335991564bd4ba2fe1ee30d47a)) - Colors ([`8331d04`](https://github.com/o2sh/onefetch/commit/8331d04c6d9d0cf770662581c17dc4f0d43fedc6)) - Add new lang in code ([`e95582a`](https://github.com/o2sh/onefetch/commit/e95582a3f13fdbaba2f76c887fdf1fe1452e70c6)) - Merge branch 'master' of https://github.com/o2sh/onefetch into prolog ([`46a2cad`](https://github.com/o2sh/onefetch/commit/46a2cad7962f45fd9df5405384dae1c0181c8302)) - Merge pull request #107 from pablodiegoss/master ([`908c866`](https://github.com/o2sh/onefetch/commit/908c86668103a112e26761764cb5102a26ee2b97)) - Fix separator color ([`ae95e95`](https://github.com/o2sh/onefetch/commit/ae95e9552492b67d172c0a12d50c6c57e70a698b)) - Update usage of write_buf and colors ([`cc6a9c8`](https://github.com/o2sh/onefetch/commit/cc6a9c8a9e114c3665f02f302bf8c38392f4443c)) - Merge remote-tracking branch 'o2sh/master' ([`419a32b`](https://github.com/o2sh/onefetch/commit/419a32bab15769475e02785b130721e6a7240816)) - Removing redundancies in string usage ([`2b5ddf4`](https://github.com/o2sh/onefetch/commit/2b5ddf4e4d7f0d945ef61dc30189b009f5ddaee1)) - Update language.rs ([`c763738`](https://github.com/o2sh/onefetch/commit/c763738e4e6d388bf1433609c1a9ef19d034de34)) - Merge pull request #99 from ccmetz/bold-flag ([`793bc43`](https://github.com/o2sh/onefetch/commit/793bc431493b89fed7618072c89fe41f8c135c5c)) - Ignore username when empty or unset ([`daa59b4`](https://github.com/o2sh/onefetch/commit/daa59b460ad217c0201e061127c98ad12ebf93f7)) - Format separator based on git info length ([`1fb086f`](https://github.com/o2sh/onefetch/commit/1fb086fcbd0823134ce920338b6f7f1c7dd8e335)) - Fix directory access on get git info ([`50b8763`](https://github.com/o2sh/onefetch/commit/50b8763b8044ca5745039890d2ebdcb03005989e)) - Add git info for user and version ([`e9f4f8d`](https://github.com/o2sh/onefetch/commit/e9f4f8d5d71475b493feaa3d9f2fc7d798046881)) - Minor refactoring ([`bcce1ca`](https://github.com/o2sh/onefetch/commit/bcce1cae2175fdb65e9579fb1a185cf4fbe2a7d8)) - Change bold option to a flag ([`0b97ed6`](https://github.com/o2sh/onefetch/commit/0b97ed6a6ed75223dc8afd13031e8bd2c944b390)) - Merge branch 'master' into bold-flag ([`f3f90d6`](https://github.com/o2sh/onefetch/commit/f3f90d60293b595bc1653858c379287fcaf8d0a6)) - Small changes in #103 ([`d74351d`](https://github.com/o2sh/onefetch/commit/d74351d8f042db42a68a51f5f8f05570bbfc4655)) - Merge pull request #103 from vypxl/master ([`1164959`](https://github.com/o2sh/onefetch/commit/116495923b312ed340ee170a7ef653ac827afd58)) - Change flag name from list to supported ([`ece4a65`](https://github.com/o2sh/onefetch/commit/ece4a658a895bc36abaaae73b011d006951ebd7b)) - Format ([`0f3b82e`](https://github.com/o2sh/onefetch/commit/0f3b82e33cf169a53bacd3eedcdb576de36061a0)) - Add --list, -l flag to list supported languages ([`2e0ac38`](https://github.com/o2sh/onefetch/commit/2e0ac38bcfc03c855cfc469117a8f2e5ad0b740b)) - Fix Created field ([`91b554c`](https://github.com/o2sh/onefetch/commit/91b554c918e485bf4ad56ea75794fa0fb64717af)) - Merge pull request #101 from andymac-2/center-padding ([`98eba71`](https://github.com/o2sh/onefetch/commit/98eba71d90864c1e634b093fb208529783556c14)) - Included center pad and double newline at end. ([`faeda05`](https://github.com/o2sh/onefetch/commit/faeda05c5f0ac31f3d6e2ae9dafc5da4c3232c65)) - Merge pull request #98 from andymac-2/refactor-modules ([`ae693b3`](https://github.com/o2sh/onefetch/commit/ae693b3c9f60fceb99ae9c1419b65b798ba86b66)) - Updated tests for tokenizer. ([`c790d0c`](https://github.com/o2sh/onefetch/commit/c790d0caae34e009c2fafa4d8885caa2a408bafb)) - Merge branch 'master' into refactor-modules ([`b5682bf`](https://github.com/o2sh/onefetch/commit/b5682bffddcef43f7331f5d9e7bd2e42b6637f37)) - Add bold flag to tests, add tests for disabled bold characters ([`9f14637`](https://github.com/o2sh/onefetch/commit/9f1463790dbac38360c3181bf709b8a44ba44cef)) - Merge branch 'master' into bold-flag ([`37e62c5`](https://github.com/o2sh/onefetch/commit/37e62c5fc900db5c2c175f96fac50c0440f15cba)) - Add bold parameter to AsciiArt, adjust boldness of logo based on flag ([`539b928`](https://github.com/o2sh/onefetch/commit/539b928fa0feb4bb98b06d2c8fbfbd9aa7233b4a)) - Merge branch 'master' into bold-flag ([`924a4ed`](https://github.com/o2sh/onefetch/commit/924a4ede12c307af72b6ab417e21b402d900d42b)) - Fixing build by adapting test to take into account boldness #96 ([`e9d3111`](https://github.com/o2sh/onefetch/commit/e9d31114060ea42e6189d2078e5f708546cfad3e)) - Refactor formatted label function to be a method of Info ([`ded5f48`](https://github.com/o2sh/onefetch/commit/ded5f48656284b9f546649ae28b57ee9302797b7)) - Adjust boldness of logo based on bold flag ([`dccea3d`](https://github.com/o2sh/onefetch/commit/dccea3dcd2092dabc1f289b9e2e9d8698576f028)) - Removed useless tests ([`0efd3c5`](https://github.com/o2sh/onefetch/commit/0efd3c53166c60096317a48bea2541e752379165)) - Split code into modules ([`8d1a3c1`](https://github.com/o2sh/onefetch/commit/8d1a3c17054bb3d69c33ee367901a861fd241bae)) - Add two line breaks when (none, none) ([`405937b`](https://github.com/o2sh/onefetch/commit/405937b4b0ca458d2d29bb18b5f6376935484fb9)) - Increase central pad and added missing colors in asccii art ([`cdf4f7a`](https://github.com/o2sh/onefetch/commit/cdf4f7a36f2ab4fb3fe2f143c6e13d444310f7e7)) - Merge pull request #96 from andymac-2/fix-art-widths ([`ae5f8bf`](https://github.com/o2sh/onefetch/commit/ae5f8bfaada6c2a4baf7b45227e8ae21c4165f24)) - Embolden logo ([`843b122`](https://github.com/o2sh/onefetch/commit/843b1225e7d145112a2838f58331efbcfc0b0e90)) - Remove lines from the top and bottom of the logo and info ([`da28b62`](https://github.com/o2sh/onefetch/commit/da28b628716d819c586c1a6b6b6c42d0d18f7b89)) - Renamed module ([`ff7814a`](https://github.com/o2sh/onefetch/commit/ff7814ad4bf5d9a0ee21426aa966a5fdaca4813a)) - Fixing ascii art widths. ([`f8dff54`](https://github.com/o2sh/onefetch/commit/f8dff546bda8562b05300d019fffac477334e448)) - Change boldness of info labels based on command line flag ([`397343b`](https://github.com/o2sh/onefetch/commit/397343b56eefbd73262ef43731e9be4cc28c07e1)) - Bold logo by default ([`962dc42`](https://github.com/o2sh/onefetch/commit/962dc42af7b1874db70b2d44482398210e82a57b)) - Prolog ([`ffe4e20`](https://github.com/o2sh/onefetch/commit/ffe4e20af0e78aa8dd454e3c0b1d056f283b8652)) - Inverse colors for tex ([`58a15c9`](https://github.com/o2sh/onefetch/commit/58a15c92f3a332026c314c0a85e53e396e8e0948)) - Merge pull request #92 from KaindlJulian/tex-support ([`55e12a5`](https://github.com/o2sh/onefetch/commit/55e12a5108db48ff655662d318d7e1f7bd5eb83b)) - Add TeX Support ([`d46df84`](https://github.com/o2sh/onefetch/commit/d46df84b60efeac4abe2faf099310c654b35faf0)) - Merge pull request #88 from spenserblack/master ([`f23aa63`](https://github.com/o2sh/onefetch/commit/f23aa630679c540bd09a35f8553c78f9bef0f1c9)) - Fix compilation error on Linux ([`0264b7c`](https://github.com/o2sh/onefetch/commit/0264b7c44f16b97399166195259889a3949ce565)) - Win10 color fix #86 ([`ad64f9c`](https://github.com/o2sh/onefetch/commit/ad64f9cde2f676b1378c10a0763f128cb06cae4b)) - Handle multiple prefixes for license detection -- COPYING ([`0fd4d43`](https://github.com/o2sh/onefetch/commit/0fd4d43ad853471d9b09f8e9ddc212ce33e1d383)) - Merge pull request #85 from amiralies/add-elixir-2 ([`b7a1761`](https://github.com/o2sh/onefetch/commit/b7a1761ddf05685d31c2aa67a22bf9b21e0ae654)) - Add elixir ([`90c7a7d`](https://github.com/o2sh/onefetch/commit/90c7a7d7106a589e86a39164089cd07422b54803)) - Improved xml art ([`7cd6fe2`](https://github.com/o2sh/onefetch/commit/7cd6fe2d99168d7de23fda1b39b32c8fb025f5b2)) - Merge pull request #84 from tacrazymage/master ([`0c905c8`](https://github.com/o2sh/onefetch/commit/0c905c829925c98be1ae82b23c951bfe6c92d385)) - Correct second color of xml ([`03cc3de`](https://github.com/o2sh/onefetch/commit/03cc3de179c88a4d5bcbc83b0552a66c0d5842c9)) - Split color marker in two lines ([`339aa73`](https://github.com/o2sh/onefetch/commit/339aa7326ff56de2e95c1426f8e3e3a64049fa13)) - Added xml ascii art ([`e7e25ee`](https://github.com/o2sh/onefetch/commit/e7e25ee72eb9d123b9e6d0b8670e57d8ecce691b)) - Merge pull request #82 from ktsuench/master ([`4d56dbb`](https://github.com/o2sh/onefetch/commit/4d56dbb538454a711981e51761e7d2fcffcf28b6)) - Updated --disable flag behaviour by removing use of vec.contains and using bool struct instead ([`149c3e4`](https://github.com/o2sh/onefetch/commit/149c3e4d6e58c555fde3cf358be572f9e453f8f7)) - Updated behaviour of --disable flag to not error out on unrecognized values ([`450a651`](https://github.com/o2sh/onefetch/commit/450a6511946097cb744418cc4597311e43c62e48)) - Revert "changed behaviour of --disable flag" ([`1dc797c`](https://github.com/o2sh/onefetch/commit/1dc797ce4b524b775bf5f6e410f6f0c91b0356e7)) - Changed behaviour of --disable flag ([`006d7e8`](https://github.com/o2sh/onefetch/commit/006d7e8d4fa1391c5948d7cb5326841b057efcff)) - Added feature to disable info fields from showing in output ([`fb62e9d`](https://github.com/o2sh/onefetch/commit/fb62e9d306716810a22e856ec0da7b95b38e3f25)) - Merge changes from fork parent ([`40ae2bd`](https://github.com/o2sh/onefetch/commit/40ae2bd7091c87223e08ab31df8eb2182b724300)) - Merge pull request #79 from spenserblack/select-colors ([`67616ea`](https://github.com/o2sh/onefetch/commit/67616ea4e998c67f425b705cb62131a07881d560)) - Add visual marker ([`bbda405`](https://github.com/o2sh/onefetch/commit/bbda405ffc3bc72add98c8b6df817bf870bfb082)) - Merge pull request #78 from astynax/master ([`6fdcae3`](https://github.com/o2sh/onefetch/commit/6fdcae3fb6b28e2d8b4fcf35452266e11e4f67f4)) - Allow custom colors via CLI ([`2c08598`](https://github.com/o2sh/onefetch/commit/2c085983d67c5dacf008b3830dfe706317480595)) - Add logo for Elm language ([`3dd22b4`](https://github.com/o2sh/onefetch/commit/3dd22b475e40d0d0c39c717c4589a01774bfa920)) - Merge pull request #75 from ktsuench/master ([`79164fd`](https://github.com/o2sh/onefetch/commit/79164fd580d6f2d5f5d85ec49bda4fcc19f3a09c)) - Added in --ascii_language option ([`bc7972e`](https://github.com/o2sh/onefetch/commit/bc7972e5bc6109bb6640a94aa2b316d4c0a793f8)) - Merge pull request #72 from ktsuench/master ([`983424d`](https://github.com/o2sh/onefetch/commit/983424d80dc3d1e67aa49635b28502e35234fc81)) - Merge pull request #71 from estevam31/master ([`a2b1d43`](https://github.com/o2sh/onefetch/commit/a2b1d43152a9df8041ae09e6f93f578d32efd8f6)) - Fixing missing comma ([`f0285a0`](https://github.com/o2sh/onefetch/commit/f0285a06a1779fc1871345d3d1d3fad090ca004c)) - Merge pull request #68 from MaxJohansen/html-css ([`7e4fd0f`](https://github.com/o2sh/onefetch/commit/7e4fd0f679cc176bfcd215e4969dc40e3658ab93)) - Add HTML/CSS language support ([`22ed5a2`](https://github.com/o2sh/onefetch/commit/22ed5a28ad6c5b782e08f6836eeeb7bd7c88375f)) - Added in coffeescript ([`b90f95b`](https://github.com/o2sh/onefetch/commit/b90f95be21655b21d7eaf56c40d7e7d406233f7a)) - Add Vue.js language support ([`f1538c1`](https://github.com/o2sh/onefetch/commit/f1538c105cfda50bbda995965c0f23614a665d49))
</details>

## v1.6.5 (2019-10-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 15 commits contributed to the release over the course of 4 calendar days.
- 5 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Move creation date to its own line ([`d9bfb73`](https://github.com/o2sh/onefetch/commit/d9bfb73787257f841c69213432264542898527bc)) - Merge pull request #61 from bojan88/project_creation_date ([`7b31ed5`](https://github.com/o2sh/onefetch/commit/7b31ed5807f1dff66d63ea7fa30460154cb2dda0)) - Merge pull request #66 from GooseDB/purescript ([`205bf18`](https://github.com/o2sh/onefetch/commit/205bf18710a6e7ad33a1dd64e7c8d2416b2ac971)) - Tcl && purescript ([`379d0c0`](https://github.com/o2sh/onefetch/commit/379d0c05a573675a490458c62397c68808a9209b)) - Merge pull request #62 from SamTebbs33/master ([`2c7aa5b`](https://github.com/o2sh/onefetch/commit/2c7aa5ba367dd07525463d1998f22cfe53e2fc93)) - Add Zig support ([`ef9af43`](https://github.com/o2sh/onefetch/commit/ef9af43b899e9baa272c64ea7d8b73a73b374a16)) - Merge pull request #64 from jadijadi/erlang_logo ([`b4b0a98`](https://github.com/o2sh/onefetch/commit/b4b0a9870da683e3804407fe9ce1997b8f49e2c6)) - Erlang ascii art is added ([`9311548`](https://github.com/o2sh/onefetch/commit/93115483eadf59d02b28afe2be01cb83ef0aa1e7)) - Added project creation date ([`8f22ff5`](https://github.com/o2sh/onefetch/commit/8f22ff5362c3a85ff842cd03e2e8f57858bda66c)) - Rollback last change ([`982a65e`](https://github.com/o2sh/onefetch/commit/982a65e79b35fce641ccb78ec0c96eda69346bcb)) - Merge pull request #58 from Vipul-Bajaj/master ([`a1aff05`](https://github.com/o2sh/onefetch/commit/a1aff056bcdf0accf71ff3b365a2119bf2dd25f8)) - Add contributors ([`7773f93`](https://github.com/o2sh/onefetch/commit/7773f9319a5add772f2899efc46e83c813ed5f85)) - Merge pull request #57 from WillyChen123/authors-info ([`05d3788`](https://github.com/o2sh/onefetch/commit/05d3788736629ee0392ff3b25dca5fe62fc1be68)) - Add number of files to Repository size ([`3ee77f5`](https://github.com/o2sh/onefetch/commit/3ee77f5b3435a28722e706df3e8fabe2f29791eb)) - Additional info about authors ([`cca43a7`](https://github.com/o2sh/onefetch/commit/cca43a701e2ed513722b55d8e26ee36190c6b6b3))
</details>

## v1.6.0 (2019-10-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 18 commits contributed to the release over the course of 3 calendar days.
- 83 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Missing reference to Idris ([`6dd7da5`](https://github.com/o2sh/onefetch/commit/6dd7da54ed69f33398a71f61c3a45696cb4b123a)) - Merge pull request #54 from Emanon42/master ([`197c56d`](https://github.com/o2sh/onefetch/commit/197c56d5ecd2727a12077bad1f051ce257514362)) - Update main.rs ([`032d015`](https://github.com/o2sh/onefetch/commit/032d015eb53c19c7e55942064b99a42995ce91c8)) - Merge pull request #53 from arvidboivie/feature/short-commit-hash ([`71f245a`](https://github.com/o2sh/onefetch/commit/71f245a44a27fa25f04ab840ec3cf924401858cf)) - Shorten commit hash to 7 characters ([`c51b68d`](https://github.com/o2sh/onefetch/commit/c51b68d508a7e4d14dae6dbdb601c595635132e8)) - Merge pull request #49 from hoop33/master ([`6474be0`](https://github.com/o2sh/onefetch/commit/6474be0f8135b4231dd3db7bae4da4d01ffc6e44)) - Add support for Objective-C ([`6df25cd`](https://github.com/o2sh/onefetch/commit/6df25cd4e9ed2beec31a412de4d177f25e7dbb0f)) - Merge pull request #48 from spenserblack/update/tokei/10.0 ([`39b3f05`](https://github.com/o2sh/onefetch/commit/39b3f054101d02cb6d02a2edf01885877c1b5b40)) - Update tokei to v10.0 ([`66b282c`](https://github.com/o2sh/onefetch/commit/66b282cc532d695f6120014e9b29ce56924b3708)) - Merge pull request #47 from spenserblack/lang/kotlin ([`59c81c2`](https://github.com/o2sh/onefetch/commit/59c81c21095da4dbf093f90f0a3456a92a44861e)) - Add colors for Kotlin ([`aea155e`](https://github.com/o2sh/onefetch/commit/aea155e8dcd61df5b95c5d54d2add9c5e60e714b)) - Add basic Kotlin support ([`4d91437`](https://github.com/o2sh/onefetch/commit/4d91437bcfd33170e4c1afc87d1cc41f890079f6)) - Merge pull request #44 from nikofil/master ([`6fbad3b`](https://github.com/o2sh/onefetch/commit/6fbad3b76ac3a8578f326799d0c9ea86d4159b98)) - Display repository size ([`004e687`](https://github.com/o2sh/onefetch/commit/004e6875a281718e61c0f46dd1bec0002c8916d4)) - Display current commit and its references' names ([`bde2d86`](https://github.com/o2sh/onefetch/commit/bde2d866b6f57e8df443660dba97d7780877a9a4)) - Merge pull request #42 from spenserblack/master ([`34bd253`](https://github.com/o2sh/onefetch/commit/34bd253a58056d8b95de96b6393a5e23520dfc1f)) - Fix unused/deprecated warnings ([`cc2d95c`](https://github.com/o2sh/onefetch/commit/cc2d95c4ab44b215373b6007af6b1ef2af5fc3bd)) - Use clap to handle command-line arguments ([`36088b7`](https://github.com/o2sh/onefetch/commit/36088b708d0b444a53d5f982ed0d3d269bd94e97))
</details>

## v1.5.5 (2019-07-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 9 commits contributed to the release over the course of 94 calendar days.
- 97 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Added support for Perl #39 ([`1dd1169`](https://github.com/o2sh/onefetch/commit/1dd11695f3d3d71393d5fa635ffa9bbf8c11dad3)) - Added support for Nim #37 ([`20ddd8b`](https://github.com/o2sh/onefetch/commit/20ddd8b2af6c48ebf2233926c8b78fe3088cc2a4)) - Added support for Dart #38 ([`40a23f9`](https://github.com/o2sh/onefetch/commit/40a23f9b7158e586547256a84c977c8cfc96a11b)) - Merge pull request #35 from jephthai/master ([`827c113`](https://github.com/o2sh/onefetch/commit/827c11334108e9b1bae277a4a4692e9b2bf1c195)) - Add Forth language ([`57eb0d0`](https://github.com/o2sh/onefetch/commit/57eb0d09a4bfa0bfab33136397f6fd48e7250e38)) - Merge pull request #32 from vinhnx/master ([`8194d16`](https://github.com/o2sh/onefetch/commit/8194d161e19ab4b04cb2ea6edf9133c8561b6432)) - Add Swift lanaguage detection support ([`948638b`](https://github.com/o2sh/onefetch/commit/948638b14098694605049a5f592b8b19584cea1e)) - Merge pull request #30 from aeter/master ([`2e8d973`](https://github.com/o2sh/onefetch/commit/2e8d9734383ce2b76592f6af132757075385f452)) - Add assembly detection and ascii image ([`580bead`](https://github.com/o2sh/onefetch/commit/580bead0f06c9ec24239e1d5a2c824606e14b590))
</details>

## v1.5.4 (2019-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 1 commit contributed to the release.
- 29 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Add last change #28 ([`f0b718e`](https://github.com/o2sh/onefetch/commit/f0b718ec260ebaa1347b702fb78dcbfbef85598e))
</details>

## v1.5.3 (2019-03-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 20 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Forgot to update error messages #27 ([`14c96dc`](https://github.com/o2sh/onefetch/commit/14c96dcffb9182d20ed0190332d7fb8030a8b0b1)) - Specify path on the command line #27 ([`aac411c`](https://github.com/o2sh/onefetch/commit/aac411c9579b364092a4e2f0e20da33226266f0b))
</details>

## v1.5.2 (2019-02-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 8 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Clippy #25 ([`3826fd1`](https://github.com/o2sh/onefetch/commit/3826fd1047bf15338f97e82c058008d3a4f930dc)) - Multiple language overflow and max between info and logo in main for loop #25 ([`14282be`](https://github.com/o2sh/onefetch/commit/14282bef79ae0e77d98bb9f627925fc5ca8dfb2e))
</details>

## v1.5.1 (2019-02-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release over the course of 4 calendar days.
- 8 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Clippy review ([`e297b36`](https://github.com/o2sh/onefetch/commit/e297b3690b070192c0152271fb7a2a18d69b6e66)) - Multiple language stats ([`511323a`](https://github.com/o2sh/onefetch/commit/511323a7874e48c4c14592c8334b2011849941fd))
</details>

## v1.5.0 (2019-01-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 4 commits contributed to the release over the course of 43 calendar days.
- 46 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Detect number of commits #24 ([`1778b9d`](https://github.com/o2sh/onefetch/commit/1778b9d892600a9a2642ab79fb4c83a80a883638)) - Detect version #24 ([`6599801`](https://github.com/o2sh/onefetch/commit/65998017b5111cca0f5992ba48cb78b7c9d02f9b)) - Multicolor ascii for haskell/python/Clojure and news ascii for Cpp/Csharp and added support for php ([`7541574`](https://github.com/o2sh/onefetch/commit/75415749137b9063ab44d6626b6370c37c4cf82f)) - JavaScript added #22 ([`1147100`](https://github.com/o2sh/onefetch/commit/1147100db57b017a9cfbf54e1b2cafa726014512))
</details>

## v1.0.5 (2018-12-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 16 commits contributed to the release over the course of 40 calendar days.
- 42 days passed between releases.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Alter rust logo ([`4792027`](https://github.com/o2sh/onefetch/commit/4792027bb7b54700769c1510d65ceb30e7734a95)) - Multicolor java logo ([`1dfb3e0`](https://github.com/o2sh/onefetch/commit/1dfb3e0a578a17155e5e5337e0f09bfb7f1bbd19)) - Merge pull request #20 from xynxynxyn/master ([`1499618`](https://github.com/o2sh/onefetch/commit/1499618ae7ad1b0b83c0f7186129a70175b310c8)) - Multi color asciis ([`1fc91ba`](https://github.com/o2sh/onefetch/commit/1fc91ba0bec077a80bec3f61eb417283777a6188)) - Merge pull request #19 from xynxynxyn/master ([`0b7297a`](https://github.com/o2sh/onefetch/commit/0b7297a4ebb288ed24b8a2bec8ee50c0d5430d93)) - Fix clippy errors ([`71a1f70`](https://github.com/o2sh/onefetch/commit/71a1f7018397a1f08af632cd958125ccc3315813)) - More adapted error message in case of wrong folder ([`d93fe8a`](https://github.com/o2sh/onefetch/commit/d93fe8afbcb79c823704a975c47c5b10341f0493)) - Merge pull request #18 from xynxynxyn/master ([`17ea45d`](https://github.com/o2sh/onefetch/commit/17ea45dc7c334885739184e2c9d86628101e92c9)) - Custom error type and proper error messages ([`809d900`](https://github.com/o2sh/onefetch/commit/809d90026ddb4cf6318ece0dd5d22e4205541bdb)) - Return Errors instead of process:exit ([`26569e1`](https://github.com/o2sh/onefetch/commit/26569e1f69036ab607272737e08fd141872ada2e)) - Square brackets since unwrap is called anyways ([`9ec4df2`](https://github.com/o2sh/onefetch/commit/9ec4df2564331387df598ec6e8019c1f6e822a72)) - Sort_by_key to reduce clutter ([`2097896`](https://github.com/o2sh/onefetch/commit/209789627a127c60138dc9c8bd4eb7d687ff6542)) - Move color impl to Info instead of language ([`01594b5`](https://github.com/o2sh/onefetch/commit/01594b5bc78fc6495395c6fef451faa4b7a6d84c)) - Use language.color() implemented function instead of get_color(&language) ([`2b1454f`](https://github.com/o2sh/onefetch/commit/2b1454f081c163cdab73c37f679b310022d56cd6)) - Exit(1) instead of return ([`fce7441`](https://github.com/o2sh/onefetch/commit/fce744140f69123f425b4d2ae5eb473ae563c9b8)) - When not git repo --> exit(1) instead of panic! ([`92d15bf`](https://github.com/o2sh/onefetch/commit/92d15bf79b71d4ca5eec3188b4d274faa83bc024))
</details>

## v1.0.0 (2018-11-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

- 38 commits contributed to the release over the course of 48 calendar days.
- 0 commits were understood as [conventional](https://www.conventionalcommits.org).
- 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **Uncategorized** - Code review ([`3e726c8`](https://github.com/o2sh/onefetch/commit/3e726c8d3d8542cebf1e2543013505940a47be78)) - Merge pull request #16 from kitlith/license-detection ([`8864385`](https://github.com/o2sh/onefetch/commit/8864385ce18b888f0653401fc6cb3eb0bccf9649)) - Add basic license detection. ([`bf89e2f`](https://github.com/o2sh/onefetch/commit/bf89e2fd392db4282581c57e9027aa8e3e320e7e)) - Merge pull request #15 from francesco-dipi/master ([`604b987`](https://github.com/o2sh/onefetch/commit/604b9874783bbe386daced566973da5911a3b0eb)) - Add Lua ascii logo ([`f346b25`](https://github.com/o2sh/onefetch/commit/f346b2575881038d9499ff6c39240e173a972b28)) - Merge pull request #13 from JoshBrudnak/master ([`61a223b`](https://github.com/o2sh/onefetch/commit/61a223b1777fba12166e7188dbbc6232ea2f8761)) - Use last part of the repo url for the repo name ([`f851ece`](https://github.com/o2sh/onefetch/commit/f851eceb688e69ebe404e83218a93d52e415fcdc)) - Merge pull request #12 from zxey/info-detect ([`668c696`](https://github.com/o2sh/onefetch/commit/668c696cb99ba7e5aad3dbf85af6da9f8466c640)) - Detect authors ([`6f92fc9`](https://github.com/o2sh/onefetch/commit/6f92fc97bb6f1d109052d05f9ce69e93a8b75700)) - Detect repository name and url ([`6c0f43d`](https://github.com/o2sh/onefetch/commit/6c0f43dc754fdaa8451893d1e49c253c8f3aef3c)) - Formatted the project using rustfmt ([`a0e2d0a`](https://github.com/o2sh/onefetch/commit/a0e2d0a40e2a5e621e3a43aef6a20eb2b61fe0a0)) - Merge pull request #11 from rtroxler/master ([`f71b797`](https://github.com/o2sh/onefetch/commit/f71b7975de724b72e2b10a05cc332d5072062029)) - Grab LOC from tokei and add it to Info ([`2944ee8`](https://github.com/o2sh/onefetch/commit/2944ee8a4284910ab474103d21096aabcaccf7cf)) - Rustfmt ([`1779fb3`](https://github.com/o2sh/onefetch/commit/1779fb30e98b2e2305a68ba12ef8f28877e1ccb0)) - Merge pull request #10 from cnsumner/master ([`dc5c4fc`](https://github.com/o2sh/onefetch/commit/dc5c4fc793fb70ff5b00636f48f8467e55502392)) - Add support for typescript ([`314ec0a`](https://github.com/o2sh/onefetch/commit/314ec0a2ccb52c0f3ca933798027475b96c51426)) - Sort languages in ascending order in various places ([`51b2b9d`](https://github.com/o2sh/onefetch/commit/51b2b9d78b794c369afd4d1a3602444f82867633)) - Add zxey as contributor ([`2f7e689`](https://github.com/o2sh/onefetch/commit/2f7e689d0359d08225fd4b679c24979627d8b3ff)) - Merge pull request #6 from zxey/lang-detect ([`e145e6d`](https://github.com/o2sh/onefetch/commit/e145e6ddb4edb50110c2490fa1672cfb0c2b99d7)) - Merge pull request #7 from francesco-dipi/master ([`310e0b0`](https://github.com/o2sh/onefetch/commit/310e0b02e29aaffef948509c525792ce5999e21e)) - Add Clojure ascii logo ([`9076cb0`](https://github.com/o2sh/onefetch/commit/9076cb09575f68ad5d1e78e47c9e7084e547ae35)) - Merge pull request #5 from zxey/move-ascii-art ([`440e4a9`](https://github.com/o2sh/onefetch/commit/440e4a9fd49f44a28af18979133366ecb3ebea38)) - Detect dominant language type ([`b854fa9`](https://github.com/o2sh/onefetch/commit/b854fa9acdcbbf21b4735070f8727106d2d706e0)) - Move all ascii art to separate files ([`15b766d`](https://github.com/o2sh/onefetch/commit/15b766de72044cf1be761c7ea2956499129cbfdb)) - Merge pull request #3 from di-wu/r ([`3e512fb`](https://github.com/o2sh/onefetch/commit/3e512fbc6a4aeb3f37c7a98229a2c3946aecaaf4)) - R ASCII! ([`00162aa`](https://github.com/o2sh/onefetch/commit/00162aa0acb18677c1dabbce6737729eb725b16d)) - Preview images ([`5eaae5b`](https://github.com/o2sh/onefetch/commit/5eaae5bb1fad24298833bb19bfa1f9f7ea24b17e)) - Switch colors ([`e028804`](https://github.com/o2sh/onefetch/commit/e0288045493dda817910dcae5c796c581b3dace2)) - Color ([`c7ec9f7`](https://github.com/o2sh/onefetch/commit/c7ec9f78205009000570692e665b3ed8c2af1b38)) - Empty line beginning ([`d1bb3dc`](https://github.com/o2sh/onefetch/commit/d1bb3dc6e7ae76b601da28257b3a83516aa0e286)) - Logo and info side by side ([`e19fb33`](https://github.com/o2sh/onefetch/commit/e19fb3300bb52ecd9b27129a0c3ad8f43ff1a3db)) - Ascii logos ([`8062d92`](https://github.com/o2sh/onefetch/commit/8062d926c7d0e1d871bd2f5204f3e175edb13949)) - Fields info ([`6eb2a8d`](https://github.com/o2sh/onefetch/commit/6eb2a8d2af313aa2b8b911ce53c02af0451143c9)) - Barely trying ([`ac35d3c`](https://github.com/o2sh/onefetch/commit/ac35d3cc0f9c9563a9978d405cd427c7a7e048ed)) - Color output ([`668a3d7`](https://github.com/o2sh/onefetch/commit/668a3d7f54f97b0270210d7e1c88f92a7a87e522)) - Enum andd struct ([`8135e95`](https://github.com/o2sh/onefetch/commit/8135e95cda56d0568160b1b4eb27cd1f59dee0df)) - Side by side ([`d90f86d`](https://github.com/o2sh/onefetch/commit/d90f86dab8d854349729eae9725d7ffeaa099b64)) - Init project ([`8556f9a`](https://github.com/o2sh/onefetch/commit/8556f9af5c5b61154502f3f92cdc4201de6a61ab))
</details>
