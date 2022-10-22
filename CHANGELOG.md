# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

<csr-id-b6cd415d049b24348150e0e2088f2fdb5822e1cb/>
<csr-id-7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e/>
<csr-id-d00ab45d3cab26e6c8394c2952d7704dd58b8245/>
<csr-id-d43fa9acbbc93cfee2e59faf3652e7893de55ffa/>
<csr-id-5e4d02552beea1a998239360fe61b8465437884a/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update changelog ([`fd22b84`](https://github.com/o2sh/onefetch/commit/fd22b847cc0e2779535d4fb28bd6c3ff01c2b474))
    - update Cargo.lock ([`5c01e18`](https://github.com/o2sh/onefetch/commit/5c01e1824cbd8801e1c99a0a41ce751d2f4bf37c))
</details>

## 2.13.0 (2022-10-21)

<csr-id-d43fa9acbbc93cfee2e59faf3652e7893de55ffa/>
<csr-id-5e4d02552beea1a998239360fe61b8465437884a/>
<csr-id-b6cd415d049b24348150e0e2088f2fdb5822e1cb/>
<csr-id-7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e/>
<csr-id-d00ab45d3cab26e6c8394c2952d7704dd58b8245/>

`onefetch` is now typically more than twice as fast when executing. This was achieved by reducing
allocations and switching to `gitoxide` for the most taxing tasks.

A new web interface [onefetch.dev](https://onefetch.dev) was developed where users can visualize an ASCII preview for all the programming languages currently supported by onefetch. Like the binary, the data is parsed from the Languages.yaml file.

### Other

- <csr-id-d43fa9acbbc93cfee2e59faf3652e7893de55ffa/> tokei is now an exempted pr label
- <csr-id-5e4d02552beea1a998239360fe61b8465437884a/> tokei is now an exempted issue label
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

 - 319 commits contributed to the release over the course of 205 calendar days.
 - 206 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 115 unique issues were worked on: [#631](https://github.com/o2sh/onefetch/issues/631), [#646](https://github.com/o2sh/onefetch/issues/646), [#647](https://github.com/o2sh/onefetch/issues/647), [#648](https://github.com/o2sh/onefetch/issues/648), [#650](https://github.com/o2sh/onefetch/issues/650), [#652](https://github.com/o2sh/onefetch/issues/652), [#653](https://github.com/o2sh/onefetch/issues/653), [#657](https://github.com/o2sh/onefetch/issues/657), [#660](https://github.com/o2sh/onefetch/issues/660), [#661](https://github.com/o2sh/onefetch/issues/661), [#664](https://github.com/o2sh/onefetch/issues/664), [#666](https://github.com/o2sh/onefetch/issues/666), [#667](https://github.com/o2sh/onefetch/issues/667), [#668](https://github.com/o2sh/onefetch/issues/668), [#672](https://github.com/o2sh/onefetch/issues/672), [#673](https://github.com/o2sh/onefetch/issues/673), [#674](https://github.com/o2sh/onefetch/issues/674), [#676](https://github.com/o2sh/onefetch/issues/676), [#677](https://github.com/o2sh/onefetch/issues/677), [#678](https://github.com/o2sh/onefetch/issues/678), [#679](https://github.com/o2sh/onefetch/issues/679), [#680](https://github.com/o2sh/onefetch/issues/680), [#683](https://github.com/o2sh/onefetch/issues/683), [#685](https://github.com/o2sh/onefetch/issues/685), [#686](https://github.com/o2sh/onefetch/issues/686), [#687](https://github.com/o2sh/onefetch/issues/687), [#688](https://github.com/o2sh/onefetch/issues/688), [#692](https://github.com/o2sh/onefetch/issues/692), [#693](https://github.com/o2sh/onefetch/issues/693), [#694](https://github.com/o2sh/onefetch/issues/694), [#698](https://github.com/o2sh/onefetch/issues/698), [#699](https://github.com/o2sh/onefetch/issues/699), [#700](https://github.com/o2sh/onefetch/issues/700), [#701](https://github.com/o2sh/onefetch/issues/701), [#705](https://github.com/o2sh/onefetch/issues/705), [#708](https://github.com/o2sh/onefetch/issues/708), [#711](https://github.com/o2sh/onefetch/issues/711), [#714](https://github.com/o2sh/onefetch/issues/714), [#716](https://github.com/o2sh/onefetch/issues/716), [#717](https://github.com/o2sh/onefetch/issues/717), [#718](https://github.com/o2sh/onefetch/issues/718), [#720](https://github.com/o2sh/onefetch/issues/720), [#722](https://github.com/o2sh/onefetch/issues/722), [#723](https://github.com/o2sh/onefetch/issues/723), [#724](https://github.com/o2sh/onefetch/issues/724), [#725](https://github.com/o2sh/onefetch/issues/725), [#726](https://github.com/o2sh/onefetch/issues/726), [#727](https://github.com/o2sh/onefetch/issues/727), [#729](https://github.com/o2sh/onefetch/issues/729), [#730](https://github.com/o2sh/onefetch/issues/730), [#731](https://github.com/o2sh/onefetch/issues/731), [#733](https://github.com/o2sh/onefetch/issues/733), [#736](https://github.com/o2sh/onefetch/issues/736), [#737](https://github.com/o2sh/onefetch/issues/737), [#738](https://github.com/o2sh/onefetch/issues/738), [#741](https://github.com/o2sh/onefetch/issues/741), [#742](https://github.com/o2sh/onefetch/issues/742), [#743](https://github.com/o2sh/onefetch/issues/743), [#744](https://github.com/o2sh/onefetch/issues/744), [#748](https://github.com/o2sh/onefetch/issues/748), [#751](https://github.com/o2sh/onefetch/issues/751), [#752](https://github.com/o2sh/onefetch/issues/752), [#753](https://github.com/o2sh/onefetch/issues/753), [#755](https://github.com/o2sh/onefetch/issues/755), [#756](https://github.com/o2sh/onefetch/issues/756), [#757](https://github.com/o2sh/onefetch/issues/757), [#758](https://github.com/o2sh/onefetch/issues/758), [#759](https://github.com/o2sh/onefetch/issues/759), [#761](https://github.com/o2sh/onefetch/issues/761), [#762](https://github.com/o2sh/onefetch/issues/762), [#763](https://github.com/o2sh/onefetch/issues/763), [#764](https://github.com/o2sh/onefetch/issues/764), [#770](https://github.com/o2sh/onefetch/issues/770), [#771](https://github.com/o2sh/onefetch/issues/771), [#772](https://github.com/o2sh/onefetch/issues/772), [#773](https://github.com/o2sh/onefetch/issues/773), [#774](https://github.com/o2sh/onefetch/issues/774), [#775](https://github.com/o2sh/onefetch/issues/775), [#778](https://github.com/o2sh/onefetch/issues/778), [#779](https://github.com/o2sh/onefetch/issues/779), [#780](https://github.com/o2sh/onefetch/issues/780), [#781](https://github.com/o2sh/onefetch/issues/781), [#782](https://github.com/o2sh/onefetch/issues/782), [#784](https://github.com/o2sh/onefetch/issues/784), [#785](https://github.com/o2sh/onefetch/issues/785), [#786](https://github.com/o2sh/onefetch/issues/786), [#789](https://github.com/o2sh/onefetch/issues/789), [#791](https://github.com/o2sh/onefetch/issues/791), [#792](https://github.com/o2sh/onefetch/issues/792), [#793](https://github.com/o2sh/onefetch/issues/793), [#794](https://github.com/o2sh/onefetch/issues/794), [#795](https://github.com/o2sh/onefetch/issues/795), [#797](https://github.com/o2sh/onefetch/issues/797), [#798](https://github.com/o2sh/onefetch/issues/798), [#800](https://github.com/o2sh/onefetch/issues/800), [#801](https://github.com/o2sh/onefetch/issues/801), [#802](https://github.com/o2sh/onefetch/issues/802), [#803](https://github.com/o2sh/onefetch/issues/803), [#804](https://github.com/o2sh/onefetch/issues/804), [#805](https://github.com/o2sh/onefetch/issues/805), [#806](https://github.com/o2sh/onefetch/issues/806), [#807](https://github.com/o2sh/onefetch/issues/807), [#808](https://github.com/o2sh/onefetch/issues/808), [#809](https://github.com/o2sh/onefetch/issues/809), [#810](https://github.com/o2sh/onefetch/issues/810), [#811](https://github.com/o2sh/onefetch/issues/811), [#812](https://github.com/o2sh/onefetch/issues/812), [#813](https://github.com/o2sh/onefetch/issues/813), [#814](https://github.com/o2sh/onefetch/issues/814), [#815](https://github.com/o2sh/onefetch/issues/815), [#818](https://github.com/o2sh/onefetch/issues/818), [#820](https://github.com/o2sh/onefetch/issues/820), [#821](https://github.com/o2sh/onefetch/issues/821), [#828](https://github.com/o2sh/onefetch/issues/828), [#829](https://github.com/o2sh/onefetch/issues/829)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#631](https://github.com/o2sh/onefetch/issues/631)**
    - Add README translation for Spanish ([`0f41ae4`](https://github.com/o2sh/onefetch/commit/0f41ae4aa96412bc07639db8fd26f6479fb56689))
 * **[#646](https://github.com/o2sh/onefetch/issues/646)**
    - Bump clap from 3.1.9 to 3.1.12 ([`80c20a2`](https://github.com/o2sh/onefetch/commit/80c20a2e5c09470107a74eeab5cdad21ee38b4db))
 * **[#647](https://github.com/o2sh/onefetch/issues/647)**
    - Bump owo-colors from 3.3.0 to 3.4.0 ([`da63743`](https://github.com/o2sh/onefetch/commit/da63743118f17c7cef4102f90d3c9e77340c32f6))
 * **[#648](https://github.com/o2sh/onefetch/issues/648)**
    - Bump anyhow from 1.0.56 to 1.0.57 ([`76619bf`](https://github.com/o2sh/onefetch/commit/76619bf68605c2cc3787d661574959a777b9c7f3))
 * **[#650](https://github.com/o2sh/onefetch/issues/650)**
    - Add bug report issue template ([`495336d`](https://github.com/o2sh/onefetch/commit/495336d34916d9a71e39300af2b191f2c3d41a5d))
 * **[#652](https://github.com/o2sh/onefetch/issues/652)**
    - Bump image from 0.24.1 to 0.24.2 ([`c72c4a0`](https://github.com/o2sh/onefetch/commit/c72c4a05d5529c3220aa6b07b7af92bc34dc4874))
 * **[#653](https://github.com/o2sh/onefetch/issues/653)**
    - Bump clap from 3.1.12 to 3.1.15 ([`97ba0c9`](https://github.com/o2sh/onefetch/commit/97ba0c9490fd738137754ebda1c4e16ef5ea012e))
 * **[#657](https://github.com/o2sh/onefetch/issues/657)**
    - Add `--completion` option ([`9aec77c`](https://github.com/o2sh/onefetch/commit/9aec77ca625d4f1114786d6bf4c9e20d5d311633))
 * **[#660](https://github.com/o2sh/onefetch/issues/660)**
    - Bump clap from 3.1.15 to 3.1.17 ([`7f14c96`](https://github.com/o2sh/onefetch/commit/7f14c9672f6d208d385176fba43d56a75894c1e7))
 * **[#661](https://github.com/o2sh/onefetch/issues/661)**
    - Bump clap_complete from 3.1.3 to 3.1.4 ([`949c3fe`](https://github.com/o2sh/onefetch/commit/949c3fe043c3432e5e53c315ba051a1925dd9d79))
 * **[#664](https://github.com/o2sh/onefetch/issues/664)**
    - Update PHP colors ([`415c460`](https://github.com/o2sh/onefetch/commit/415c4601b786b699f0b67728b10101fbd96fb1f7))
 * **[#666](https://github.com/o2sh/onefetch/issues/666)**
    - switch to actions/stale ([`3a2aa4b`](https://github.com/o2sh/onefetch/commit/3a2aa4b67278b56605a6ddcd818407c8298b0f48))
 * **[#667](https://github.com/o2sh/onefetch/issues/667)**
    - Add github issue forms ([`a3ab809`](https://github.com/o2sh/onefetch/commit/a3ab809515ed38dbdd017028dcfe0abb8a3bfcbb))
 * **[#668](https://github.com/o2sh/onefetch/issues/668)**
    - Generate Windows installer from CD ([`ac338c8`](https://github.com/o2sh/onefetch/commit/ac338c85a0b3d68a27c88ab458ae14169593930d))
 * **[#672](https://github.com/o2sh/onefetch/issues/672)**
    - add CODEOWNERS ([`7d37ef7`](https://github.com/o2sh/onefetch/commit/7d37ef7ad37eae02d169df8a062318a89244b50a))
 * **[#673](https://github.com/o2sh/onefetch/issues/673)**
    - Create WinGet workflow ([`b886823`](https://github.com/o2sh/onefetch/commit/b88682338a6b18d4b0a94e5034ac41bd6745505a))
 * **[#674](https://github.com/o2sh/onefetch/issues/674)**
    - Bump spenserblack/actions-msrv from 0.3 to 0.4 ([`523e1dd`](https://github.com/o2sh/onefetch/commit/523e1dde9c6f4d6d5c5834e6ed4cadd8b90a3405))
 * **[#676](https://github.com/o2sh/onefetch/issues/676)**
    - Bump regex from 1.5.5 to 1.5.6 ([`4c01553`](https://github.com/o2sh/onefetch/commit/4c01553d35529935186e15bfabcbd5a37ddb54f8))
 * **[#677](https://github.com/o2sh/onefetch/issues/677)**
    - update logo: shell ([`2c2d827`](https://github.com/o2sh/onefetch/commit/2c2d8271e6dc6b79b92186385d07b8c9e85542cf))
 * **[#678](https://github.com/o2sh/onefetch/issues/678)**
    - Bump bytecount from 0.6.2 to 0.6.3 ([`86f7307`](https://github.com/o2sh/onefetch/commit/86f730791348357e9306d8b028a64341f1ed618e))
 * **[#679](https://github.com/o2sh/onefetch/issues/679)**
    - Bump more-asserts from 0.2.2 to 0.3.0 ([`2c3cef9`](https://github.com/o2sh/onefetch/commit/2c3cef9fd9e8181381ac1bff779f950927feadd5))
 * **[#680](https://github.com/o2sh/onefetch/issues/680)**
    - fix for --ascii-colors and --ascii-input ([`9f9764d`](https://github.com/o2sh/onefetch/commit/9f9764db812b552365db96cb9e789e24df5cd132))
 * **[#683](https://github.com/o2sh/onefetch/issues/683)**
    - Bump strum from 0.24.0 to 0.24.1 ([`023c091`](https://github.com/o2sh/onefetch/commit/023c09180e4aee4af50db5678e9d09a4b838b610))
 * **[#685](https://github.com/o2sh/onefetch/issues/685)**
    - Bump clap from 3.1.18 to 3.2.5 and use clap derive ([`2fa44e2`](https://github.com/o2sh/onefetch/commit/2fa44e2b2517cdcc9eef3fbd606d173f2c178a07))
 * **[#686](https://github.com/o2sh/onefetch/issues/686)**
    - Update cd.yml ([`40bdaef`](https://github.com/o2sh/onefetch/commit/40bdaef1751e9e8dd94e24c5628ecee53ac8cd90))
 * **[#687](https://github.com/o2sh/onefetch/issues/687)**
    - Bump askalono from 0.4.5 to 0.4.6 ([`73cfe23`](https://github.com/o2sh/onefetch/commit/73cfe23c18f9d30cb136bb5efbbcf416c47b9606))
 * **[#688](https://github.com/o2sh/onefetch/issues/688)**
    - Bump anyhow from 1.0.57 to 1.0.58 ([`5d88e65`](https://github.com/o2sh/onefetch/commit/5d88e653217e9b56c5ca9d474b18d8d52f3cc7a4))
 * **[#692](https://github.com/o2sh/onefetch/issues/692)**
    - Bump time from 0.3.9 to 0.3.11 ([`d59d7c2`](https://github.com/o2sh/onefetch/commit/d59d7c29172b899002855f8120e0584f35ffb568))
 * **[#693](https://github.com/o2sh/onefetch/issues/693)**
    - adding french documentation support ([`5dd1ad8`](https://github.com/o2sh/onefetch/commit/5dd1ad8c7590267ec5698b95aee3b89ced3aa34a))
 * **[#694](https://github.com/o2sh/onefetch/issues/694)**
    - Ignore docs for CI runs ([`795a420`](https://github.com/o2sh/onefetch/commit/795a420df9e9b5f29709d458d4c67d9cc55ad9a4))
 * **[#698](https://github.com/o2sh/onefetch/issues/698)**
    - Bump clap_complete from 3.2.1 to 3.2.3 ([`f0dfb28`](https://github.com/o2sh/onefetch/commit/f0dfb28ec30d7b24d32d2b765b10f74818c72639))
 * **[#699](https://github.com/o2sh/onefetch/issues/699)**
    - Extract language definitions into data file ([`d4e6cda`](https://github.com/o2sh/onefetch/commit/d4e6cdadc6c89efe994b65ead870af27e6468ee9))
 * **[#700](https://github.com/o2sh/onefetch/issues/700)**
    - Add tests for `author.rs` ([`9cf8df4`](https://github.com/o2sh/onefetch/commit/9cf8df44196cfec2e5c8514f6616fcd5e8b4d77d))
 * **[#701](https://github.com/o2sh/onefetch/issues/701)**
    - Create Vercel app for onefetch with ASCII preview ([`a2d6cf1`](https://github.com/o2sh/onefetch/commit/a2d6cf171a0b0b79f0b3b83398710132a892910c))
 * **[#705](https://github.com/o2sh/onefetch/issues/705)**
    - gitoxide update ([`2bacf5a`](https://github.com/o2sh/onefetch/commit/2bacf5ad5d616c426cc621f59855b8988522016a))
 * **[#708](https://github.com/o2sh/onefetch/issues/708)**
    - Bump actions/stale from 5.0.0 to 5.1.1 ([`d00885b`](https://github.com/o2sh/onefetch/commit/d00885b519a4b2196a14905bf0441da22d8a91dd))
 * **[#711](https://github.com/o2sh/onefetch/issues/711)**
    - Bump vite from 3.0.0 to 3.0.4 in /docs/vercel ([`b01ba21`](https://github.com/o2sh/onefetch/commit/b01ba212e59cfad1983eca53935c23742e711be7))
 * **[#714](https://github.com/o2sh/onefetch/issues/714)**
    - Fix working directory for Vercel CI ([`2b4de39`](https://github.com/o2sh/onefetch/commit/2b4de39665f069be173d5fa119660d714a3605d1))
 * **[#716](https://github.com/o2sh/onefetch/issues/716)**
    - Bump regex from 1.5.6 to 1.6.0 ([`c65b46f`](https://github.com/o2sh/onefetch/commit/c65b46f1787fa684485256eaaf62fc2d157bed40))
 * **[#717](https://github.com/o2sh/onefetch/issues/717)**
    - Bump clap from 3.2.5 to 3.2.16 ([`aaaf89b`](https://github.com/o2sh/onefetch/commit/aaaf89b60b84ade611d301a45a11467603da98e8))
 * **[#718](https://github.com/o2sh/onefetch/issues/718)**
    - Bump git2 from 0.14.4 to 0.15.0 ([`bbcafe2`](https://github.com/o2sh/onefetch/commit/bbcafe2d03a9a9f901cc75006c43bdbec671e709))
 * **[#720](https://github.com/o2sh/onefetch/issues/720)**
    - Bump anyhow from 1.0.58 to 1.0.59 ([`8a641e8`](https://github.com/o2sh/onefetch/commit/8a641e8c36bf9103261c7482cc92285353911271))
 * **[#722](https://github.com/o2sh/onefetch/issues/722)**
    - Bump eslint from 8.20.0 to 8.21.0 in /docs/vercel ([`c91d805`](https://github.com/o2sh/onefetch/commit/c91d8056877eadacbc445692ff0b7027fcbd5f0d))
 * **[#723](https://github.com/o2sh/onefetch/issues/723)**
    - Add stylechecks to Vercel CI ([`1a481c8`](https://github.com/o2sh/onefetch/commit/1a481c82de1aed9b701cd933cdb26b9244ff19b3))
 * **[#724](https://github.com/o2sh/onefetch/issues/724)**
    - Bump @vercel/node from 2.4.4 to 2.5.3 in /docs/vercel ([`83d509b`](https://github.com/o2sh/onefetch/commit/83d509bf964ac43d3de39d8a704192f64b074520))
 * **[#725](https://github.com/o2sh/onefetch/issues/725)**
    - Simplify web layout ([`9b5a799`](https://github.com/o2sh/onefetch/commit/9b5a799ef3bf68013406aaa53a378a5736cfa170))
 * **[#726](https://github.com/o2sh/onefetch/issues/726)**
    - Bump serde from 1.0.137 to 1.0.142 ([`244c2e6`](https://github.com/o2sh/onefetch/commit/244c2e6214e7f11963baf02863dcc10ade18a26a))
 * **[#727](https://github.com/o2sh/onefetch/issues/727)**
    - Bump serde_json from 1.0.81 to 1.0.83 ([`b197c69`](https://github.com/o2sh/onefetch/commit/b197c6935904d9e8d8910dda075e26022cfab2c5))
 * **[#729](https://github.com/o2sh/onefetch/issues/729)**
    - Bump serde_yaml from 0.8.24 to 0.9.4 ([`af0db35`](https://github.com/o2sh/onefetch/commit/af0db35452aee00b4435b6da60abb6821c72c687))
 * **[#730](https://github.com/o2sh/onefetch/issues/730)**
    - Bump image from 0.24.2 to 0.24.3 ([`46798a6`](https://github.com/o2sh/onefetch/commit/46798a6e5c1b61ca83c6690bae6616f91dec7f4c))
 * **[#731](https://github.com/o2sh/onefetch/issues/731)**
    - Bump tera from 1.15.0 to 1.16.0 ([`da42d35`](https://github.com/o2sh/onefetch/commit/da42d355fa3ed3c60c23401d22af5eb31c52d8a3))
 * **[#733](https://github.com/o2sh/onefetch/issues/733)**
    - Switch to `terminal_size` ([`68fe7e4`](https://github.com/o2sh/onefetch/commit/68fe7e4d78f37f16eb8cb06b0b54cfe875b36fff))
 * **[#736](https://github.com/o2sh/onefetch/issues/736)**
    - Update the README in Russian ([`f3bf446`](https://github.com/o2sh/onefetch/commit/f3bf446933d5a15a35d72dd30bb87c9eb8497717))
 * **[#737](https://github.com/o2sh/onefetch/issues/737)**
    - Update the README in Russian ([`85e1b79`](https://github.com/o2sh/onefetch/commit/85e1b79e527ddd383e5dfd9471c7e5b44b980cfc))
 * **[#738](https://github.com/o2sh/onefetch/issues/738)**
    - Bump clap from 3.2.16 to 3.2.17 ([`2dc2e63`](https://github.com/o2sh/onefetch/commit/2dc2e630547f3990a17d0ff484f019150f1fed4a))
 * **[#741](https://github.com/o2sh/onefetch/issues/741)**
    - Bump serde from 1.0.142 to 1.0.143 ([`83e3769`](https://github.com/o2sh/onefetch/commit/83e376991d4171615e7a52278aa1185448abae75))
 * **[#742](https://github.com/o2sh/onefetch/issues/742)**
    - Bump tera from 1.16.0 to 1.17.0 ([`109d978`](https://github.com/o2sh/onefetch/commit/109d978b4a166026b4ec834b6ff0d8ff98aefaa2))
 * **[#743](https://github.com/o2sh/onefetch/issues/743)**
    - Bump time from 0.3.11 to 0.3.13 ([`2260933`](https://github.com/o2sh/onefetch/commit/22609338ef34259acee4065a50049589dcee08bf))
 * **[#744](https://github.com/o2sh/onefetch/issues/744)**
    - Bump clap_complete from 3.2.3 to 3.2.4 ([`ff08e2a`](https://github.com/o2sh/onefetch/commit/ff08e2a6df8c5a0e0d2b383d099eecbad53c2907))
 * **[#748](https://github.com/o2sh/onefetch/issues/748)**
    - Bump serde from 1.0.143 to 1.0.144 ([`f0ff6e4`](https://github.com/o2sh/onefetch/commit/f0ff6e470fe1a34c5cfcf9fd3544f31770ed7b85))
 * **[#751](https://github.com/o2sh/onefetch/issues/751)**
    - Bump serde_json from 1.0.83 to 1.0.85 ([`3704ddc`](https://github.com/o2sh/onefetch/commit/3704ddc893be4d9a59d7fa708219ea47891f7aff))
 * **[#752](https://github.com/o2sh/onefetch/issues/752)**
    - update gitoxide to v0.21.1 ([`f52fed6`](https://github.com/o2sh/onefetch/commit/f52fed6423a6136d5af24900771b06238415fe0b))
 * **[#753](https://github.com/o2sh/onefetch/issues/753)**
    - Bump git-repository from 0.22.0 to 0.22.1 ([`388cf7c`](https://github.com/o2sh/onefetch/commit/388cf7cf77c47519bd58b897c610cebb27e44a91))
 * **[#755](https://github.com/o2sh/onefetch/issues/755)**
    - Turn InfoField into a trait (big refactoring) ([`066be27`](https://github.com/o2sh/onefetch/commit/066be27eb2128b1257e35d25f2a5decfab0dc955))
 * **[#756](https://github.com/o2sh/onefetch/issues/756)**
    - Fix typos in CHANGELOG.md ([`7623f7d`](https://github.com/o2sh/onefetch/commit/7623f7d7f07ec5d62ecc99007ea56fe7cbebb9e1))
 * **[#757](https://github.com/o2sh/onefetch/issues/757)**
    - Bump iana-time-zone from 0.1.44 to 0.1.46 ([`9571ba6`](https://github.com/o2sh/onefetch/commit/9571ba647f676adb35d00770a5a99b9334e87784))
 * **[#758](https://github.com/o2sh/onefetch/issues/758)**
    - Bump svelte-check from 2.8.0 to 2.9.0 in /docs/vercel ([`3979fd4`](https://github.com/o2sh/onefetch/commit/3979fd4dc623c8a7aa9b6cfa6f2f4551c7b265df))
 * **[#759](https://github.com/o2sh/onefetch/issues/759)**
    - Bump eslint from 8.21.0 to 8.23.0 in /docs/vercel ([`f8667a9`](https://github.com/o2sh/onefetch/commit/f8667a9f4f4e9c10433e33ecbe134458b08ee23a))
 * **[#761](https://github.com/o2sh/onefetch/issues/761)**
    - Bump vite from 3.0.4 to 3.0.9 in /docs/vercel ([`70c6681`](https://github.com/o2sh/onefetch/commit/70c668184476afe6c39d75fb40b8c1feec3a5a62))
 * **[#762](https://github.com/o2sh/onefetch/issues/762)**
    - Bump @sveltejs/vite-plugin-svelte from 1.0.1 to 1.0.4 in /docs/vercel ([`61e1644`](https://github.com/o2sh/onefetch/commit/61e16440fba9194711c8864e3d0340b2cc4aaaab))
 * **[#763](https://github.com/o2sh/onefetch/issues/763)**
    - Bump @typescript-eslint/eslint-plugin in /docs/vercel ([`049f2b2`](https://github.com/o2sh/onefetch/commit/049f2b26be118a375d7c8e06b6fedf8dcec67ad0))
 * **[#764](https://github.com/o2sh/onefetch/issues/764)**
    - Bump typescript from 4.7.4 to 4.8.2 in /docs/vercel ([`38b72aa`](https://github.com/o2sh/onefetch/commit/38b72aaac77a44271a39cedd4e00f3e54e92f0c2))
 * **[#770](https://github.com/o2sh/onefetch/issues/770)**
    - Bump anyhow from 1.0.59 to 1.0.65 ([`3b111b7`](https://github.com/o2sh/onefetch/commit/3b111b76ac07bb7bf9cabdb5358a653a81d5f141))
 * **[#771](https://github.com/o2sh/onefetch/issues/771)**
    - Bump serde_yaml from 0.9.4 to 0.9.13 ([`d46d7bc`](https://github.com/o2sh/onefetch/commit/d46d7bcad06e18523e90cdcec5646f10b09a4323))
 * **[#772](https://github.com/o2sh/onefetch/issues/772)**
    - Bump owo-colors from 3.4.0 to 3.5.0 ([`c760755`](https://github.com/o2sh/onefetch/commit/c760755e642eea0eebc9e0d17da18818e837d176))
 * **[#773](https://github.com/o2sh/onefetch/issues/773)**
    - Bump time from 0.3.13 to 0.3.14 ([`28ba241`](https://github.com/o2sh/onefetch/commit/28ba2418ac4bf4efa762f26573d3d9408e8e117c))
 * **[#774](https://github.com/o2sh/onefetch/issues/774)**
    - Bump clap_complete from 3.2.4 to 3.2.5 ([`ad83bd9`](https://github.com/o2sh/onefetch/commit/ad83bd9b277cb24302f5bda778feff674ce1bad1))
 * **[#775](https://github.com/o2sh/onefetch/issues/775)**
    - Bump clap from 3.2.17 to 3.2.22 ([`16694b3`](https://github.com/o2sh/onefetch/commit/16694b395f6bb938e6b25e54688bbfe48b692269))
 * **[#778](https://github.com/o2sh/onefetch/issues/778)**
    - Update cd.yml ([`59c8220`](https://github.com/o2sh/onefetch/commit/59c82204d96890a50ecd6177d8608a28f2871b02))
 * **[#779](https://github.com/o2sh/onefetch/issues/779)**
    - Bump svelte from 3.49.0 to 3.50.1 in /docs/vercel ([`856ffde`](https://github.com/o2sh/onefetch/commit/856ffde5d9ec35742a696117f491dfe99166a13c))
 * **[#780](https://github.com/o2sh/onefetch/issues/780)**
    - Bump vite from 3.0.9 to 3.1.4 in /docs/vercel ([`e692ed6`](https://github.com/o2sh/onefetch/commit/e692ed6becc8e1ba5788a74d438f49d9b684bffe))
 * **[#781](https://github.com/o2sh/onefetch/issues/781)**
    - Bump typescript from 4.8.2 to 4.8.4 in /docs/vercel ([`799e3f8`](https://github.com/o2sh/onefetch/commit/799e3f84ff23f01e515fdf12374d7c9b6af09e0b))
 * **[#782](https://github.com/o2sh/onefetch/issues/782)**
    - Bump prettier-plugin-svelte from 2.7.0 to 2.7.1 in /docs/vercel ([`1b322c2`](https://github.com/o2sh/onefetch/commit/1b322c2afbcd28cdbdab5461e1bee9baec1d3400))
 * **[#784](https://github.com/o2sh/onefetch/issues/784)**
    - Improve JSX ASCII logo ([`1ab5be9`](https://github.com/o2sh/onefetch/commit/1ab5be9e2ef015c3e33ac5d2a257783cd466b643))
 * **[#785](https://github.com/o2sh/onefetch/issues/785)**
    - Improve TSX ASCII logo ([`1fc333a`](https://github.com/o2sh/onefetch/commit/1fc333a61993a772a96940132fa0fb5470d6a917))
 * **[#786](https://github.com/o2sh/onefetch/issues/786)**
    - improve ruby logo #777 ([`b9b57a7`](https://github.com/o2sh/onefetch/commit/b9b57a72dcc0407a796bacb4969dc3e7c549dcc2))
 * **[#789](https://github.com/o2sh/onefetch/issues/789)**
    - adding language verilog #490 ([`3c3c1b4`](https://github.com/o2sh/onefetch/commit/3c3c1b48bb8ef5bcd9694380a2acbfdd5727bc61))
 * **[#791](https://github.com/o2sh/onefetch/issues/791)**
    - Bump clap from 3.2.22 to 4.0.8 ([`9681164`](https://github.com/o2sh/onefetch/commit/968116428f35d0ec61ab90a96ac0f1c372fe8065))
 * **[#792](https://github.com/o2sh/onefetch/issues/792)**
    - Bump image from 0.24.3 to 0.24.4 ([`bb81354`](https://github.com/o2sh/onefetch/commit/bb81354a9cdc54c1265b06e4ddc2523da6f4164a))
 * **[#793](https://github.com/o2sh/onefetch/issues/793)**
    - Bump actions/stale from 5.1.1 to 6.0.0 ([`1d5fd1c`](https://github.com/o2sh/onefetch/commit/1d5fd1cd7a11afe57d641054e651a91901ab6ec2))
 * **[#794](https://github.com/o2sh/onefetch/issues/794)**
    - Bump libc from 0.2.126 to 0.2.134 ([`2318bd2`](https://github.com/o2sh/onefetch/commit/2318bd2bdc7d2ae5329d1851593012a661c4a695))
 * **[#795](https://github.com/o2sh/onefetch/issues/795)**
    - Improve Bash logo ([`4e5b2e2`](https://github.com/o2sh/onefetch/commit/4e5b2e258d2265cf10e134c292dad91455f04eea))
 * **[#797](https://github.com/o2sh/onefetch/issues/797)**
    - adding language systemverilog #490 ([`8f27c93`](https://github.com/o2sh/onefetch/commit/8f27c93861dc6c0de2c3a72ce3c8ba85d0d9dfbc))
 * **[#798](https://github.com/o2sh/onefetch/issues/798)**
    - adding language xsl #490 ([`baafdad`](https://github.com/o2sh/onefetch/commit/baafdad9b90270f67e7c3aaaffd5672c2965d2b0))
 * **[#800](https://github.com/o2sh/onefetch/issues/800)**
    - Automate publish to crates.io ([`6eabbed`](https://github.com/o2sh/onefetch/commit/6eabbed679c5621718e8f83f6419f7daa020bed5))
 * **[#801](https://github.com/o2sh/onefetch/issues/801)**
    - Bump tera from 1.17.0 to 1.17.1 ([`8a32c48`](https://github.com/o2sh/onefetch/commit/8a32c487d9128c2ee1bc8b9a141ddf5d928a7e06))
 * **[#802](https://github.com/o2sh/onefetch/issues/802)**
    - Bump clap from 4.0.8 to 4.0.10 ([`8be72ac`](https://github.com/o2sh/onefetch/commit/8be72ace9e72e14be4fbd02ae7c52b028203098d))
 * **[#803](https://github.com/o2sh/onefetch/issues/803)**
    - Bump serde from 1.0.144 to 1.0.145 ([`27189e0`](https://github.com/o2sh/onefetch/commit/27189e0c639d5664006b1bc0ed62b16379a94d42))
 * **[#804](https://github.com/o2sh/onefetch/issues/804)**
    - Bump @typescript-eslint/eslint-plugin in /docs/vercel ([`c7b032e`](https://github.com/o2sh/onefetch/commit/c7b032ead820b1f764f5b1356fa67734f9c7d50b))
 * **[#805](https://github.com/o2sh/onefetch/issues/805)**
    - Bump @typescript-eslint/parser from 5.32.0 to 5.39.0 in /docs/vercel ([`38adb14`](https://github.com/o2sh/onefetch/commit/38adb14eea23e6c44264f95a67b65e90e02c8ade))
 * **[#806](https://github.com/o2sh/onefetch/issues/806)**
    - Bump eslint from 8.23.0 to 8.24.0 in /docs/vercel ([`2a80e7b`](https://github.com/o2sh/onefetch/commit/2a80e7ba7ad0077282836122902059d2dbe4877d))
 * **[#807](https://github.com/o2sh/onefetch/issues/807)**
    - Bump svelte-check from 2.9.0 to 2.9.1 in /docs/vercel ([`e18a148`](https://github.com/o2sh/onefetch/commit/e18a1487a33fd3145e01363579ceb714405e1110))
 * **[#808](https://github.com/o2sh/onefetch/issues/808)**
    - Bump @sveltejs/vite-plugin-svelte from 1.0.4 to 1.0.8 in /docs/vercel ([`8f08daa`](https://github.com/o2sh/onefetch/commit/8f08daa4f7bcda431edd13f77ce8bee80c03e837))
 * **[#809](https://github.com/o2sh/onefetch/issues/809)**
    - Bump vite from 3.1.4 to 3.1.6 in /docs/vercel ([`a9c07d9`](https://github.com/o2sh/onefetch/commit/a9c07d9fdd5716c39d3cd56345749390100ae31a))
 * **[#810](https://github.com/o2sh/onefetch/issues/810)**
    - Adding test coverage to src/info/info_field.rs ([`3cb479a`](https://github.com/o2sh/onefetch/commit/3cb479aa475d9d6ccdbca3190261d4c301bb7752))
 * **[#811](https://github.com/o2sh/onefetch/issues/811)**
    - adding language ABNF ([`9742b6d`](https://github.com/o2sh/onefetch/commit/9742b6dd9c21a2063df11302f017a8cae9182aab))
 * **[#812](https://github.com/o2sh/onefetch/issues/812)**
    - testing get_git_username using git-testtools for #700 ([`cbcd100`](https://github.com/o2sh/onefetch/commit/cbcd100e7014a1073b0ecf6c8d7d23f63c44c2d9))
 * **[#813](https://github.com/o2sh/onefetch/issues/813)**
    - Improve code coverage of src/info/repo/commits.rs ([`c88541d`](https://github.com/o2sh/onefetch/commit/c88541d3162e32119ba0805e312e17382fb1c4b5))
 * **[#814](https://github.com/o2sh/onefetch/issues/814)**
    - Improve code coverage of src/info/repo/contributors.rs ([`adee7d3`](https://github.com/o2sh/onefetch/commit/adee7d31b7baaebb7515ff09e87d99a8a34f6fa8))
 * **[#815](https://github.com/o2sh/onefetch/issues/815)**
    - Convert line endings to LF ([`7f37977`](https://github.com/o2sh/onefetch/commit/7f37977f40b4a4dc3119f4ce741dc952873c5f9c))
 * **[#818](https://github.com/o2sh/onefetch/issues/818)**
    - Bump time from 0.3.14 to 0.3.15 ([`4bf1575`](https://github.com/o2sh/onefetch/commit/4bf1575a6bc9c3171fbdec2322fb5eabaa7125d4))
 * **[#820](https://github.com/o2sh/onefetch/issues/820)**
    - Bump actions/stale from 6.0.0 to 6.0.1 ([`4c615ab`](https://github.com/o2sh/onefetch/commit/4c615ab2a4974fdc021f2496597730247f9369e8))
 * **[#821](https://github.com/o2sh/onefetch/issues/821)**
    - adding language ABAP #490 ([`00d3905`](https://github.com/o2sh/onefetch/commit/00d3905a3b63c362b58917e50b81582492ed2834))
 * **[#828](https://github.com/o2sh/onefetch/issues/828)**
    - Bump clap from 4.0.10 to 4.0.15 ([`c6777e8`](https://github.com/o2sh/onefetch/commit/c6777e8b0096932d7be592ac14c3b92a2a326245))
 * **[#829](https://github.com/o2sh/onefetch/issues/829)**
    - Add tests for `author.rs` ([`9cf8df4`](https://github.com/o2sh/onefetch/commit/9cf8df44196cfec2e5c8514f6616fcd5e8b4d77d))
 * **Uncategorized**
    - bump version ([`9ad12b8`](https://github.com/o2sh/onefetch/commit/9ad12b808822ad967e33b54496eb4ab115cea089))
    - cargo changelog --write ([`ad1f584`](https://github.com/o2sh/onefetch/commit/ad1f584cf6938adc4195b4504292a4fd13c51afa))
    - unit test author, assert with color ([`64a8148`](https://github.com/o2sh/onefetch/commit/64a8148d716b965c27b87dc9fe099975864e2c77))
    - use contains instead of exact match in title unit tests ([`cc7a59c`](https://github.com/o2sh/onefetch/commit/cc7a59cb9c8ede88ff574942832bde4736c93031))
    - update assembly logo ([`f47bfb0`](https://github.com/o2sh/onefetch/commit/f47bfb0ae4173a07651b94637edd71299eefee95))
    - update assembly logo ([`193af37`](https://github.com/o2sh/onefetch/commit/193af379d793afd0d174b00c803f02c4c41dd63b))
    - update assembly logo ([`a28a75b`](https://github.com/o2sh/onefetch/commit/a28a75be80bbdd29d16feefa0efbc232b6ccd85a))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`1499358`](https://github.com/o2sh/onefetch/commit/1499358d6cc0127cac0c3237cfc29769184e1901))
    - update assembly language ([`cbf7c7b`](https://github.com/o2sh/onefetch/commit/cbf7c7b0e32ed1d769f92b44cb0873a152595d2a))
    - Update language badge [Skip CI] ([`053d0b3`](https://github.com/o2sh/onefetch/commit/053d0b3f464962433a96119d9401f4acec1ff227))
    - update assembly logo ([`18e4fe5`](https://github.com/o2sh/onefetch/commit/18e4fe51fb0cd777b20a0573af5dd1e1a3060aa3))
    - update assembly logo ([`39e9090`](https://github.com/o2sh/onefetch/commit/39e9090b353d87a6a4eb03faae0cf5f080b988de))
    - fix typo ([`e52b22c`](https://github.com/o2sh/onefetch/commit/e52b22c12fb61da0159ba99307bda9e037b36841))
    - update assembly logo ([`96618c8`](https://github.com/o2sh/onefetch/commit/96618c8306ede2fa5edea5cb9013f3f1ff56598e))
    - better phrasing ([`79c5a33`](https://github.com/o2sh/onefetch/commit/79c5a3358b4902bc0148d17c44d1418702b3a7ce))
    - revert last change ([`d6d329d`](https://github.com/o2sh/onefetch/commit/d6d329dd693efb5efe942ca4ffe324b2fed5bfa5))
    - update abnp logo ([`9e4f6df`](https://github.com/o2sh/onefetch/commit/9e4f6df71f012d27d6efed2f85cc0f0e93b9d4fb))
    - Update language badge [Skip CI] ([`3d6445c`](https://github.com/o2sh/onefetch/commit/3d6445cd206da40babfa939bc3b770218e74b8dc))
    - update systemverilog logo ([`86d237b`](https://github.com/o2sh/onefetch/commit/86d237b673dc533479aad5ee44d2399ce5fb6caf))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`4c56fb9`](https://github.com/o2sh/onefetch/commit/4c56fb950de79d49ab820b65e26b76e16928e47f))
    - update typescript logo ([`40833db`](https://github.com/o2sh/onefetch/commit/40833dbf389308e6d95c463cce07b3ee732168c0))
    - update js and ts logo ([`5e35023`](https://github.com/o2sh/onefetch/commit/5e35023502f1761c3cc417956ffc45083fc24c50))
    - update systemverilog logo ([`23722dc`](https://github.com/o2sh/onefetch/commit/23722dcc53973f4010f8b83bbaa9c860e1e8d2cc))
    - update systemverilog logo ([`a4e1e9d`](https://github.com/o2sh/onefetch/commit/a4e1e9dfc614a406f682c56b7459b17c666d1d11))
    - update ruby logo ([`741cba3`](https://github.com/o2sh/onefetch/commit/741cba3e17544e4e0774e749c24940a48427bcbc))
    - update ruby logo ([`c559be4`](https://github.com/o2sh/onefetch/commit/c559be432996eb105ec1fe3da46bdde217d40d1a))
    - update ruby logo ([`ebf1aa7`](https://github.com/o2sh/onefetch/commit/ebf1aa78b05d24ff534f2f3cb8fe4ab85024fadf))
    - update ruby logo ([`c5629d9`](https://github.com/o2sh/onefetch/commit/c5629d9ac9be07e3d12f14dd1949bc7305b91484))
    - remove trailing spaces ([`ac5d5dd`](https://github.com/o2sh/onefetch/commit/ac5d5dd9ebbc444447490ed24ef940fdb1063362))
    - update tsx logo ([`a1a08aa`](https://github.com/o2sh/onefetch/commit/a1a08aad090a0da14990b7448bd97b73acb2ba18))
    - update jsx logo ([`420ffd4`](https://github.com/o2sh/onefetch/commit/420ffd4cac5edd78b6e75d442749b898b58c4c81))
    - update tsx logo ([`570ced5`](https://github.com/o2sh/onefetch/commit/570ced52378beecabef5685caf2c79060e979195))
    - update tsx logo ([`5fc75f7`](https://github.com/o2sh/onefetch/commit/5fc75f7a6cfdd72013dc7d21c22235fa70f7209c))
    - update tsx logo ([`2a76649`](https://github.com/o2sh/onefetch/commit/2a766493b2aba201848b26273d05184659a0d930))
    - update jsx logo ([`bbb53d0`](https://github.com/o2sh/onefetch/commit/bbb53d0bfe2f8be8798b46954610b0f46a0fba91))
    - update jsx logo ([`dea3d51`](https://github.com/o2sh/onefetch/commit/dea3d5130e5265a8354bc2c5aa64db7400fbce85))
    - remove trailing space ([`ce24f23`](https://github.com/o2sh/onefetch/commit/ce24f2391cdc93731d94fdeca618d1d546526c13))
    - remove trailing space ([`63eef57`](https://github.com/o2sh/onefetch/commit/63eef579207d7f67a7a916fc802ad85259051072))
    - add flags to README ([`0ec3f31`](https://github.com/o2sh/onefetch/commit/0ec3f3117c0e2ba81e50e36417cbb6a371d5ac37))
    - add anchor to chip ([`28b85a0`](https://github.com/o2sh/onefetch/commit/28b85a09546c6cda585b1b733a36a430e04d516a))
    - remove outtline on focus ([`7c5247c`](https://github.com/o2sh/onefetch/commit/7c5247c6bd1446f40c22999a9b7c96db2fe159c6))
    - remove anchor svg ([`1e39298`](https://github.com/o2sh/onefetch/commit/1e392980e46faf09e4c725346ae4b53cc6c266fb))
    - add anchor links for languages ([`3fec0c5`](https://github.com/o2sh/onefetch/commit/3fec0c57cb6f5bd0f8f4d2a8ea0c4d7ddb07474f))
    - match media size of sakura css to hide button ([`6a7a3d6`](https://github.com/o2sh/onefetch/commit/6a7a3d6b680225a423ebae4781bfbc6e35349010))
    - bigger button for back to top ([`3ba55d5`](https://github.com/o2sh/onefetch/commit/3ba55d54b262602fb0d16f9067f80cfeb2d6b0b8))
    - better style for back to top button ([`62fd9d5`](https://github.com/o2sh/onefetch/commit/62fd9d576622a5d4f878a25adcf127dfa383413a))
    - add back to top button ([`4925b3f`](https://github.com/o2sh/onefetch/commit/4925b3f20eb015a77b35db986a33ec33830f9db1))
    - Merge pull request #776 from o2sh/dependabot/cargo/git-repository-0.23.1 ([`94bfe2e`](https://github.com/o2sh/onefetch/commit/94bfe2ed05175af0cdc84b0eef100e4527f215a2))
    - Make sure the fastest available zlib configuration is used in `gitoxide`. ([`29197ab`](https://github.com/o2sh/onefetch/commit/29197ab94f5019d5f4d44b71731eb31e6baf82fa))
    - Bump git-repository from 0.22.1 to 0.23.1 ([`1b5b416`](https://github.com/o2sh/onefetch/commit/1b5b41614e78ddf7f09ddf1c0ad0cfa1e4ed80b9))
    - replace ansi_term with enable-ansi-support #747 ([`7cfbef6`](https://github.com/o2sh/onefetch/commit/7cfbef600bf39d9f3441e8cea88bd94c34e3d1c6))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`a011243`](https://github.com/o2sh/onefetch/commit/a011243412597ac46cd1ed67612c5c18524ffd38))
    - fix typo ([`335e34e`](https://github.com/o2sh/onefetch/commit/335e34ecafd5714caf143a54c5c0e56773e75816))
    - drop @vercel/node dependency #760 ([`e158ed4`](https://github.com/o2sh/onefetch/commit/e158ed4f908eadf8cbc787582f012a939aa4fb16))
    - remove serde feature from git-repository ([`42e6dc7`](https://github.com/o2sh/onefetch/commit/42e6dc76aa6c150b7c4150dd132e7ddb712d6bcd))
    - add codecov yml file ([`0832aad`](https://github.com/o2sh/onefetch/commit/0832aad141b8c779e9cd44757470a41d010867e2))
    - fix ci.yml ([`d2fe78f`](https://github.com/o2sh/onefetch/commit/d2fe78f00d6be419e4787414caf6a9235b771793))
    - fix ci.yml ([`bbc1e39`](https://github.com/o2sh/onefetch/commit/bbc1e399aedb5dee896369227a9db19c2c05f0b9))
    - update ci.yml ([`3626461`](https://github.com/o2sh/onefetch/commit/3626461ca28e2f74742b13d38ad43a582625ea0d))
    - update ci.yml ([`7b57858`](https://github.com/o2sh/onefetch/commit/7b57858bb5444dfac9cc80df4a60245d1233bf4b))
    - rustfmt ([`e0f122a`](https://github.com/o2sh/onefetch/commit/e0f122acc653ebdbc79732945f0352dd68776747))
    - rename git_info to title ([`2a3d995`](https://github.com/o2sh/onefetch/commit/2a3d995cdf24dca8859bfd6f791abe7057e71a3b))
    - cargo clippy ([`4afc056`](https://github.com/o2sh/onefetch/commit/4afc0567009b586899707f435bbdf55f875f4791))
    - add unit tests to author #700 ([`f8285fd`](https://github.com/o2sh/onefetch/commit/f8285fd1f90646fd618e43de154b09ba828e4b47))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`db2a2f6`](https://github.com/o2sh/onefetch/commit/db2a2f6350097b2d6b1d4a48dc8f434f9777b97a))
    - add unit tests to head_refs #700 ([`0b7111a`](https://github.com/o2sh/onefetch/commit/0b7111a0e5f7d96acbf2158041ba54a62f829294))
    - add unit test to ascii_art #700 ([`8fe3e42`](https://github.com/o2sh/onefetch/commit/8fe3e420d08998d5fac4d4d30de825d5ab1ab1d6))
    - add unit tests to deps/mod #700 ([`e348771`](https://github.com/o2sh/onefetch/commit/e348771ded234ca13dbde0b33d2074fc9e966979))
    - more unit tests for cli #700 ([`89d194f`](https://github.com/o2sh/onefetch/commit/89d194f3c9f9c9192fe70f37b01cedd68ebcd8aa))
    - add unit tests to cli #700 ([`f94f2e2`](https://github.com/o2sh/onefetch/commit/f94f2e2185271ffb838797b3dd8b13a93194f3d3))
    - setup unit tests for cli #700 ([`c8c70ba`](https://github.com/o2sh/onefetch/commit/c8c70bad744f8eb396045227a97b40ffa5b5b2e6))
    - add unit tests for package_manager #700 ([`2d57637`](https://github.com/o2sh/onefetch/commit/2d576372c5bd88e4481e745bdab94125efad32de))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`1ec516f`](https://github.com/o2sh/onefetch/commit/1ec516f3f591d8d99d07f64bec30dd958133afd2))
    - remove rs extension from tera template ([`c395be8`](https://github.com/o2sh/onefetch/commit/c395be81b9b8283c46800213ee535d6618a1453e))
    - apply bot regex pattern after mailmap ([`264c48f`](https://github.com/o2sh/onefetch/commit/264c48f471d2f4692ff2f247547de8938d3cd2db))
    - update clap derive declarations ([`80c863c`](https://github.com/o2sh/onefetch/commit/80c863c55be4493a115a246b6c8d7a5aa19c0e42))
    - update README.fr ([`bd532df`](https://github.com/o2sh/onefetch/commit/bd532df70849dfd282bd590591ae6afb91db85a6))
    - add unit test for info_field #700 ([`45d690a`](https://github.com/o2sh/onefetch/commit/45d690aa071804be96b664adab400f563f474cab))
    - impl from trait for InfoFieldOff ([`8e0450d`](https://github.com/o2sh/onefetch/commit/8e0450d8c109606fafc54d9675d4a4e475049b96))
    - add unit tests to ui/mod.rs #700 ([`d562e86`](https://github.com/o2sh/onefetch/commit/d562e86c32eec9bb8cd8fff8547b7fe99446a575))
    - add unit tests to text_colors #700 ([`26cb0f1`](https://github.com/o2sh/onefetch/commit/26cb0f132f2a33797d81da6fab80474031a31a02))
    - update unit tests for license.rs ([`f5d8fe7`](https://github.com/o2sh/onefetch/commit/f5d8fe72101a8e531c94cf468a61d09f6d841350))
    - add unit tests for license.rs #700 ([`edc27d8`](https://github.com/o2sh/onefetch/commit/edc27d84a925b3bcdaccf92a33c6e64145d1d025))
    - update license cache dataset ([`e434cdf`](https://github.com/o2sh/onefetch/commit/e434cdf064ed4710b2db3dd51f43047c16e8a81b))
    - fix links in READMEs ([`3823042`](https://github.com/o2sh/onefetch/commit/3823042da06421435067817c579bf6bdd49ef570))
    - improve text ([`ab03d46`](https://github.com/o2sh/onefetch/commit/ab03d46b9aaa5b0b675593279397528bf211455a))
    - center ascii logo in ascii preview ([`aed7e3d`](https://github.com/o2sh/onefetch/commit/aed7e3d8439842dba4b1b360156e391e23fa4bf7))
    - add link to ascii preview in readme ([`99c0dbb`](https://github.com/o2sh/onefetch/commit/99c0dbb3254497e82ef72a1542e3fc62e05580be))
    - redirect language support badge to ascii preview web page ([`d7f7099`](https://github.com/o2sh/onefetch/commit/d7f7099d5b768fc2db5e70f7ea34de5d2a7adefb))
    - default to bold like the binary ([`3d18f08`](https://github.com/o2sh/onefetch/commit/3d18f08a6ef746c36bd51eee8fac6ec56db7a2d3))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`8e2756a`](https://github.com/o2sh/onefetch/commit/8e2756a0e87f79a41050eb2bbb73ad49ec481ed3))
    - reduce gap in flex box ([`67a2609`](https://github.com/o2sh/onefetch/commit/67a2609f933c4d98c1eea2133070386945e1d7a2))
    - reduce gap between chip and language name ([`cfd8ee0`](https://github.com/o2sh/onefetch/commit/cfd8ee064c56bc7793fbb037bc600efc9ee485b2))
    - prevent vercel from commenting on every commit ([`2480783`](https://github.com/o2sh/onefetch/commit/2480783847c5b932541cb42efeea44648f0361e2))
    - fix path in vercel ci ([`6a0a5ee`](https://github.com/o2sh/onefetch/commit/6a0a5ee952e543b0c7b86d90b94f1f10b5ba8520))
    - move vercel into docs folder ([`9b74c17`](https://github.com/o2sh/onefetch/commit/9b74c172f8693cb318dedb6db859809eb6775870))
    - bump codecove-action ([`debd453`](https://github.com/o2sh/onefetch/commit/debd4538591d0e7a9f0d24286a267ceb157d9ce0))
    - bump codecove-action ([`4d6b967`](https://github.com/o2sh/onefetch/commit/4d6b96700f1eefe6a47ea239296d8e2fed56e14d))
    - remove codecov token ([`b758264`](https://github.com/o2sh/onefetch/commit/b758264f5d1ad7dc84f486d2949c78f376f8c239))
    - update codecov action ([`f268064`](https://github.com/o2sh/onefetch/commit/f268064a53dcdd605b133783edb902e752e65082))
    - update codecov action ([`aaa3fe4`](https://github.com/o2sh/onefetch/commit/aaa3fe4b20ddd8eb72ba598263dddc52efd64ba9))
    - update codecov action ([`07ccdb8`](https://github.com/o2sh/onefetch/commit/07ccdb8573e46721a56e47d72ee204edd6607c0d))
    - update codecov action ([`fbc8de3`](https://github.com/o2sh/onefetch/commit/fbc8de30cc0d46991cd9b555e6911f40e3358c44))
    - update codecov action ([`1e4a8a2`](https://github.com/o2sh/onefetch/commit/1e4a8a22a359094169871e547f4d718c5c578167))
    - move codecov.yml to root ([`223995f`](https://github.com/o2sh/onefetch/commit/223995f7f08b36d2ba01373854821762401a0144))
    - add codecov.yml ([`91ef43c`](https://github.com/o2sh/onefetch/commit/91ef43c288f5cb5a6fc973c47067802cd7b08003))
    - add codecov + tarpaulin in ci ([`4f492df`](https://github.com/o2sh/onefetch/commit/4f492dfe08272fdc1e713d4cdb48f93c067311b6))
    - remove empty line from ascii logo ([`ae4a1fe`](https://github.com/o2sh/onefetch/commit/ae4a1fea08c3f14f7e36fe2b77253d496e097cd4))
    - remove empty lines from ascii logos ([`e350a81`](https://github.com/o2sh/onefetch/commit/e350a814a7d3af304f6b3f039b2617bf60642c61))
    - remove licenses folder ([`86e6136`](https://github.com/o2sh/onefetch/commit/86e61365febb81a4d9d24627ceb63b49375af561))
    - remove tools folder ([`4130e70`](https://github.com/o2sh/onefetch/commit/4130e7065c9706a4afa5b751e9c3df262c919973))
    - update python ascii logo ([`e170c25`](https://github.com/o2sh/onefetch/commit/e170c25e74520a8add837e9686b10428a5c49a27))
    - update python ascii ([`eb99a4b`](https://github.com/o2sh/onefetch/commit/eb99a4bfa0d637e93e3cac3b65f51ca44d8ac75d))
    - update python ascii logo ([`ba0ef16`](https://github.com/o2sh/onefetch/commit/ba0ef16434c2c487d566ecb59cc255f797f097bf))
    - cargo clippy ([`70cea29`](https://github.com/o2sh/onefetch/commit/70cea29274b4af7a9c1cfca1371b23aacd1de2a4))
    - update README.fr.md ([`b8ad88a`](https://github.com/o2sh/onefetch/commit/b8ad88a39003c2fc67e0168af6c28c8c850d559c))
    - update README.fr.md ([`a8d2961`](https://github.com/o2sh/onefetch/commit/a8d296136d716672eb7d66a9728b9b806f5ee9f2))
    - Update README.fr.md ([`478c583`](https://github.com/o2sh/onefetch/commit/478c5830a67c9635ed0d609e28b102e1603eadb3))
    - replace default regex for --no-bots ([`de37245`](https://github.com/o2sh/onefetch/commit/de3724565223339f6fac6d292b077957dfacbd38))
    - cargo fmt ([`779d7a0`](https://github.com/o2sh/onefetch/commit/779d7a065c5da41b0cad002abef5b9f92a191cb4))
    - remove break lined ([`595f7a1`](https://github.com/o2sh/onefetch/commit/595f7a1045fad54d25f413ead9f4bf097bf0c55c))
    - fix --no-bots option ([`93b7d68`](https://github.com/o2sh/onefetch/commit/93b7d68fb9e12c3d8bc86f0809e654e949cd4ed6))
    - fix serialization of language enum ([`15968d9`](https://github.com/o2sh/onefetch/commit/15968d99823c746290ff51a311fcb927c9214137))
    - allow upper case acronyms for language enum ([`e3c5f5d`](https://github.com/o2sh/onefetch/commit/e3c5f5d0d84687edcd15fcc7d362581e93c7f6c5))
    - remove unused derive attributes ([`3a4a2e8`](https://github.com/o2sh/onefetch/commit/3a4a2e80e1c427d03a61851d6949d9a3cbe42887))
    - Bump git-repository from 0.18.1 to 0.19.0 ([`093c2ab`](https://github.com/o2sh/onefetch/commit/093c2ab702c920c68a067b0f5b1883575067d5b1))
    - clean prolog logo ([`f1eee48`](https://github.com/o2sh/onefetch/commit/f1eee48cf60e72a5bcd503737383b8201843e0b0))
    - tokei is now an exempted pr label ([`d43fa9a`](https://github.com/o2sh/onefetch/commit/d43fa9acbbc93cfee2e59faf3652e7893de55ffa))
    - tokei is now an exempted issue label ([`5e4d025`](https://github.com/o2sh/onefetch/commit/5e4d02552beea1a998239360fe61b8465437884a))
    - Bump git-repository from 0.17.0 to 0.18.1 ([`e3bbe63`](https://github.com/o2sh/onefetch/commit/e3bbe63599c673ca0c9c432469947f8910f5d7b9))
    - add code owner for ascii files ([`796ecf5`](https://github.com/o2sh/onefetch/commit/796ecf521c977e8ba9c42ac716d5fc19749b519b))
    - make publish package to winget a step instead of a job ([`442dbd1`](https://github.com/o2sh/onefetch/commit/442dbd177b6c53df91a3659b884474f296b1313e))
    - change trigger event from published to released on CD pipeline ([`0eb642a`](https://github.com/o2sh/onefetch/commit/0eb642a58b0d9000866cdfc7ec916f146a003fc3))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`2c752d8`](https://github.com/o2sh/onefetch/commit/2c752d890c45496038a31d4effcafc7811772faa))
    - add rust-version to Cargo.toml #670 ([`4e6c547`](https://github.com/o2sh/onefetch/commit/4e6c547716d8680b509207f20fe1b93f623e3661))
    - Update msrv badge [Skip CI] ([`3afeeea`](https://github.com/o2sh/onefetch/commit/3afeeeaa178e3954ced9a2e883dad80744311395))
    - Bump MSRV action #670 ([`e210288`](https://github.com/o2sh/onefetch/commit/e21028870100e0583c51f4fea8dba8f342478c2b))
    - run stale workflow once every week ([`7a8f9ef`](https://github.com/o2sh/onefetch/commit/7a8f9ef40bd796d11e6f4c26d9a6e1928f0e8b99))
    - Update msrv badge [Skip CI] ([`bfe4c29`](https://github.com/o2sh/onefetch/commit/bfe4c29e99395f1d5927dc5ecad1e2a069bb75f6))
    - make badge generation run every week ([`63d8494`](https://github.com/o2sh/onefetch/commit/63d849463994748b1ebe0e385b659ca27532ad5f))
    - make badge generation part of CI ([`c2d840f`](https://github.com/o2sh/onefetch/commit/c2d840f9a72fd5caaf4bd7635e43426d23356f52))
    - update Cargo.toml to match Cargo.lock ([`15fe2dd`](https://github.com/o2sh/onefetch/commit/15fe2ddf067d352bebd9f9e57d900699e97f2b72))
    - cargo update ([`8a52754`](https://github.com/o2sh/onefetch/commit/8a5275493f4c2786cc451046a13bfa37b673782b))
    - Simplify Repository instantiation ([`e394155`](https://github.com/o2sh/onefetch/commit/e39415524592f59d691dbeb9fe17c77465c16c0b))
    - switch to crates.io for git-repository and update dependencies ([`eb37c18`](https://github.com/o2sh/onefetch/commit/eb37c18562cd9f724324bd6a0898f7059d20a937))
    - Allow to open worktrees with gitoxide ([`d8e3c19`](https://github.com/o2sh/onefetch/commit/d8e3c19bac6a13b6ed1f0cd7f9e1ace6b07225db))
    - update to git-repository v0.17 for worktree support ([`76e2279`](https://github.com/o2sh/onefetch/commit/76e22797daf8065e78081032dfdbd60b0c4f3511))
    - remove .rustfmt.toml ([`633fe7c`](https://github.com/o2sh/onefetch/commit/633fe7c7fcc0610611ffeba7a4762a2afbd7ba47))
    - cargo update ([`638e12c`](https://github.com/o2sh/onefetch/commit/638e12ce356901457d5145a64d765de18e39cca2))
    - fix indentation ([`103e719`](https://github.com/o2sh/onefetch/commit/103e719ee56d03058fb885a64328559b8c72f2ba))
    - typo in issue template ([`aeb0734`](https://github.com/o2sh/onefetch/commit/aeb073426bc6203f35606cca47cc2557413265aa))
    - clean issue templates ([`4101e2f`](https://github.com/o2sh/onefetch/commit/4101e2f2c0f1901936384240fbc3f923391e56d4))
    - add feature request issue template ([`43ee6d6`](https://github.com/o2sh/onefetch/commit/43ee6d6bb1e8ee42846162b7c991dfb0e5d880d7))
    - rename issue template for bug report ([`4c6099c`](https://github.com/o2sh/onefetch/commit/4c6099c3c20dc1b5cf8f45be7f3611811704a3e4))
    - upgrade to gitoxide 0.16 ([`39e905d`](https://github.com/o2sh/onefetch/commit/39e905dea4ccea4b945cdc2d353421c5a800a526))
    - Bump askalono from 0.4.4 to 0.4.5 ([`2b9b8a3`](https://github.com/o2sh/onefetch/commit/2b9b8a3aa22feda3cc5bc2e15bf2312d5a034283))
    - Bump clap from 3.1.8 to 3.1.9 ([`511af52`](https://github.com/o2sh/onefetch/commit/511af52386dabcb585b679e763e7885b06d1493c))
    - Bump libc from 0.2.121 to 0.2.123 ([`181609f`](https://github.com/o2sh/onefetch/commit/181609fd83bfcd4ee7fc39e998dbfc8dad7ec4a2))
    - Bump toml from 0.5.8 to 0.5.9 ([`2d0fccf`](https://github.com/o2sh/onefetch/commit/2d0fccf6db0a52356ff77996693fb9908d9a6a4e))
    - Merge pull request #637 from Byron/changelog ([`f78fb67`](https://github.com/o2sh/onefetch/commit/f78fb675beca08b0a9ce63824a433e66829e211e))
    - Prepare changelog for upcoming 2.13 release. ([`9173cb2`](https://github.com/o2sh/onefetch/commit/9173cb2275adc104e0be96933d136ec8e6bcfe44))
    - update Cargo.log file which seemingly wasn't updated accidentally ([`542ae13`](https://github.com/o2sh/onefetch/commit/542ae13596d43c76d8aadac9ed710fd60fd2e77d))
    - Update CONTRIBUTING.md ([`d717a8f`](https://github.com/o2sh/onefetch/commit/d717a8ff680075433922823332097b5a9fc01755))
    - Result of `cargo changelog --write` ([`11ae25f`](https://github.com/o2sh/onefetch/commit/11ae25ff24cfe285815bc56d685bb83089104256))
    - Merge pull request #635 from Byron/gitoxide-for-traversal ([`cc6f332`](https://github.com/o2sh/onefetch/commit/cc6f332e086a952092037c11c7f19585ccca6b07))
    - git2 repository can now be owned by the `Repo` type ([`b6cd415`](https://github.com/o2sh/onefetch/commit/b6cd415d049b24348150e0e2088f2fdb5822e1cb))
    - completely separate `Commits` and `Repo` structure ([`7b34b0a`](https://github.com/o2sh/onefetch/commit/7b34b0aef20b1bc1dfd5de56596d3dca53e28d3e))
    - put all commit-traversal related initialization into own struct ([`d00ab45`](https://github.com/o2sh/onefetch/commit/d00ab45d3cab26e6c8394c2952d7704dd58b8245))
    - Make expect("msg") more informative to help users file an issue ([`c6d7cba`](https://github.com/o2sh/onefetch/commit/c6d7cbae0835a3ee21aa88c353f17d885e8164ee))
    - Improve Cargo.toml dependency declaration ([`4159442`](https://github.com/o2sh/onefetch/commit/41594424e79178db1bc2b26ecba4c284aacc11df))
    - switch gitoxide crates from git to crates.io ([`80f4710`](https://github.com/o2sh/onefetch/commit/80f4710104ef90a82c07de073964f1691d819fe5))
    - Remove additional deduplication of contributors by email ([`fb4d449`](https://github.com/o2sh/onefetch/commit/fb4d449134eff556aac1d14fb95ee9243b0e747e))
    - Update to use gitoxide's built-in shallow clone detection ([`b9b65c7`](https://github.com/o2sh/onefetch/commit/b9b65c7eea79b30582be58541554ad35948c909f))
    - Make clear that the commit count might be truncated due to shallow cloning ([`927815a`](https://github.com/o2sh/onefetch/commit/927815a5d46f9cc22261ba3796bdee515e139bb1))
    - Don't peel references - this would resolve remotes/origin/HEAD to …main… ([`eb753f9`](https://github.com/o2sh/onefetch/commit/eb753f9ca38adcc3e4ad21196bfdd0d1a52c8aa6))
    - Use email and name to identify contributors, similar to what git does ([`d3d20ed`](https://github.com/o2sh/onefetch/commit/d3d20eda4dd76720110f70b4992bb9c6483b1f12))
    - support for shallow clones ([`1a494a9`](https://github.com/o2sh/onefetch/commit/1a494a933856f4494749ecf66a58161c2a167535))
    - See if running onefetch on itself can reproduce the crashing on windows ([`82fbbb2`](https://github.com/o2sh/onefetch/commit/82fbbb25acd558ee85fe8fa56ab1291d3b7a68b1))
    - thanks clippy ([`3f94c51`](https://github.com/o2sh/onefetch/commit/3f94c5138809a67e7d77fd36795d506f896dfd8d))
    - Compute contributor identity using emails, lowercase, only ([`397b4ae`](https://github.com/o2sh/onefetch/commit/397b4aec6e125b6a85dbb0fd85f9b7ed978aebdb))
    - Improve unification of contributors by taking the lower-case email as identity ([`04ff547`](https://github.com/o2sh/onefetch/commit/04ff54742b8a58b61a6dd3a5c558187ea248bcb5))
    - Tune the object cache based on science™️ ([`a5ab965`](https://github.com/o2sh/onefetch/commit/a5ab965a24b97cb0fe33e7b68a8e9fdf8af40701))
    - Don't take risks when making assumptions about what branches point at ([`6817e48`](https://github.com/o2sh/onefetch/commit/6817e48be827d95efa1c3ad87cdc0bfaf3421cd4))
    - Do three things in parallel, don't wait for `Repo::new()` ([`633f0ce`](https://github.com/o2sh/onefetch/commit/633f0ce6cad0a664e764627b3f193aa59eb212a6))
    - gather language statistics in parallel to everything else ([`d178a5c`](https://github.com/o2sh/onefetch/commit/d178a5c721cf2d7154548ab92f90beaabe2bcda3))
    - get worktree status in parallel ([`5394f3c`](https://github.com/o2sh/onefetch/commit/5394f3c0bc21bb7bb3249f8723986d2542634ba6))
    - refactor ([`4085053`](https://github.com/o2sh/onefetch/commit/4085053f6a10658b98592008d66f4b44987bfdea))
    - refactor ([`4fc3334`](https://github.com/o2sh/onefetch/commit/4fc333417165e969b93147224803e6a110165294))
    - Assure short ids are not ambiguous ([`1942087`](https://github.com/o2sh/onefetch/commit/1942087b15d6aa4ef192724f71936cd085ac1340))
    - Collect branches at current head-commit with gitoxide ([`f61761d`](https://github.com/o2sh/onefetch/commit/f61761d990f76ecaba82557da3259c9cb6731af7))
    - get most recent version with gitoxide ([`2c6016e`](https://github.com/o2sh/onefetch/commit/2c6016eba41caccddea3ac02a18ea5079b92298f))
    - Retrieve all branches with `gitoxide` ([`615e071`](https://github.com/o2sh/onefetch/commit/615e0712ab0fba29ff309b3a01dfc9433cf1d988))
    - gitoxide for tags; Fix author name and email printing; avoid doing unnecessary work ([`c42a1ef`](https://github.com/o2sh/onefetch/commit/c42a1ef809bb122caf064741554fa9e7ca5482f3))
    - refactor ([`9b2774c`](https://github.com/o2sh/onefetch/commit/9b2774cf046523240af85787e8d0f2afcc874426))
    - Use `gitoxide` for calculating repo size ([`2a67bb4`](https://github.com/o2sh/onefetch/commit/2a67bb4c8b19fd829ac0d1e96f439b4c882ead03))
    - refactor ([`28deadf`](https://github.com/o2sh/onefetch/commit/28deadfe3422c238e7838f13bbcba718bb54f712))
    - fix commit count ([`65da5be`](https://github.com/o2sh/onefetch/commit/65da5be0af20c38fdc3994cae88d3218d2ae6f49))
    - Calculate authors on the fly as much as possible; don't store commits ([`0652bbe`](https://github.com/o2sh/onefetch/commit/0652bbeb488c51243aa3a2283dc0bc6994a4f37b))
    - no cloning for `Sig` and `Author` by using BString directly ([`954de84`](https://github.com/o2sh/onefetch/commit/954de842f1f91dff19898158ba593a1a39f3e3a5))
    - obtain all but author information on the fly ([`8df0d19`](https://github.com/o2sh/onefetch/commit/8df0d1966642cfd77783b8222be8537f1fa31b7c))
    - Update msrv badge [Skip CI] ([`a802f89`](https://github.com/o2sh/onefetch/commit/a802f89fc823c6c8cf2467c2b7908e1d78e8b755))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`dabbdb0`](https://github.com/o2sh/onefetch/commit/dabbdb0fddedf3bfc50fae4dd4598b310947c3e5))
    - fix msrv badge ci ([`410068d`](https://github.com/o2sh/onefetch/commit/410068dda8b9be75f8242b4c19163593e4805456))
    - Update language badge [Skip CI] ([`031bad3`](https://github.com/o2sh/onefetch/commit/031bad3da61413aa7c6089346413b94ee0341a28))
    - fix language badge ci ([`bd93480`](https://github.com/o2sh/onefetch/commit/bd93480387ea8ebf600b1bf01210969dbd10318e))
</details>

## v2.12.0 (2022-03-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 107 commits contributed to the release over the course of 125 calendar days.
 - 125 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 51 unique issues were worked on: [#545](https://github.com/o2sh/onefetch/issues/545), [#546](https://github.com/o2sh/onefetch/issues/546), [#547](https://github.com/o2sh/onefetch/issues/547), [#548](https://github.com/o2sh/onefetch/issues/548), [#549](https://github.com/o2sh/onefetch/issues/549), [#550](https://github.com/o2sh/onefetch/issues/550), [#551](https://github.com/o2sh/onefetch/issues/551), [#554](https://github.com/o2sh/onefetch/issues/554), [#555](https://github.com/o2sh/onefetch/issues/555), [#556](https://github.com/o2sh/onefetch/issues/556), [#557](https://github.com/o2sh/onefetch/issues/557), [#558](https://github.com/o2sh/onefetch/issues/558), [#559](https://github.com/o2sh/onefetch/issues/559), [#560](https://github.com/o2sh/onefetch/issues/560), [#561](https://github.com/o2sh/onefetch/issues/561), [#562](https://github.com/o2sh/onefetch/issues/562), [#563](https://github.com/o2sh/onefetch/issues/563), [#566](https://github.com/o2sh/onefetch/issues/566), [#568](https://github.com/o2sh/onefetch/issues/568), [#569](https://github.com/o2sh/onefetch/issues/569), [#573](https://github.com/o2sh/onefetch/issues/573), [#576](https://github.com/o2sh/onefetch/issues/576), [#580](https://github.com/o2sh/onefetch/issues/580), [#583](https://github.com/o2sh/onefetch/issues/583), [#585](https://github.com/o2sh/onefetch/issues/585), [#590](https://github.com/o2sh/onefetch/issues/590), [#591](https://github.com/o2sh/onefetch/issues/591), [#598](https://github.com/o2sh/onefetch/issues/598), [#599](https://github.com/o2sh/onefetch/issues/599), [#600](https://github.com/o2sh/onefetch/issues/600), [#601](https://github.com/o2sh/onefetch/issues/601), [#602](https://github.com/o2sh/onefetch/issues/602), [#604](https://github.com/o2sh/onefetch/issues/604), [#606](https://github.com/o2sh/onefetch/issues/606), [#607](https://github.com/o2sh/onefetch/issues/607), [#608](https://github.com/o2sh/onefetch/issues/608), [#609](https://github.com/o2sh/onefetch/issues/609), [#610](https://github.com/o2sh/onefetch/issues/610), [#613](https://github.com/o2sh/onefetch/issues/613), [#614](https://github.com/o2sh/onefetch/issues/614), [#616](https://github.com/o2sh/onefetch/issues/616), [#617](https://github.com/o2sh/onefetch/issues/617), [#620](https://github.com/o2sh/onefetch/issues/620), [#622](https://github.com/o2sh/onefetch/issues/622), [#623](https://github.com/o2sh/onefetch/issues/623), [#624](https://github.com/o2sh/onefetch/issues/624), [#625](https://github.com/o2sh/onefetch/issues/625), [#626](https://github.com/o2sh/onefetch/issues/626), [#630](https://github.com/o2sh/onefetch/issues/630), [#632](https://github.com/o2sh/onefetch/issues/632), [#634](https://github.com/o2sh/onefetch/issues/634)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#545](https://github.com/o2sh/onefetch/issues/545)**
    - Bump actions/cache from 2.1.6 to 2.1.7 ([`f50543c`](https://github.com/o2sh/onefetch/commit/f50543c1e9a403b082fca8b6dde384b73d5857c1))
 * **[#546](https://github.com/o2sh/onefetch/issues/546)**
    - Bump anyhow from 1.0.48 to 1.0.51 ([`301e27a`](https://github.com/o2sh/onefetch/commit/301e27ad675970458b7b51acc5eb8fe0e3639b44))
 * **[#547](https://github.com/o2sh/onefetch/issues/547)**
    - Bump serde_json from 1.0.71 to 1.0.72 ([`b18f88a`](https://github.com/o2sh/onefetch/commit/b18f88ac2c477b3e6df1712b182a6025ef4bc6fb))
 * **[#548](https://github.com/o2sh/onefetch/issues/548)**
    - Update MSRV badge and set timeout ([`0f5ad27`](https://github.com/o2sh/onefetch/commit/0f5ad27a66311e92c04597c3e8555e7d4aab9fa8))
 * **[#549](https://github.com/o2sh/onefetch/issues/549)**
    - Bump git2 from 0.13.24 to 0.13.25 ([`cd6d624`](https://github.com/o2sh/onefetch/commit/cd6d624f3dc9c0ec439e9aa4723a3d80570e5b11))
 * **[#550](https://github.com/o2sh/onefetch/issues/550)**
    - Bump clap from 2.33.3 to 2.34.0 ([`3a1368a`](https://github.com/o2sh/onefetch/commit/3a1368a971bcd3ddca55563ac5671cf958c52f36))
 * **[#551](https://github.com/o2sh/onefetch/issues/551)**
    - Bump libc from 0.2.108 to 0.2.109 ([`a0115c0`](https://github.com/o2sh/onefetch/commit/a0115c0c9a505c606a3a523248d6d39caf63c5ce))
 * **[#554](https://github.com/o2sh/onefetch/issues/554)**
    - Bump serde_yaml from 0.8.21 to 0.8.23 ([`006cc24`](https://github.com/o2sh/onefetch/commit/006cc244e41e696f933014a39218afd495ff7335))
 * **[#555](https://github.com/o2sh/onefetch/issues/555)**
    - Bump libc from 0.2.109 to 0.2.112 ([`9020c1d`](https://github.com/o2sh/onefetch/commit/9020c1d9cddd240ae40752bd616282b40d69ce90))
 * **[#556](https://github.com/o2sh/onefetch/issues/556)**
    - Bump serde from 1.0.130 to 1.0.131 ([`3cbb387`](https://github.com/o2sh/onefetch/commit/3cbb3875602335616a9744ee816e54bcd6a51ab0))
 * **[#557](https://github.com/o2sh/onefetch/issues/557)**
    - Bump serde_json from 1.0.72 to 1.0.73 ([`0d81bb1`](https://github.com/o2sh/onefetch/commit/0d81bb194757a29e7d4b89f2a447ac0a3ee4db3e))
 * **[#558](https://github.com/o2sh/onefetch/issues/558)**
    - Bump serde from 1.0.131 to 1.0.132 ([`0f9fb1f`](https://github.com/o2sh/onefetch/commit/0f9fb1f2c64937642455c71dacb90ff54e81feae))
 * **[#559](https://github.com/o2sh/onefetch/issues/559)**
    - Bump anyhow from 1.0.51 to 1.0.52 ([`1cfa2f2`](https://github.com/o2sh/onefetch/commit/1cfa2f2be61360ddcdd55afd166ac5cedebd994e))
 * **[#560](https://github.com/o2sh/onefetch/issues/560)**
    - Add Ren'Py language support ([`19e8d72`](https://github.com/o2sh/onefetch/commit/19e8d72944388f34bd540d48e4c30b81b00a2507))
 * **[#561](https://github.com/o2sh/onefetch/issues/561)**
    - Bump serde from 1.0.132 to 1.0.133 ([`0bd3645`](https://github.com/o2sh/onefetch/commit/0bd3645f3696b89eed88151de74296214617b4c8))
 * **[#562](https://github.com/o2sh/onefetch/issues/562)**
    - Bump clap from 2.34.0 to 3.0.1 ([`8cbb6d6`](https://github.com/o2sh/onefetch/commit/8cbb6d66fd4031991ad2106954649b5fef16ca3f))
 * **[#563](https://github.com/o2sh/onefetch/issues/563)**
    - Bump serde_json from 1.0.73 to 1.0.74 ([`fb2908e`](https://github.com/o2sh/onefetch/commit/fb2908e65fae6ff22036fda1ee7fa65d7a5f544a))
 * **[#566](https://github.com/o2sh/onefetch/issues/566)**
    - Bump serde_json from 1.0.74 to 1.0.75 ([`667cb5e`](https://github.com/o2sh/onefetch/commit/667cb5edb466153c05ddebc9f9128b139df459a8))
 * **[#568](https://github.com/o2sh/onefetch/issues/568)**
    - Bump serde from 1.0.133 to 1.0.135 ([`94545f7`](https://github.com/o2sh/onefetch/commit/94545f7849d09e5f238e243989c10f08918a33b0))
 * **[#569](https://github.com/o2sh/onefetch/issues/569)**
    - Bump anyhow from 1.0.52 to 1.0.53 ([`a88391b`](https://github.com/o2sh/onefetch/commit/a88391bec2539b754acd507449b1148055b120ee))
 * **[#573](https://github.com/o2sh/onefetch/issues/573)**
    - Bump serde from 1.0.135 to 1.0.136 ([`4449a7d`](https://github.com/o2sh/onefetch/commit/4449a7d1709aca4114a401e6897d5fc924c1d52b))
 * **[#576](https://github.com/o2sh/onefetch/issues/576)**
    - Bump time from 0.3.5 to 0.3.7 ([`9ecadfa`](https://github.com/o2sh/onefetch/commit/9ecadfa2c13aabc430a705fe972917412037d2ac))
 * **[#580](https://github.com/o2sh/onefetch/issues/580)**
    - Bump clap from 3.0.8 to 3.0.14 ([`15f97fa`](https://github.com/o2sh/onefetch/commit/15f97fa977160bf72aa420176d9688ba56173667))
 * **[#583](https://github.com/o2sh/onefetch/issues/583)**
    - Bump serde_json from 1.0.78 to 1.0.79 ([`d18ebc1`](https://github.com/o2sh/onefetch/commit/d18ebc15eb9b8e85d87524253f64cebc6aa6de2f))
 * **[#585](https://github.com/o2sh/onefetch/issues/585)**
    - add language bar ([`e9e20bb`](https://github.com/o2sh/onefetch/commit/e9e20bbc83d71ae74caf12e2cb7ae2099f991353))
 * **[#590](https://github.com/o2sh/onefetch/issues/590)**
    - Bump libc from 0.2.112 to 0.2.119 ([`63bae56`](https://github.com/o2sh/onefetch/commit/63bae56418f91da1f768accf79c4bbd9360e1b63))
 * **[#591](https://github.com/o2sh/onefetch/issues/591)**
    - Bump clap from 3.0.14 to 3.1.1 ([`8c6236e`](https://github.com/o2sh/onefetch/commit/8c6236edc422a85cbb9d8bfa18fd8428af342658))
 * **[#598](https://github.com/o2sh/onefetch/issues/598)**
    - Bump git2 from 0.13.25 to 0.14.1 ([`4f5c3a2`](https://github.com/o2sh/onefetch/commit/4f5c3a2b49866212d037e735a3e4cb46b8831307))
 * **[#599](https://github.com/o2sh/onefetch/issues/599)**
    - Bump anyhow from 1.0.53 to 1.0.55 ([`4a425ef`](https://github.com/o2sh/onefetch/commit/4a425ef8473dacba4b7a875ab96286fc5906411e))
 * **[#600](https://github.com/o2sh/onefetch/issues/600)**
    - Bump clap from 3.1.1 to 3.1.3 ([`dbf4ec2`](https://github.com/o2sh/onefetch/commit/dbf4ec29b7b71836789b99fc1341d3a319e41851))
 * **[#601](https://github.com/o2sh/onefetch/issues/601)**
    - Bump strum from 0.23.0 to 0.24.0 ([`7c35966`](https://github.com/o2sh/onefetch/commit/7c35966ded8f36243cf24c6fbd5bda2b4d2d9e52))
 * **[#602](https://github.com/o2sh/onefetch/issues/602)**
    - Add Ceylon language support ([`96f8d61`](https://github.com/o2sh/onefetch/commit/96f8d61f891e16ad75c49620c30b51245072b0c1))
 * **[#604](https://github.com/o2sh/onefetch/issues/604)**
    - Default to terminal foreground color for tilde, underline, colon and info ([`0ef5f58`](https://github.com/o2sh/onefetch/commit/0ef5f580619e66737423cf66e4d168901aa717a6))
 * **[#606](https://github.com/o2sh/onefetch/issues/606)**
    - Bump byte-unit from 4.0.13 to 4.0.14 ([`8af141d`](https://github.com/o2sh/onefetch/commit/8af141de798ee3ee326ef4cc7a452a4072c32839))
 * **[#607](https://github.com/o2sh/onefetch/issues/607)**
    - Bump clap from 3.1.3 to 3.1.6 ([`f989518`](https://github.com/o2sh/onefetch/commit/f9895186af36721eb61a37a8a31d4077687b25df))
 * **[#608](https://github.com/o2sh/onefetch/issues/608)**
    - Bump actions/checkout from 2 to 3 ([`822b41d`](https://github.com/o2sh/onefetch/commit/822b41d7ce33b08e50fbc591d2625059e84ec52d))
 * **[#609](https://github.com/o2sh/onefetch/issues/609)**
    - Add Wolfram language support ([`6d14f6e`](https://github.com/o2sh/onefetch/commit/6d14f6e5801cca390d676326a3f7bd61bfc8d959))
 * **[#610](https://github.com/o2sh/onefetch/issues/610)**
    - fix Lua logo so that it can be read even without color ([`8517ef6`](https://github.com/o2sh/onefetch/commit/8517ef6da53878ad7ed0e5afbe355c4483c426ce))
 * **[#613](https://github.com/o2sh/onefetch/issues/613)**
    - Make time test relative to current time ([`2c1f2f0`](https://github.com/o2sh/onefetch/commit/2c1f2f0b2c666f6ce94af0299f88048dd1d83484))
 * **[#614](https://github.com/o2sh/onefetch/issues/614)**
    - Bump git2 from 0.14.1 to 0.14.2 ([`884b11a`](https://github.com/o2sh/onefetch/commit/884b11a6e82529e931c31c313e4c4c2a1a1826ff))
 * **[#616](https://github.com/o2sh/onefetch/issues/616)**
    - Bump anyhow from 1.0.55 to 1.0.56 ([`1498ea3`](https://github.com/o2sh/onefetch/commit/1498ea380da54e3f3b7021ec1ebd065f9666ca77))
 * **[#617](https://github.com/o2sh/onefetch/issues/617)**
    - Bump regex from 1.5.4 to 1.5.5 ([`ea98719`](https://github.com/o2sh/onefetch/commit/ea987195d526f41c09cc96a00dd925995430edde))
 * **[#620](https://github.com/o2sh/onefetch/issues/620)**
    - Replace colored with owo-colors ([`1773abe`](https://github.com/o2sh/onefetch/commit/1773abe613dad021433667e00809a85e112595bf))
 * **[#622](https://github.com/o2sh/onefetch/issues/622)**
    - Bump libc from 0.2.119 to 0.2.121 ([`0b91af0`](https://github.com/o2sh/onefetch/commit/0b91af07a97e072b88163c487266bf468a32128e))
 * **[#623](https://github.com/o2sh/onefetch/issues/623)**
    - Bump actions/cache from 2.1.7 to 3 ([`3d914aa`](https://github.com/o2sh/onefetch/commit/3d914aa1c756a4c00f851f5bdb4f028f4980672c))
 * **[#624](https://github.com/o2sh/onefetch/issues/624)**
    - Bump owo-colors from 3.2.0 to 3.3.0 ([`d63f791`](https://github.com/o2sh/onefetch/commit/d63f791546ea7e8142406903b7364b05c38d14d1))
 * **[#625](https://github.com/o2sh/onefetch/issues/625)**
    - Remap White ANSI color to Default ([`7b89eff`](https://github.com/o2sh/onefetch/commit/7b89eff51d4fa282b4d7966b3a65bfb58c1b9327))
 * **[#626](https://github.com/o2sh/onefetch/issues/626)**
    - Fix "other" language block hidden in background ([`dd2b6c5`](https://github.com/o2sh/onefetch/commit/dd2b6c5e3e90f6d3b383c385ea8bf7581d0d8fff))
 * **[#630](https://github.com/o2sh/onefetch/issues/630)**
    - Match circle color with github linguist ([`03139c9`](https://github.com/o2sh/onefetch/commit/03139c90b5000c1ff25bc83ff89798ddc5b45f5c))
 * **[#632](https://github.com/o2sh/onefetch/issues/632)**
    - Add language support for VHDL ([`1018d9a`](https://github.com/o2sh/onefetch/commit/1018d9a907d9325cc000ed9430f306b9ac259769))
 * **[#634](https://github.com/o2sh/onefetch/issues/634)**
    - Bump paste from 1.0.6 to 1.0.7 ([`b51ddc3`](https://github.com/o2sh/onefetch/commit/b51ddc3a3aa41058591deb40c65a210a8f061b45))
 * **Uncategorized**
    - bump version ([`913ad45`](https://github.com/o2sh/onefetch/commit/913ad45ed95e80722b974795a460d6161b6489ae))
    - update CONTRIBUTING.md ([`9ab7e79`](https://github.com/o2sh/onefetch/commit/9ab7e7942db02338cfcc2e87955e41f558c09f2b))
    - update CONTRIBUTING.md ([`e36a78f`](https://github.com/o2sh/onefetch/commit/e36a78f725813895acd061b62638406e1814d91b))
    - update assets ([`4cfc84e`](https://github.com/o2sh/onefetch/commit/4cfc84eee6b1a762c969fc9871f2b2cff1b0d7ac))
    - update assets ([`99e088e`](https://github.com/o2sh/onefetch/commit/99e088e10b9129000aed5126297305f3ccda248a))
    - update assets ([`4baf7db`](https://github.com/o2sh/onefetch/commit/4baf7db5cb7c2a0d59422984d3dd5f01c74d77bc))
    - Use gitoxide in all methods related to commits ([`e3b29b0`](https://github.com/o2sh/onefetch/commit/e3b29b0aee7391baa39f613aa8798b0c137e386a))
    - get_logs() with gitoxide ([`1bb11e6`](https://github.com/o2sh/onefetch/commit/1bb11e649e2c09d0601409274094081ce3a26803))
    - Add gitoxide as dependency ([`5d7b5c8`](https://github.com/o2sh/onefetch/commit/5d7b5c8258b22fb71adf3f142e794949b703643c))
    - make info.text_colors private ([`17047db`](https://github.com/o2sh/onefetch/commit/17047db0089b227b5a1ca22634f8010d640e6530))
    - rename function ([`b228e46`](https://github.com/o2sh/onefetch/commit/b228e4615e1457d55a60c2d97a9612f48c8ac5dc))
    - refacto styling of info lines ([`452ad6e`](https://github.com/o2sh/onefetch/commit/452ad6eb55997a0b883809ee18b4ba4bd013ba73))
    - remove blank space ([`06dbe1a`](https://github.com/o2sh/onefetch/commit/06dbe1a32b3754fb8fbe679c756927e542fba47d))
    - rename function in ascii_art ([`6fbcfb5`](https://github.com/o2sh/onefetch/commit/6fbcfb58fc1fa8bafa1581b5155013099c56e086))
    - improve readability get_language_field ([`0f6765c`](https://github.com/o2sh/onefetch/commit/0f6765c9d29635c27f6733afb2df60d2247410a5))
    - Update CONTRIBUTING.md ([`08c94c9`](https://github.com/o2sh/onefetch/commit/08c94c94792f43060c48f2a37f95e72e9abca0a4))
    - map HOME to SNAP_REAL_HOME, #588 ([`923e486`](https://github.com/o2sh/onefetch/commit/923e486a52a8c7441118a3e1ebeb82380fbf2c86))
    - use SNAP_REAL_HOME instead of HOME, #588 ([`9aee3c4`](https://github.com/o2sh/onefetch/commit/9aee3c478b2c5d542593814716f0bf80508b6b72))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`9df7631`](https://github.com/o2sh/onefetch/commit/9df7631b548d6366ebc971a7093276285ee8aae9))
    - rename snapcraft plugs, #588 ([`aa60fdf`](https://github.com/o2sh/onefetch/commit/aa60fdfe19a312a2c79f6c3990424096c01c9046))
    - cargo clippy ([`51e65ac`](https://github.com/o2sh/onefetch/commit/51e65ac1344c0752274ff8ed94bba0133995cacd))
    - fix lua ascii logo ([`01d8211`](https://github.com/o2sh/onefetch/commit/01d821119b90e2aa69cc181c880ba93e7c57a1ab))
    - bump image dependency ([`29e1056`](https://github.com/o2sh/onefetch/commit/29e1056ad78a7e086332d170816263d7c3a06a10))
    - add snapcraft plugs to read gitconfig files from  HOME, #588 ([`d1eb649`](https://github.com/o2sh/onefetch/commit/d1eb6494158f7d24eff8b054278a7f0deae0fd51))
    - add snap interface to allow read access to /etc/gitconfig #588 ([`125c610`](https://github.com/o2sh/onefetch/commit/125c610dd438dbd5abaf91d4933235a615aba766))
    - update error message ([`bc09db5`](https://github.com/o2sh/onefetch/commit/bc09db57a03bbe8c4947f52b870a54dc2e707ffe))
    - skip first condition in is_valid check, #588 ([`43511dc`](https://github.com/o2sh/onefetch/commit/43511dc2b022c79bd73a12cf9540892fb2c6b3f7))
    - updata assets ([`af3967f`](https://github.com/o2sh/onefetch/commit/af3967fde055ab6d66c2e690b9cd4b83a9e1c68f))
    - update c# ascii logo ([`0c20aff`](https://github.com/o2sh/onefetch/commit/0c20aff77809b5a992453d6c10a0e87bd2db893d))
    - update c ascii logo ([`412a662`](https://github.com/o2sh/onefetch/commit/412a66240f8c2218cf4f20e58565c58a103254d0))
    - update assets ([`23c910b`](https://github.com/o2sh/onefetch/commit/23c910b199c08331cec326077d738e8fa23fa8d5))
    - update assets ([`a818450`](https://github.com/o2sh/onefetch/commit/a8184507fd4ba6c5afca5dcf07bbc8a507bf2a08))
    - update assets ([`3de0a37`](https://github.com/o2sh/onefetch/commit/3de0a37446b844a3020b8bf97f6f6bafe8d2b14d))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`f00c8b3`](https://github.com/o2sh/onefetch/commit/f00c8b308c7bb2a616821fc31d8fc34a6c705f6d))
    - update assets ([`aecdeff`](https://github.com/o2sh/onefetch/commit/aecdeff9efaf9064672cb888a4668dcf37390ef9))
    - update assets ([`9b87fd6`](https://github.com/o2sh/onefetch/commit/9b87fd61deb4c0034bb7f91e5a4b6bb198139cce))
    - update c++ ascii logo ([`e19b106`](https://github.com/o2sh/onefetch/commit/e19b1062ce0a2e969ebe473d2fd94fe983006f15))
    - update assets ([`9a446cd`](https://github.com/o2sh/onefetch/commit/9a446cd4c5ffdb3e34a1d5a69eb962d33a501b68))
    - update assets ([`bbb98f2`](https://github.com/o2sh/onefetch/commit/bbb98f2dca3936468f84f907f38d8340e2ce87ee))
    - update assets ([`4e817af`](https://github.com/o2sh/onefetch/commit/4e817aff0e133fbd997127558292a91ee7054a3f))
    - update assets ([`a924808`](https://github.com/o2sh/onefetch/commit/a924808840b345f8ca3927f83e81a7eda1850f10))
    - update assets ([`3536a03`](https://github.com/o2sh/onefetch/commit/3536a0317cb5e5a61a6e376ecac36bd22a2072a7))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`9770026`](https://github.com/o2sh/onefetch/commit/9770026c862b1e888e5fe19236b2adb8ca80ea1f))
    - fix #572, update color palette for svelte logo ([`ec8b992`](https://github.com/o2sh/onefetch/commit/ec8b99219a5ef31c372a0a5b3b4c840de213d7c1))
    - Merge pull request #571 from o2sh/dependabot/cargo/serde_json-1.0.78 ([`d80efdf`](https://github.com/o2sh/onefetch/commit/d80efdfa129eab3ae0e97c316d52a2191459b29b))
    - Bump serde_json from 1.0.75 to 1.0.78 ([`0264c0d`](https://github.com/o2sh/onefetch/commit/0264c0de1aa9ea61ae4ed3e5299ff76a883f7a36))
    - Merge pull request #565 from o2sh/dependabot/cargo/clap-3.0.8 ([`4144f23`](https://github.com/o2sh/onefetch/commit/4144f23de7790a1794d70224b826deb1032ac111))
    - Bump clap from 3.0.1 to 3.0.8 ([`6029c42`](https://github.com/o2sh/onefetch/commit/6029c42abaeb35f91efec21818042ae68b4e672d))
    - slight retouching renpy ascii logo ([`5c57f7c`](https://github.com/o2sh/onefetch/commit/5c57f7ca4506dbbe9be8a38da741ef052ec270e1))
    - Update language badge [Skip CI] ([`c60e181`](https://github.com/o2sh/onefetch/commit/c60e1810eb6a58933eb4635739de8d8667b1a641))
    - fix typo language-badge GHA ([`64c40b3`](https://github.com/o2sh/onefetch/commit/64c40b334e4d80c04c6a5d80198dd0b3df0877aa))
    - Update language badge [Skip CI] ([`67b86f1`](https://github.com/o2sh/onefetch/commit/67b86f1a59022e6732650e28945e66313195c093))
    - fix language-badge GHA with ::set-ouput ([`e18b3ca`](https://github.com/o2sh/onefetch/commit/e18b3cad4059c78f6a4eda7565268e2ba537c43c))
    - manually update Language Badge ([`f236104`](https://github.com/o2sh/onefetch/commit/f236104239ea1a5874f842a39b7e20b064c2e7b4))
    - add 'Install Rust' step in language-badge GH action ([`c3a61f9`](https://github.com/o2sh/onefetch/commit/c3a61f9c4865903ad056f8e6442cb0b4d1dc852a))
    - Update language badge [Skip CI] ([`4db941f`](https://github.com/o2sh/onefetch/commit/4db941ffaa511ec5122723098275522676f0f3b1))
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

 - 123 commits contributed to the release over the course of 142 calendar days.
 - 142 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 55 unique issues were worked on: [#460](https://github.com/o2sh/onefetch/issues/460), [#461](https://github.com/o2sh/onefetch/issues/461), [#463](https://github.com/o2sh/onefetch/issues/463), [#465](https://github.com/o2sh/onefetch/issues/465), [#466](https://github.com/o2sh/onefetch/issues/466), [#469](https://github.com/o2sh/onefetch/issues/469), [#470](https://github.com/o2sh/onefetch/issues/470), [#473](https://github.com/o2sh/onefetch/issues/473), [#474](https://github.com/o2sh/onefetch/issues/474), [#475](https://github.com/o2sh/onefetch/issues/475), [#476](https://github.com/o2sh/onefetch/issues/476), [#477](https://github.com/o2sh/onefetch/issues/477), [#478](https://github.com/o2sh/onefetch/issues/478), [#479](https://github.com/o2sh/onefetch/issues/479), [#480](https://github.com/o2sh/onefetch/issues/480), [#481](https://github.com/o2sh/onefetch/issues/481), [#482](https://github.com/o2sh/onefetch/issues/482), [#483](https://github.com/o2sh/onefetch/issues/483), [#484](https://github.com/o2sh/onefetch/issues/484), [#485](https://github.com/o2sh/onefetch/issues/485), [#486](https://github.com/o2sh/onefetch/issues/486), [#487](https://github.com/o2sh/onefetch/issues/487), [#488](https://github.com/o2sh/onefetch/issues/488), [#489](https://github.com/o2sh/onefetch/issues/489), [#491](https://github.com/o2sh/onefetch/issues/491), [#492](https://github.com/o2sh/onefetch/issues/492), [#494](https://github.com/o2sh/onefetch/issues/494), [#495](https://github.com/o2sh/onefetch/issues/495), [#496](https://github.com/o2sh/onefetch/issues/496), [#497](https://github.com/o2sh/onefetch/issues/497), [#498](https://github.com/o2sh/onefetch/issues/498), [#499](https://github.com/o2sh/onefetch/issues/499), [#500](https://github.com/o2sh/onefetch/issues/500), [#501](https://github.com/o2sh/onefetch/issues/501), [#502](https://github.com/o2sh/onefetch/issues/502), [#503](https://github.com/o2sh/onefetch/issues/503), [#505](https://github.com/o2sh/onefetch/issues/505), [#506](https://github.com/o2sh/onefetch/issues/506), [#507](https://github.com/o2sh/onefetch/issues/507), [#508](https://github.com/o2sh/onefetch/issues/508), [#509](https://github.com/o2sh/onefetch/issues/509), [#511](https://github.com/o2sh/onefetch/issues/511), [#513](https://github.com/o2sh/onefetch/issues/513), [#514](https://github.com/o2sh/onefetch/issues/514), [#518](https://github.com/o2sh/onefetch/issues/518), [#519](https://github.com/o2sh/onefetch/issues/519), [#527](https://github.com/o2sh/onefetch/issues/527), [#528](https://github.com/o2sh/onefetch/issues/528), [#529](https://github.com/o2sh/onefetch/issues/529), [#530](https://github.com/o2sh/onefetch/issues/530), [#531](https://github.com/o2sh/onefetch/issues/531), [#532](https://github.com/o2sh/onefetch/issues/532), [#537](https://github.com/o2sh/onefetch/issues/537), [#541](https://github.com/o2sh/onefetch/issues/541), [#542](https://github.com/o2sh/onefetch/issues/542)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#460](https://github.com/o2sh/onefetch/issues/460)**
    - add Persian language (file:docs/README.fa.md) 🇮🇷 ([`6f606b8`](https://github.com/o2sh/onefetch/commit/6f606b80cfa98e7c82b4fb719df7c3336de3b970))
 * **[#461](https://github.com/o2sh/onefetch/issues/461)**
    - Bump libc from 0.2.97 to 0.2.98 ([`09c2f3c`](https://github.com/o2sh/onefetch/commit/09c2f3cca7a3e4b76818805d3e4e231080b39aca))
 * **[#463](https://github.com/o2sh/onefetch/issues/463)**
    - Add Chinese README ([`764aaf0`](https://github.com/o2sh/onefetch/commit/764aaf0956ae7f511eeeb81c764ad9f6c4673709))
 * **[#465](https://github.com/o2sh/onefetch/issues/465)**
    - Count hidden files and directories ([`8a9cb2d`](https://github.com/o2sh/onefetch/commit/8a9cb2dede5efbdf4b2ba629c593caea3cdd6d06))
 * **[#466](https://github.com/o2sh/onefetch/issues/466)**
    - Better error handling ([`9466f26`](https://github.com/o2sh/onefetch/commit/9466f26925efbc11707c7bcf08d69bfa8b89fbf7))
 * **[#469](https://github.com/o2sh/onefetch/issues/469)**
    - Bump serde_json from 1.0.64 to 1.0.65 ([`7d1bd47`](https://github.com/o2sh/onefetch/commit/7d1bd475a8a66698515a99169b52d5fecd8ffbf0))
 * **[#470](https://github.com/o2sh/onefetch/issues/470)**
    - Bump serde_json from 1.0.65 to 1.0.66 ([`90f1281`](https://github.com/o2sh/onefetch/commit/90f1281b17f03025c9476ca1b1f4b2ce08d0c594))
 * **[#473](https://github.com/o2sh/onefetch/issues/473)**
    - Bump git2 from 0.13.20 to 0.13.21 ([`42d5df2`](https://github.com/o2sh/onefetch/commit/42d5df25f640a38c3ae26c52a57e40bca7f3f26e))
 * **[#474](https://github.com/o2sh/onefetch/issues/474)**
    - Bump anyhow from 1.0.42 to 1.0.43 ([`74b79dd`](https://github.com/o2sh/onefetch/commit/74b79dda31c109eab6ab09eab66f4f0faa05f57c))
 * **[#475](https://github.com/o2sh/onefetch/issues/475)**
    - Bump libc from 0.2.98 to 0.2.100 ([`a36c588`](https://github.com/o2sh/onefetch/commit/a36c588fa48561679a5c253b5aa87c83578ed5dd))
 * **[#476](https://github.com/o2sh/onefetch/issues/476)**
    - Bump serde from 1.0.126 to 1.0.128 ([`e9d2c77`](https://github.com/o2sh/onefetch/commit/e9d2c775e95856ab51ec608306a0a34e795f909a))
 * **[#477](https://github.com/o2sh/onefetch/issues/477)**
    - Bump serde_yaml from 0.8.17 to 0.8.19 ([`4dc28ac`](https://github.com/o2sh/onefetch/commit/4dc28ac1423ad32aaac784a4e971449096fd70dd))
 * **[#478](https://github.com/o2sh/onefetch/issues/478)**
    - Bump serde from 1.0.128 to 1.0.129 ([`5ee8e51`](https://github.com/o2sh/onefetch/commit/5ee8e51cc01e02d70a68c0d3f4ad7dfb431b204a))
 * **[#479](https://github.com/o2sh/onefetch/issues/479)**
    - Fix typo ([`e8b3da4`](https://github.com/o2sh/onefetch/commit/e8b3da4534909b93cf1a40eb83392193e13ffa34))
 * **[#480](https://github.com/o2sh/onefetch/issues/480)**
    - Bump libc from 0.2.100 to 0.2.101 ([`3249d15`](https://github.com/o2sh/onefetch/commit/3249d15e218190537d0baea89699493377e60a13))
 * **[#481](https://github.com/o2sh/onefetch/issues/481)**
    - Bump serde_yaml from 0.8.19 to 0.8.20 ([`1ad83b1`](https://github.com/o2sh/onefetch/commit/1ad83b1dfaf61be5a2f70b22d97f250cc6d883ae))
 * **[#482](https://github.com/o2sh/onefetch/issues/482)**
    - Bump serde_json from 1.0.66 to 1.0.67 ([`5e58c3c`](https://github.com/o2sh/onefetch/commit/5e58c3cccab71db7a3d1fdea956f75c867d0561d))
 * **[#483](https://github.com/o2sh/onefetch/issues/483)**
    - Bump serde from 1.0.129 to 1.0.130 ([`ea01f69`](https://github.com/o2sh/onefetch/commit/ea01f69f562e33c5c7a66a118d8ab728b7a82775))
 * **[#484](https://github.com/o2sh/onefetch/issues/484)**
    - Add Russian README ([`311b4ba`](https://github.com/o2sh/onefetch/commit/311b4ba7910cd6c01f199c8f68aa091832b32d62))
 * **[#485](https://github.com/o2sh/onefetch/issues/485)**
    - Bump git2 from 0.13.21 to 0.13.22 ([`7b2aec2`](https://github.com/o2sh/onefetch/commit/7b2aec21b74d27639eefe89e9491c9a541aca496))
 * **[#486](https://github.com/o2sh/onefetch/issues/486)**
    - Bump serde_yaml from 0.8.20 to 0.8.21 ([`0fe32d5`](https://github.com/o2sh/onefetch/commit/0fe32d5b5c00cf478625c42cc70ec8978f4c308b))
 * **[#487](https://github.com/o2sh/onefetch/issues/487)**
    - Bump anyhow from 1.0.43 to 1.0.44 ([`a6fdaeb`](https://github.com/o2sh/onefetch/commit/a6fdaeb7bf7814505d8459f1cdd9b322bebd3ad1))
 * **[#488](https://github.com/o2sh/onefetch/issues/488)**
    - Bump serde_json from 1.0.67 to 1.0.68 ([`4449fb6`](https://github.com/o2sh/onefetch/commit/4449fb694ed38d15a35191be3dd4272aa25ecfe4))
 * **[#489](https://github.com/o2sh/onefetch/issues/489)**
    - Bump libc from 0.2.101 to 0.2.102 ([`c3af87a`](https://github.com/o2sh/onefetch/commit/c3af87a22d11d57b9c015e5dddc5a061a08b9b0c))
 * **[#491](https://github.com/o2sh/onefetch/issues/491)**
    - add sql support ([`a3455d4`](https://github.com/o2sh/onefetch/commit/a3455d494a48d0d9dc986deda7855c9cb1788e8b))
 * **[#492](https://github.com/o2sh/onefetch/issues/492)**
    - Bump libc from 0.2.102 to 0.2.103 ([`b9380ee`](https://github.com/o2sh/onefetch/commit/b9380ee2da673dfa2d0b53d1d16460e1184685a4))
 * **[#494](https://github.com/o2sh/onefetch/issues/494)**
    - Automate "languages supported" badge ([`49ab482`](https://github.com/o2sh/onefetch/commit/49ab482240064f30f426cd1a971de13915f8e3ae))
 * **[#495](https://github.com/o2sh/onefetch/issues/495)**
    - Add WebAssembly language support ([`81e3a03`](https://github.com/o2sh/onefetch/commit/81e3a033ff9f9ef23787a128e2f599123b5efb9b))
 * **[#496](https://github.com/o2sh/onefetch/issues/496)**
    - Add TOML language support ([`484b9e4`](https://github.com/o2sh/onefetch/commit/484b9e44577cfb5c984cde565df1020dd6d9aca8))
 * **[#497](https://github.com/o2sh/onefetch/issues/497)**
    - removed json dependency ([`94a100a`](https://github.com/o2sh/onefetch/commit/94a100aa66da14cf76a6e5a573a2588a5caf2d54))
 * **[#498](https://github.com/o2sh/onefetch/issues/498)**
    - Add Jsonnet language support ([`f77f376`](https://github.com/o2sh/onefetch/commit/f77f376be571f7c831e6a5ce9337289c17af4548))
 * **[#499](https://github.com/o2sh/onefetch/issues/499)**
    - Add YAML language support ([`de015f0`](https://github.com/o2sh/onefetch/commit/de015f03c097fdedf6ee88ec6277430ad0a0ead1))
 * **[#500](https://github.com/o2sh/onefetch/issues/500)**
    - Bump git2 from 0.13.22 to 0.13.23 ([`cd95811`](https://github.com/o2sh/onefetch/commit/cd95811b4a5b8d46d2bcf4a5b3a37133cdd828b7))
 * **[#501](https://github.com/o2sh/onefetch/issues/501)**
    - Add Solidity language support ([`01e06df`](https://github.com/o2sh/onefetch/commit/01e06df1e71b280191aa1980171f927c70bd2a00))
 * **[#502](https://github.com/o2sh/onefetch/issues/502)**
    - Bump strum from 0.21.0 to 0.22.0 ([`e6ae17a`](https://github.com/o2sh/onefetch/commit/e6ae17a8b25744845513a7b44995fca48706c074))
 * **[#503](https://github.com/o2sh/onefetch/issues/503)**
    - Fix contributing link in cn readme ([`bb019ec`](https://github.com/o2sh/onefetch/commit/bb019ec4150f02bf6266dac6652ea20e56a003cf))
 * **[#505](https://github.com/o2sh/onefetch/issues/505)**
    - Add support for json ([`35796b7`](https://github.com/o2sh/onefetch/commit/35796b71b7272667774fb4e5914920103cb33438))
 * **[#506](https://github.com/o2sh/onefetch/issues/506)**
    - Add SASS language support ([`7aa883f`](https://github.com/o2sh/onefetch/commit/7aa883f99154318c4b715f03c41818b9fa0281cc))
 * **[#507](https://github.com/o2sh/onefetch/issues/507)**
    - Add LLVM language support ([`07e8929`](https://github.com/o2sh/onefetch/commit/07e8929ca21bc6a51bb419d3bfa104e8abde1836))
 * **[#508](https://github.com/o2sh/onefetch/issues/508)**
    - support AutoHotKey ([`be7787e`](https://github.com/o2sh/onefetch/commit/be7787ea25e4dfeb7dae813d37dfbc6033041640))
 * **[#509](https://github.com/o2sh/onefetch/issues/509)**
    - Change Ruby logo to red ([`f11777c`](https://github.com/o2sh/onefetch/commit/f11777cbdf417665f28b91167b0f248876a57f0f))
 * **[#511](https://github.com/o2sh/onefetch/issues/511)**
    - Bump spenserblack/actions-set-output from 0.1 to 1 ([`163d2e8`](https://github.com/o2sh/onefetch/commit/163d2e8e7a65ae7ddfa840330fc662b486488358))
 * **[#513](https://github.com/o2sh/onefetch/issues/513)**
    - add language type ([`b98a26c`](https://github.com/o2sh/onefetch/commit/b98a26c684f02e08dcc66f948315320c7bd6f8b8))
 * **[#514](https://github.com/o2sh/onefetch/issues/514)**
    - Add Coq support ([`5074a16`](https://github.com/o2sh/onefetch/commit/5074a16f693212f212a04de1db398c94d917ae77))
 * **[#518](https://github.com/o2sh/onefetch/issues/518)**
    - Bump byte-unit from 4.0.12 to 4.0.13 ([`987b206`](https://github.com/o2sh/onefetch/commit/987b2063603cc2a0a76f48f2e26419e500abbd4a))
 * **[#519](https://github.com/o2sh/onefetch/issues/519)**
    - Add support for fortran legacy ([`4ad136a`](https://github.com/o2sh/onefetch/commit/4ad136ac45ab792cba2092d22afe80d55eb7895c))
 * **[#527](https://github.com/o2sh/onefetch/issues/527)**
    - Add workflow for Minimum Rust Version badge ([`04d0b01`](https://github.com/o2sh/onefetch/commit/04d0b01216f69fc69cd76a1a879e1f70fc1d2348))
 * **[#528](https://github.com/o2sh/onefetch/issues/528)**
    - Bump libc from 0.2.103 to 0.2.107 ([`6368ea4`](https://github.com/o2sh/onefetch/commit/6368ea42440cf40abb0f2f339694dc48806ebed6))
 * **[#529](https://github.com/o2sh/onefetch/issues/529)**
    - Bump paste from 1.0.5 to 1.0.6 ([`bbd5040`](https://github.com/o2sh/onefetch/commit/bbd50401d2c57aee7e244b306535476c7ce2c8e8))
 * **[#530](https://github.com/o2sh/onefetch/issues/530)**
    - Bump serde_json from 1.0.68 to 1.0.69 ([`72aaa68`](https://github.com/o2sh/onefetch/commit/72aaa685d076755485677283a5931384453cd1a9))
 * **[#531](https://github.com/o2sh/onefetch/issues/531)**
    - Bump anyhow from 1.0.44 to 1.0.45 ([`bc0e249`](https://github.com/o2sh/onefetch/commit/bc0e24900724c44ba06323dab680c554df5467e8))
 * **[#532](https://github.com/o2sh/onefetch/issues/532)**
    - Bump serde_json from 1.0.69 to 1.0.70 ([`5566d87`](https://github.com/o2sh/onefetch/commit/5566d876fb8fcf578e6f324923bcbdc8af401e18))
 * **[#537](https://github.com/o2sh/onefetch/issues/537)**
    - Bump askalono from 0.4.3 to 0.4.4 ([`1c22554`](https://github.com/o2sh/onefetch/commit/1c22554c430c12d2272c6e8d77ffc486eba3c0bf))
 * **[#541](https://github.com/o2sh/onefetch/issues/541)**
    - Bump libc from 0.2.107 to 0.2.108 ([`2c5b850`](https://github.com/o2sh/onefetch/commit/2c5b8503d9f9b978d78dbd74629dbf0b32c762f8))
 * **[#542](https://github.com/o2sh/onefetch/issues/542)**
    - Bump serde_json from 1.0.70 to 1.0.71 ([`dd3d6ac`](https://github.com/o2sh/onefetch/commit/dd3d6acd6f2d650012b20870c7af1f30eeda201b))
 * **Uncategorized**
    - bump version ([`9f89bc2`](https://github.com/o2sh/onefetch/commit/9f89bc2c574bd333e73b60a1d3f984b313cdbd39))
    - add audit.toml to ignore time rustsec errors ([`34afc4d`](https://github.com/o2sh/onefetch/commit/34afc4d933638ff66a060b24370aaa00e1ac1156))
    - Merge pull request #538 from o2sh/dependabot/cargo/git2-0.13.24 ([`0e4ec8c`](https://github.com/o2sh/onefetch/commit/0e4ec8c97f6fb8ac9f00ba09f149ba26da604ed7))
    - Merge pull request #539 from o2sh/dependabot/cargo/strum-0.23.0 ([`6757f42`](https://github.com/o2sh/onefetch/commit/6757f4256133a59018de0d3a963a76e4e6dee582))
    - Merge pull request #540 from o2sh/dependabot/cargo/more-asserts-0.2.2 ([`9346e3c`](https://github.com/o2sh/onefetch/commit/9346e3c103c6848440b7163403631935f3026c0e))
    - Merge pull request #536 from o2sh/dependabot/cargo/anyhow-1.0.48 ([`96a7d02`](https://github.com/o2sh/onefetch/commit/96a7d02ad8810ca668a77d0a9a0ad415407c6fa9))
    - Bump anyhow from 1.0.45 to 1.0.48 ([`9cf9fea`](https://github.com/o2sh/onefetch/commit/9cf9fea922b7172d1bd9a9ffb5b9cea3845a12f7))
    - Bump more-asserts from 0.2.1 to 0.2.2 ([`6eed612`](https://github.com/o2sh/onefetch/commit/6eed61222868389cc982576559d88879ee831884))
    - Merge pull request #535 from o2sh/dependabot/cargo/time-humanize-0.1.3 ([`40e8eca`](https://github.com/o2sh/onefetch/commit/40e8eca31e474c985a057a03ea52e758868c9d20))
    - Bump strum from 0.22.0 to 0.23.0 ([`2678cb9`](https://github.com/o2sh/onefetch/commit/2678cb9d42a8908b2e47b044bb9c42fd7de0fa6f))
    - Bump git2 from 0.13.23 to 0.13.24 ([`c0de936`](https://github.com/o2sh/onefetch/commit/c0de9363d6660f9c68eab2d91586a34fc696a2a4))
    - Bump time-humanize from 0.1.2 to 0.1.3 ([`f4c4a27`](https://github.com/o2sh/onefetch/commit/f4c4a2712156f2c51fc9fe707e86d202a82105f5))
    - Merge pull request #533 from HallerPatrick/main ([`ee6ef95`](https://github.com/o2sh/onefetch/commit/ee6ef95ab2430f858c1f1c80a78c8677391b8533))
    - Update repo.rs ([`742c979`](https://github.com/o2sh/onefetch/commit/742c979b3b07dda2f486734daf6489ad0e170fc7))
    - Merge branch 'main' of https://github.com/HallerPatrick/onefetch into main ([`5359046`](https://github.com/o2sh/onefetch/commit/5359046b9eec55cb524f3753ed1dd2266e0f631e))
    - Adding tests for display time in output ([`3d7b14b`](https://github.com/o2sh/onefetch/commit/3d7b14b856300b145f2b2dfec2ca62cb5c0785e8))
    - Update Cargo.toml ([`4b574ed`](https://github.com/o2sh/onefetch/commit/4b574ed4ce5182d4275d7088d77f099f1251b1da))
    - Merge branch 'o2sh:main' into main ([`829ee02`](https://github.com/o2sh/onefetch/commit/829ee02b5f3edcaae9583df155b437ef705c06b6))
    - #526 Remove chrono as dependency and swiched out with time-rs and time-humanize, refactored code to display creaton of repo time ([`6ceb13f`](https://github.com/o2sh/onefetch/commit/6ceb13f36d589643f016009cad1485e4fb256d93))
    - add DS_STORE to gitignore ([`689a0ab`](https://github.com/o2sh/onefetch/commit/689a0ab8ed92fad7743a72775d709ae43bc84718))
    - fix some language serialization name ([`f86070b`](https://github.com/o2sh/onefetch/commit/f86070b6a102f483b0b58b20fd6ec96d1343664a))
    - update rustfmt ([`f34303b`](https://github.com/o2sh/onefetch/commit/f34303ba2de35ef8334ef2f270d11efe1ea93b51))
    - update Cargo.toml ([`23507b0`](https://github.com/o2sh/onefetch/commit/23507b0971adf8f357d09211d8bbfaf43af63427))
    - update llvm ascii logo ([`f4400c1`](https://github.com/o2sh/onefetch/commit/f4400c15f48c45127c0fe86276b835dbf54c3b78))
    - update license cache ([`e3279b1`](https://github.com/o2sh/onefetch/commit/e3279b1a4973558f49991db84542c10237818de1))
    - change language type for yaml ([`6788f02`](https://github.com/o2sh/onefetch/commit/6788f0273751dc8e8b66296ed02c1257d038f691))
    - update ruby ascii logo ([`34267d4`](https://github.com/o2sh/onefetch/commit/34267d42726957cc0f74128a6e6eed575aa7c43d))
    - add langs module ([`8699d26`](https://github.com/o2sh/onefetch/commit/8699d26444607eb71b1faf8748be223d1d460a90))
    - fix json ascii logo ([`ce78907`](https://github.com/o2sh/onefetch/commit/ce78907a65fa18823008f13bfb79f68f38ab72cc))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`3363357`](https://github.com/o2sh/onefetch/commit/33633573fe2d1d9412229b87f9be8beb0f057212))
    - fix webassembly logo ([`39d36f3`](https://github.com/o2sh/onefetch/commit/39d36f3936b78337f40643f3bca9f951887951c6))
    - Update README.ru.md ([`69d7726`](https://github.com/o2sh/onefetch/commit/69d7726c81af3a9365b9d5938b031ef5411d18f0))
    - Update README.fa.md ([`832e79d`](https://github.com/o2sh/onefetch/commit/832e79dc426d831375a52ca19d411238b10b298a))
    - Update README.cn.md ([`be7c502`](https://github.com/o2sh/onefetch/commit/be7c502d2e7749bb9a24925bb75dabea22d1852a))
    - Update README.ja.md ([`f7e42c5`](https://github.com/o2sh/onefetch/commit/f7e42c570ee1de9a65d2f1a4178a1187c1c5dde7))
    - Update README.md ([`090732f`](https://github.com/o2sh/onefetch/commit/090732fa7a81befc0fc7564cb869bc563084aae1))
    - switch to weekly schedule ([`b85dda8`](https://github.com/o2sh/onefetch/commit/b85dda815ed3a9275d5c4d6b858f23d6ccad15e1))
    - update css logo ([`4a1a63a`](https://github.com/o2sh/onefetch/commit/4a1a63a785daf5ba647d0965be6d9533aec95548))
    - update html and css ascii logo ([`bdcfca1`](https://github.com/o2sh/onefetch/commit/bdcfca16d0710c283a1392858a3b7376df12015b))
    - update html and css ascii logo ([`16f04e7`](https://github.com/o2sh/onefetch/commit/16f04e7ecaf270814ebe6419b684ab0a32d64fa4))
    - fix cargo clippy ([`ce74501`](https://github.com/o2sh/onefetch/commit/ce74501b141acd907da1b34e44172d3afac9c2c3))
    - fix r ascii logo ([`066d18d`](https://github.com/o2sh/onefetch/commit/066d18d2e84435f0d79cc922a9757785c97a9532))
    - fix nim ascii logo ([`ec42580`](https://github.com/o2sh/onefetch/commit/ec4258067d3c36072c39e5915e504d3c2177f97b))
    - fix zig ascii logo ([`1138c2a`](https://github.com/o2sh/onefetch/commit/1138c2abeffd9a03d0d85ba90718395274b191cb))
    - fix lisp ascii logo ([`f4ee5fc`](https://github.com/o2sh/onefetch/commit/f4ee5fc035b74efe2e09c49a34593371aecdeb04))
    - enable wrap_help feature for clap ([`8499b2a`](https://github.com/o2sh/onefetch/commit/8499b2ae649c15f0a1e1e71fc93dd7330830aa07))
    - remove error-chain recursion limit ([`dfe5646`](https://github.com/o2sh/onefetch/commit/dfe5646a83f07019306d4199c1a1767e9be80eb5))
    - #467, fix coloring protobuf ([`ce07578`](https://github.com/o2sh/onefetch/commit/ce07578a980e30bf5a9989abd61c34e53f2b46e8))
    - Merge pull request #468 from HallerPatrick/main ([`2bc490f`](https://github.com/o2sh/onefetch/commit/2bc490fe0b4cb7a78b89534bc80acfea2f2e562f))
    - final touches on protobuf logo ([`dcee332`](https://github.com/o2sh/onefetch/commit/dcee332eddb5f266ca8cf8cb0a4f420930945e89))
    - final touch on protobuf logo ([`c2ff9e1`](https://github.com/o2sh/onefetch/commit/c2ff9e1fd9ad305a080cd7f2b59cb2fb39deab40))
    - trim trailing whitespade in ascii art ([`579405f`](https://github.com/o2sh/onefetch/commit/579405ff2e2a23179db25d9d73ae20472599bf77))
    - improved symetry for protobuf logo ([`254498e`](https://github.com/o2sh/onefetch/commit/254498e36fbd14fc4cda617d9e50488b5e5efbbf))
    - protobuf logo improved symetry ([`80af6ec`](https://github.com/o2sh/onefetch/commit/80af6ec54b34fa2847984775f44896a584cb1a88))
    - New language support for protobuf with ascii image ([`04dd387`](https://github.com/o2sh/onefetch/commit/04dd3876e26d599927162594008fd065bf468def))
    - fix exclude option ([`4cf3600`](https://github.com/o2sh/onefetch/commit/4cf3600822496cf43b0eaa18900ad0fd7fc3bb07))
    - change cli name from --show-email to --email ([`6d032da`](https://github.com/o2sh/onefetch/commit/6d032da6376958e5617939e764ab403581c5f183))
    - remove unnecessary logic in exclude pattern ([`968009e`](https://github.com/o2sh/onefetch/commit/968009e43c406d9650930284a7d6cdc3b9ccf111))
    - Update README.cn.md ([`77502b4`](https://github.com/o2sh/onefetch/commit/77502b4fbe2f9bbf2331d0d92363f109a2b05393))
    - relative link for CONTRIBUTING.md in README.md files ([`2607edb`](https://github.com/o2sh/onefetch/commit/2607edbd200e9f63cc8a10e07fc056eb860c052f))
    - fix text coloring ([`23e6d53`](https://github.com/o2sh/onefetch/commit/23e6d53adefa6e9350f66884a0c8c781b4d12967))
    - refacto info formatter ([`d890933`](https://github.com/o2sh/onefetch/commit/d8909335ed0909678f740d3661d0dd38c755d4fe))
    - update assets ([`3a15223`](https://github.com/o2sh/onefetch/commit/3a15223c32c87e0d6fd3cd2972ec390ff727527e))
    - revert to old screenshots ([`4bc907c`](https://github.com/o2sh/onefetch/commit/4bc907c2303b4559a6a995dd261afc8c849dd3cc))
    - update screenshots ([`5e129ea`](https://github.com/o2sh/onefetch/commit/5e129ea7dcf5d6dc53603bb135bb4d63d9c77101))
    - resize screenshots ([`344a8bc`](https://github.com/o2sh/onefetch/commit/344a8bcb1bee78216b79a6b609d86e11437a205d))
    - Merge branch 'main' of github.com:o2sh/onefetch ([`567d74a`](https://github.com/o2sh/onefetch/commit/567d74ad7916fd5c5493629a8a2c2f604d0c71e0))
    - update screenshots ([`ed6220a`](https://github.com/o2sh/onefetch/commit/ed6220ade2cf8c3656c8889e506763e26690aa1c))
</details>

## v2.10.2 (2021-07-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version ([`0064edc`](https://github.com/o2sh/onefetch/commit/0064edcf612062644749766ba90c8b1b5553ed15))
    - fix text coloring ([`0515730`](https://github.com/o2sh/onefetch/commit/0515730f9170ca1474a4cde1c9520bc7165b40fb))
</details>

## v2.10.1 (2021-07-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version ([`420bc28`](https://github.com/o2sh/onefetch/commit/420bc28fe08d7486391e20c766c53929aaf5052e))
    - add build step to Makefile to fix CD ([`7fc5930`](https://github.com/o2sh/onefetch/commit/7fc59306cc0f5cc06218fbb4c7d4cdbe6e19bdc5))
</details>

## v2.10.0 (2021-07-03)

### New Features

 - <csr-id-b88abf9042732e36380e06aae7418f81a3aee56a/> add Svelte support
   - feat(languages): add Svelte support
* Svelte, no need to override the serialized name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 149 commits contributed to the release over the course of 165 calendar days.
 - 168 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 25 unique issues were worked on: [#380](https://github.com/o2sh/onefetch/issues/380), [#394](https://github.com/o2sh/onefetch/issues/394), [#404](https://github.com/o2sh/onefetch/issues/404), [#412](https://github.com/o2sh/onefetch/issues/412), [#416](https://github.com/o2sh/onefetch/issues/416), [#417](https://github.com/o2sh/onefetch/issues/417), [#418](https://github.com/o2sh/onefetch/issues/418), [#421](https://github.com/o2sh/onefetch/issues/421), [#422](https://github.com/o2sh/onefetch/issues/422), [#425](https://github.com/o2sh/onefetch/issues/425), [#426](https://github.com/o2sh/onefetch/issues/426), [#427](https://github.com/o2sh/onefetch/issues/427), [#429](https://github.com/o2sh/onefetch/issues/429), [#430](https://github.com/o2sh/onefetch/issues/430), [#431](https://github.com/o2sh/onefetch/issues/431), [#435](https://github.com/o2sh/onefetch/issues/435), [#436](https://github.com/o2sh/onefetch/issues/436), [#437](https://github.com/o2sh/onefetch/issues/437), [#438](https://github.com/o2sh/onefetch/issues/438), [#439](https://github.com/o2sh/onefetch/issues/439), [#443](https://github.com/o2sh/onefetch/issues/443), [#445](https://github.com/o2sh/onefetch/issues/445), [#452](https://github.com/o2sh/onefetch/issues/452), [#454](https://github.com/o2sh/onefetch/issues/454), [#456](https://github.com/o2sh/onefetch/issues/456)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#380](https://github.com/o2sh/onefetch/issues/380)**
    - Cache dependencies ([`99e5beb`](https://github.com/o2sh/onefetch/commit/99e5beba0ba4b973456d199fdb0f065e89cb1c60))
 * **[#394](https://github.com/o2sh/onefetch/issues/394)**
    - add haxe language support ([`356f71e`](https://github.com/o2sh/onefetch/commit/356f71e634a48d900377ff35d24560bfc1436b18))
 * **[#404](https://github.com/o2sh/onefetch/issues/404)**
    - Run cargo update ([`af24c66`](https://github.com/o2sh/onefetch/commit/af24c66fe0d1c0fdd2b41cf13bb25487b5728f06))
 * **[#412](https://github.com/o2sh/onefetch/issues/412)**
    - Add Scheme language ([`ae2cc1b`](https://github.com/o2sh/onefetch/commit/ae2cc1b35c876f8b092a1c7eb9fd9021354930a0))
 * **[#416](https://github.com/o2sh/onefetch/issues/416)**
    - Bump regex from 1.4.5 to 1.4.6 ([`e06e137`](https://github.com/o2sh/onefetch/commit/e06e1375fd3b238d647d3f259c503141585b7ba4))
 * **[#417](https://github.com/o2sh/onefetch/issues/417)**
    - Bump libc from 0.2.93 to 0.2.94 ([`613b868`](https://github.com/o2sh/onefetch/commit/613b868eebefce6c5563544824cad8aa99be2023))
 * **[#418](https://github.com/o2sh/onefetch/issues/418)**
    - Bump chrono-humanize from 0.1.2 to 0.2.1 ([`f0978ce`](https://github.com/o2sh/onefetch/commit/f0978ce07b8c64f5ac6e21a0e40d7d3268cd8a66))
 * **[#421](https://github.com/o2sh/onefetch/issues/421)**
    - Bump byte-unit from 4.0.11 to 4.0.12 ([`8c11165`](https://github.com/o2sh/onefetch/commit/8c111651b475ec6a9682b91fc959d74bb0400f31))
 * **[#422](https://github.com/o2sh/onefetch/issues/422)**
    - Bump git2 from 0.13.18 to 0.13.19 ([`9456c88`](https://github.com/o2sh/onefetch/commit/9456c88856f95bd523aaea6c385d5a501759cd66))
 * **[#425](https://github.com/o2sh/onefetch/issues/425)**
    - Bump actions/checkout from 2 to 2.3.4 ([`8b376b8`](https://github.com/o2sh/onefetch/commit/8b376b882f9616e4c2a45a7addc9dbbbd3508b80))
 * **[#426](https://github.com/o2sh/onefetch/issues/426)**
    - Revert actions/checkout to v2 ([`b4b799d`](https://github.com/o2sh/onefetch/commit/b4b799d1c56e70be7db84d89e3971102489df8a4))
 * **[#427](https://github.com/o2sh/onefetch/issues/427)**
    - Bump serde from 1.0.125 to 1.0.126 ([`2bf30e0`](https://github.com/o2sh/onefetch/commit/2bf30e05850894b8fecabdf634613dbc2ddf960c))
 * **[#429](https://github.com/o2sh/onefetch/issues/429)**
    - Bump libc from 0.2.94 to 0.2.95 ([`20a445d`](https://github.com/o2sh/onefetch/commit/20a445da443f506d14f536c793113917c342873d))
 * **[#430](https://github.com/o2sh/onefetch/issues/430)**
    - Make audit scheduled and manually runnable ([`7a0fe2b`](https://github.com/o2sh/onefetch/commit/7a0fe2be732f14ec94f1221b0a78054a249b011a))
 * **[#431](https://github.com/o2sh/onefetch/issues/431)**
    - Bump actions/cache from 2.1.5 to 2.1.6 ([`f841197`](https://github.com/o2sh/onefetch/commit/f841197fb62f20ea42f2c305282292bdc5dd4a19))
 * **[#435](https://github.com/o2sh/onefetch/issues/435)**
    - Added Ada support ([`f8bf292`](https://github.com/o2sh/onefetch/commit/f8bf29206e624b2a8b64752456e00a561122a5bf))
 * **[#436](https://github.com/o2sh/onefetch/issues/436)**
    - add Svelte support ([`b88abf9`](https://github.com/o2sh/onefetch/commit/b88abf9042732e36380e06aae7418f81a3aee56a))
 * **[#437](https://github.com/o2sh/onefetch/issues/437)**
    - Bump strum from 0.20.0 to 0.21.0 ([`34e31dd`](https://github.com/o2sh/onefetch/commit/34e31dd4d9eeafe95162389e48b1c9d47340f39f))
 * **[#438](https://github.com/o2sh/onefetch/issues/438)**
    - Add HCL Support ([`1bfd60a`](https://github.com/o2sh/onefetch/commit/1bfd60a39293b8cbb1a55ac7266732a2302d2d87))
 * **[#439](https://github.com/o2sh/onefetch/issues/439)**
    - Add README.ja.md ([`7262d4a`](https://github.com/o2sh/onefetch/commit/7262d4aa4b178fe4e75ed7bf709432eee934ef7f))
 * **[#443](https://github.com/o2sh/onefetch/issues/443)**
    - Fixed Fortran ASCII to be more symetrical ([`a7cabb2`](https://github.com/o2sh/onefetch/commit/a7cabb242155f21247a31e088831d1761f233af3))
 * **[#445](https://github.com/o2sh/onefetch/issues/445)**
    - Bump libc from 0.2.95 to 0.2.97 ([`fadf515`](https://github.com/o2sh/onefetch/commit/fadf5158bb46e87c930afafdd733ef764f5b51ed))
 * **[#452](https://github.com/o2sh/onefetch/issues/452)**
    - Add CLI option for displaying author emails ([`0130a25`](https://github.com/o2sh/onefetch/commit/0130a253289449fb2bd56b910a553b28a5a5cc40))
 * **[#454](https://github.com/o2sh/onefetch/issues/454)**
    - add support for powershell ([`5244828`](https://github.com/o2sh/onefetch/commit/5244828a369d2d6844249a994910b44d545bd7e1))
 * **[#456](https://github.com/o2sh/onefetch/issues/456)**
    - display the number of contributors ([`4028f8c`](https://github.com/o2sh/onefetch/commit/4028f8c51543ca27cc94a9b3e954e45b6890e9db))
 * **Uncategorized**
    - bump version ([`764f5d5`](https://github.com/o2sh/onefetch/commit/764f5d5deca17209ada37af7e49b2f49910af8dc))
    - fix links in CONTRIBUTING.md ([`9739ddc`](https://github.com/o2sh/onefetch/commit/9739ddc616e269dd9fd7ee6f0d815aba38760d79))
    - fix links in CONTRIBUTING.md ([`dd8691d`](https://github.com/o2sh/onefetch/commit/dd8691d8e42e1ccc9067ba5fe396213c44b53db3))
    - Merge pull request #453 from o2sh/feature/use-mailmap ([`12df1fa`](https://github.com/o2sh/onefetch/commit/12df1fac6fbe126eed64baa1f863a68b9ca883b3))
    - fix json/yaml serializer ([`9b1aa16`](https://github.com/o2sh/onefetch/commit/9b1aa1691607212c187ebe88b65e8e931bd0c211))
    - Merge branch 'main' into feature/use-mailmap ([`6d84c2d`](https://github.com/o2sh/onefetch/commit/6d84c2d1057654a85516d111201c27c097b664c2))
    - reorganize file structure ([`f352e98`](https://github.com/o2sh/onefetch/commit/f352e98f2068a00661c57657c8690f88b46e2ed5))
    - reorganize project file structure ([`7a37b01`](https://github.com/o2sh/onefetch/commit/7a37b01442692ae8658ea27bb341d3394a083229))
    - Update CONTRIBUTING.md ([`735a435`](https://github.com/o2sh/onefetch/commit/735a435e194bc5a4b0ee095d5e9cd7aaa459730a))
    - remove  from language macro in favor tokei fmt ([`da375be`](https://github.com/o2sh/onefetch/commit/da375be6f7810f4ef00b83fd21043bb0fe22be4d))
    - Update CONTRIBUTING.md ([`f4869ee`](https://github.com/o2sh/onefetch/commit/f4869ee693cdb62a552bdd09b8439c3fbbde5a3a))
    - update CONTRIBUTING.md ([`16a971b`](https://github.com/o2sh/onefetch/commit/16a971bfbadeea4f68ea328c3f11f15b6e12c635))
    - update CONTRIBUTING.md ([`b4ade98`](https://github.com/o2sh/onefetch/commit/b4ade9892934b8e79524d1140c6c4da1f8ab219c))
    - clean powershell and zsh ascii logos ([`405ed07`](https://github.com/o2sh/onefetch/commit/405ed074ea2b6553f156e3e05d1b1d5302347021))
    - clean zsh ascii logo ([`0e42855`](https://github.com/o2sh/onefetch/commit/0e4285573f6555be7a4dddd81d9119d558284b72))
    - fmt image_to_ascii.py ([`abce472`](https://github.com/o2sh/onefetch/commit/abce472ddfa837711272c7941cd9e0d7b9eefa14))
    - replace master to main ([`228f385`](https://github.com/o2sh/onefetch/commit/228f3858d918f336fee4d4e6b6c0b4629f0d1e9e))
    - remove empty line jupyter ascii logo ([`20b721e`](https://github.com/o2sh/onefetch/commit/20b721e1be406f4f2a342f8eed7cd04e10ddd685))
    - fix javascript ascii logo ([`eb26d9c`](https://github.com/o2sh/onefetch/commit/eb26d9ccd0a9cb3230a34928ff52722ae2b904f8))
    - fix symmetry on jupyter ascii logo ([`1e0b6c9`](https://github.com/o2sh/onefetch/commit/1e0b6c90092afdc634f15b852cc29b62eaf82101))
    - fallback to commit.author() if commit.author_with_mailmap() fails ([`e031a88`](https://github.com/o2sh/onefetch/commit/e031a88ce7718330174b8a691a5921d4917ac5eb))
    - cargo clippy ([`67ea4c7`](https://github.com/o2sh/onefetch/commit/67ea4c7a092f750be19692bd3d008f9dad86be9b))
    - merge authors by signature ~ username + email ([`98f5756`](https://github.com/o2sh/onefetch/commit/98f57566ce05b91294dea51af72102a1bd37f114))
    - use mailmap bindings ([`9ba9533`](https://github.com/o2sh/onefetch/commit/9ba9533ba179744fc3f2ba2c81b73a6c8bf8ac98))
    - update .mailmap ([`49e46e5`](https://github.com/o2sh/onefetch/commit/49e46e515e7f835708057c127c8e9b6deb928457))
    - add .mailmap ([`47839dc`](https://github.com/o2sh/onefetch/commit/47839dcab8e8f7731c8374779a1f3ccb2f08027c))
    - use cargo install instead of sudo cp in Makefile ([`7c201c7`](https://github.com/o2sh/onefetch/commit/7c201c7753a67ece1f79043866f2455d3f01a265))
    - fix warning: derive helper attribute is used before it is introduced ([`2e39715`](https://github.com/o2sh/onefetch/commit/2e397155b3ab2ed6f15ea8ec2f3e84fd07dd20f7))
    - fmt cli flags declaration ([`28a1c13`](https://github.com/o2sh/onefetch/commit/28a1c13347c36242c29db2c7c144b5dd3e2b47a7))
    - cargo clippy ([`bff472f`](https://github.com/o2sh/onefetch/commit/bff472fa40f5b1b3336e6cd3004f7c62c2c102d9))
    - better centering for fortran ascii logo ([`b5d8d27`](https://github.com/o2sh/onefetch/commit/b5d8d27259963ad3e49aa7ed220fbcba0beeefbe))
    - Merge pull request #448 from o2sh/feature/ignore-bot-commits ([`a611fe4`](https://github.com/o2sh/onefetch/commit/a611fe48154b1f8d61f60c1ce9ed776b3cc229bc))
    - CR, ignore bot commits ([`eb526b2`](https://github.com/o2sh/onefetch/commit/eb526b29c5ae87aaf0bfffe06fa440ac40635110))
    - no-bots with optional pattern ([`63d2234`](https://github.com/o2sh/onefetch/commit/63d22345fa679a96a69a694a874ad481802e6260))
    - ignore bots in commits ([`76e665f`](https://github.com/o2sh/onefetch/commit/76e665f77a0dcfec674e7040904186d06a9a4ac1))
    - onefetch logo retouching ([`0ede2b6`](https://github.com/o2sh/onefetch/commit/0ede2b691c34c296a46db924fb5ee5ecf0e0ea31))
    - onefetch logo retouching ([`bb042c0`](https://github.com/o2sh/onefetch/commit/bb042c03496ba547c0e72351f646c8bd7c260f4d))
    - fix dimensions onefetch.svg ([`ce5fa77`](https://github.com/o2sh/onefetch/commit/ce5fa778123acdc884f32583c5fafecdfaff8bac))
    - test logo with white bg ([`631a1d2`](https://github.com/o2sh/onefetch/commit/631a1d257d738cad0f7a739f810341b9f5ca479f))
    - revert back to black logo ([`a84c7eb`](https://github.com/o2sh/onefetch/commit/a84c7ebcf6b4a196569e57ec4cbf740cbd2c1674))
    - Shorter CLI flags ([`aa80dc9`](https://github.com/o2sh/onefetch/commit/aa80dc96cd9a94940ccbba5f1160ca3adc69523b))
    - slight retouching on javascript and typescript logos ([`4b57f3d`](https://github.com/o2sh/onefetch/commit/4b57f3ddc04884298f0d50047f2633181c08101b))
    - #443, better centering in fortran logo ([`c04a7cd`](https://github.com/o2sh/onefetch/commit/c04a7cda31013eb19be0103b24b4a7065239a50f))
    - round up instead of truncate in perc. of contribution ([`6821f96`](https://github.com/o2sh/onefetch/commit/6821f96f48160f52093f9e68d74f84fc43a86b75))
    - bump git2 dep ([`329af1d`](https://github.com/o2sh/onefetch/commit/329af1d31232c8cd7a1fc36078ac619908cb7930))
    - fix color numbering in gdscript ([`888d34e`](https://github.com/o2sh/onefetch/commit/888d34e315b70fe107a2afe9389613ab9951bba9))
    - #428, add support for GDScript ([`f4b2833`](https://github.com/o2sh/onefetch/commit/f4b2833b9625dabd90ad47ecd07d2031983812d8))
    - fix #381, use mislav/bump-homebrew-formula-action@v1 ([`63294f0`](https://github.com/o2sh/onefetch/commit/63294f06b97eebfdaaf6491f6015f283108abfa3))
    - cargo update ([`bafef56`](https://github.com/o2sh/onefetch/commit/bafef56511d4755c147ee57a36aba845c0d97c4d))
    - Merge pull request #414 from o2sh/dependabot/cargo/git2-0.13.18 ([`50b5d6a`](https://github.com/o2sh/onefetch/commit/50b5d6a56ea280c38a51f149e0314fd918b1102e))
    - Merge pull request #415 from o2sh/dependabot/cargo/byte-unit-4.0.11 ([`4c7f07f`](https://github.com/o2sh/onefetch/commit/4c7f07fa78834abdacfa56c8c0d79ef4ab534686))
    - Bump byte-unit from 4.0.10 to 4.0.11 ([`7a0438a`](https://github.com/o2sh/onefetch/commit/7a0438aa01ede79414015b56c3e5c98c893910b2))
    - Bump git2 from 0.13.17 to 0.13.18 ([`d101185`](https://github.com/o2sh/onefetch/commit/d1011850ebbf85cffffc300be9b8146382712cee))
    - cargo update ([`03d3820`](https://github.com/o2sh/onefetch/commit/03d38203afb82887971452efaa3a3b4800fbb939))
    - Merge pull request #411 from o2sh/dependabot/cargo/libc-0.2.93 ([`c9bc9f8`](https://github.com/o2sh/onefetch/commit/c9bc9f8dcc30185c0c6e120815a53798ece46ee6))
    - Merge pull request #413 from o2sh/dependabot/github_actions/actions/cache-v2.1.5 ([`8bd5093`](https://github.com/o2sh/onefetch/commit/8bd5093433fb72cb257feefda07348a8eb024887))
    - Bump actions/cache from v2.1.4 to v2.1.5 ([`1688e5b`](https://github.com/o2sh/onefetch/commit/1688e5bb47d0635810d997601b2e3c07dcb00984))
    - Bump libc from 0.2.92 to 0.2.93 ([`feb1d6a`](https://github.com/o2sh/onefetch/commit/feb1d6a11a0132c8107a6798cf6a3004ec8dff7a))
    - cargo update ([`f88945c`](https://github.com/o2sh/onefetch/commit/f88945c2ec18627e9be1048c747d67ccdc2e082a))
    - Merge pull request #410 from o2sh/dependabot/cargo/libc-0.2.92 ([`957f7bb`](https://github.com/o2sh/onefetch/commit/957f7bb87f10b5b24670a3636eba314a52f0c3cf))
    - Bump libc from 0.2.91 to 0.2.92 ([`9d4aa25`](https://github.com/o2sh/onefetch/commit/9d4aa25c204f5a4dd9ff0231bc179248ed030a80))
    - Cargo update ([`0ed670a`](https://github.com/o2sh/onefetch/commit/0ed670a6e4016e7108f39d86cb3c1d34ee6c1ef3))
    - Merge pull request #409 from o2sh/dependabot/cargo/libc-0.2.91 ([`2b1db9e`](https://github.com/o2sh/onefetch/commit/2b1db9ea16265c180846e1eecbcd44a9619777c1))
    - Merge pull request #408 from o2sh/dependabot/cargo/serde-1.0.125 ([`47c5adf`](https://github.com/o2sh/onefetch/commit/47c5adfdbe0e7d5e2c0bcbc098f30d28a9784308))
    - Bump libc from 0.2.90 to 0.2.91 ([`8d79f25`](https://github.com/o2sh/onefetch/commit/8d79f25cff12559f83668925e1542a6c1c8388ca))
    - Bump serde from 1.0.124 to 1.0.125 ([`01fa5aa`](https://github.com/o2sh/onefetch/commit/01fa5aacd3b08d472c1a753bf207dfe6a0671b6d))
    - cargo update ([`1ff7370`](https://github.com/o2sh/onefetch/commit/1ff7370379400852269472be6e0af1f3e6964dbc))
    - Merge pull request #407 from o2sh/dependabot/cargo/libc-0.2.90 ([`9be878b`](https://github.com/o2sh/onefetch/commit/9be878b56631c6a5582caafcfc5192732557f9fd))
    - Bump libc from 0.2.89 to 0.2.90 ([`4c21ce6`](https://github.com/o2sh/onefetch/commit/4c21ce62c9cdd34c1c575c50c7ea3b72f7ee31b0))
    - reorder cargo deps ([`4c145a6`](https://github.com/o2sh/onefetch/commit/4c145a6c37650efd150f2311828c5823ee4ecf23))
    - cargo update ([`0f377c5`](https://github.com/o2sh/onefetch/commit/0f377c58fb3b055daff543322452bdc8536a84f0))
    - Merge pull request #405 from o2sh/dependabot/cargo/paste-1.0.5 ([`324f969`](https://github.com/o2sh/onefetch/commit/324f969654644d6172ee12a469f803a10afaadc1))
    - Merge pull request #406 from o2sh/dependabot/cargo/regex-1.4.5 ([`2d75de3`](https://github.com/o2sh/onefetch/commit/2d75de30b436e09b62c42d68a879b56783cc2295))
    - Bump regex from 1.4.4 to 1.4.5 ([`beb09c1`](https://github.com/o2sh/onefetch/commit/beb09c1edb260fcc0965d5b2ab12324848828c27))
    - Bump paste from 1.0.4 to 1.0.5 ([`0796036`](https://github.com/o2sh/onefetch/commit/079603646c92f4829864302ec81d0c9e4564692d))
    - cargo update ([`29e2b29`](https://github.com/o2sh/onefetch/commit/29e2b2998c0277afc78e353baef275c970d5365b))
    - Merge pull request #399 from o2sh/dependabot/cargo/image-0.23.14 ([`eb53737`](https://github.com/o2sh/onefetch/commit/eb53737e33a782863d985c389e3ad2cfa6f017ae))
    - Merge pull request #401 from o2sh/dependabot/cargo/libc-0.2.88 ([`36cf614`](https://github.com/o2sh/onefetch/commit/36cf614831763338dc51af661aaf34315f8d975e))
    - Bump libc from 0.2.85 to 0.2.88 ([`a1484af`](https://github.com/o2sh/onefetch/commit/a1484af70595ccaa635da513f640aec93d3500ee))
    - Merge pull request #403 from o2sh/dependabot/cargo/regex-1.4.4 ([`09bf3fe`](https://github.com/o2sh/onefetch/commit/09bf3fe267b7e86154a915c93141248fcf38a474))
    - Bump regex from 1.4.3 to 1.4.4 ([`223e23e`](https://github.com/o2sh/onefetch/commit/223e23e2b6d2acf97e981100bf972c8a3c05f236))
    - Merge pull request #402 from o2sh/dependabot/cargo/byte-unit-4.0.10 ([`4c4a1ac`](https://github.com/o2sh/onefetch/commit/4c4a1aca062069955c300cce24efff92936f49b8))
    - Bump byte-unit from 4.0.9 to 4.0.10 ([`c3eea6f`](https://github.com/o2sh/onefetch/commit/c3eea6fd9b563a25cb202af0fd98158ede2c906c))
    - Merge pull request #400 from o2sh/dependabot/cargo/serde-1.0.124 ([`742bbca`](https://github.com/o2sh/onefetch/commit/742bbcacc3c4bf5e109c2b79a5f4b2e48a129f28))
    - Bump serde from 1.0.123 to 1.0.124 ([`abe4fcb`](https://github.com/o2sh/onefetch/commit/abe4fcb1c9c49ed941b9455453237191e80c6181))
    - Bump image from 0.23.13 to 0.23.14 ([`ff42efc`](https://github.com/o2sh/onefetch/commit/ff42efc1586337b0fa2710af7bec18b461753847))
    - Merge pull request #397 from o2sh/dependabot/cargo/serde_json-1.0.64 ([`2470998`](https://github.com/o2sh/onefetch/commit/247099895b2aea45451da719c70d0061abfb7d19))
    - Bump serde_json from 1.0.63 to 1.0.64 ([`f6b2856`](https://github.com/o2sh/onefetch/commit/f6b28567fb6c59f1c7f21b14995a19f9d322a421))
    - Merge pull request #396 from o2sh/dependabot/cargo/serde_json-1.0.63 ([`e576426`](https://github.com/o2sh/onefetch/commit/e5764266ff64909bd82a764e768a0dfea6c08d46))
    - Bump serde_json from 1.0.62 to 1.0.63 ([`ab321d0`](https://github.com/o2sh/onefetch/commit/ab321d0e9be0a519f5ddc52324d8f4995accd706))
    - Merge pull request #395 from o2sh/dependabot/cargo/serde_yaml-0.8.17 ([`01a5ee9`](https://github.com/o2sh/onefetch/commit/01a5ee9f81e8fb2d1334a51724fd52aea7fb45e1))
    - Bump serde_yaml from 0.8.16 to 0.8.17 ([`33f0fe5`](https://github.com/o2sh/onefetch/commit/33f0fe5a393fc36a3624f8b16f41962c51916b59))
    - revert better basic coloring for haxe ([`6047ad8`](https://github.com/o2sh/onefetch/commit/6047ad8b4c53f39ba611ccd74442219987ec4fd1))
    - better basic  coloring for haxe ([`a17981f`](https://github.com/o2sh/onefetch/commit/a17981f34dcb94d596d1755fa6fe7b1b4696a4a7))
    - fix haxe alphabetical order ([`bb0c6ac`](https://github.com/o2sh/onefetch/commit/bb0c6ac4f4ecd13ab100a5dae4b438aa37c38359))
    - Merge pull request #387 from o2sh/dependabot/cargo/serde_yaml-0.8.16 ([`5b095a1`](https://github.com/o2sh/onefetch/commit/5b095a1d05ea4b0649f03e809d04d90d5f0b301f))
    - Merge pull request #388 from o2sh/dependabot/cargo/libc-0.2.85 ([`3d3ba18`](https://github.com/o2sh/onefetch/commit/3d3ba18265ffa6323fd29dcd9e330785969fa883))
    - Merge pull request #390 from o2sh/dependabot/cargo/image-0.23.13 ([`061e6c8`](https://github.com/o2sh/onefetch/commit/061e6c899d38aba5f1aea8a11b21bfd3af455f9d))
    - Merge pull request #392 from o2sh/dependabot/github_actions/actions/cache-v2.1.4 ([`2795ead`](https://github.com/o2sh/onefetch/commit/2795ead8f31427c1f3b68958cee14767f3969da7))
    - Bump actions/cache from v2 to v2.1.4 ([`da2c998`](https://github.com/o2sh/onefetch/commit/da2c998e49d9894ce110a3079723202c7b396c29))
    - Bump serde_yaml from 0.8.15 to 0.8.16 ([`ded2920`](https://github.com/o2sh/onefetch/commit/ded2920c4468286a2912a5d354388f46bf33f010))
    - Merge pull request #391 from o2sh/dependabot/cargo/serde_json-1.0.62 ([`4978dc4`](https://github.com/o2sh/onefetch/commit/4978dc4653d00e9d0de9b99239c3e53a86f36f13))
    - Bump serde_json from 1.0.61 to 1.0.62 ([`53e3f8a`](https://github.com/o2sh/onefetch/commit/53e3f8ab0c24c13c00a6ae462a08e4fccffaf605))
    - Bump image from 0.23.12 to 0.23.13 ([`13a8cb7`](https://github.com/o2sh/onefetch/commit/13a8cb7f409ec6e4cd197160feef47a236c1a952))
    - Merge pull request #389 from o2sh/dependabot/cargo/chrono-humanize-0.1.2 ([`405e9e1`](https://github.com/o2sh/onefetch/commit/405e9e1a8c8918c6c688a4c93b0c74ec5ea68d45))
    - Bump chrono-humanize from 0.1.1 to 0.1.2 ([`8c0c92c`](https://github.com/o2sh/onefetch/commit/8c0c92ca24d3c5e77e9c9b6395cfab489e6ea986))
    - Bump libc from 0.2.83 to 0.2.85 ([`597c0c4`](https://github.com/o2sh/onefetch/commit/597c0c40e0b1a8702f2e84df6b6189646af59317))
    - run markdownlint on CONTRIBUTING.md ([`62cfc8c`](https://github.com/o2sh/onefetch/commit/62cfc8c0fcf7535fd35e67ef6b4857ec9ea86c0a))
    - run markdownlint on CONTRIBUTING.md ([`9b7c4f5`](https://github.com/o2sh/onefetch/commit/9b7c4f5274444adbcfb6305b65355d39e5ff2a8c))
    - run markdownlint on CONTRIBUTING.md ([`e308ed5`](https://github.com/o2sh/onefetch/commit/e308ed543142e37881ec0ce8267128e4e08a09cb))
    - Merge pull request #385 from o2sh/dependabot/cargo/libc-0.2.83 ([`b634a80`](https://github.com/o2sh/onefetch/commit/b634a80de9aa37ef41514d286e4cff7110cfb5fb))
    - Merge pull request #384 from o2sh/dependabot/cargo/serde-1.0.123 ([`cd2b176`](https://github.com/o2sh/onefetch/commit/cd2b1763e1ee5c084ba12ef88ee3ce7bd20aaa99))
    - Bump libc from 0.2.82 to 0.2.83 ([`756ed74`](https://github.com/o2sh/onefetch/commit/756ed741ee8be5cb6e0d22a97ab91622600b79e1))
    - Update New Language Request issue template ([`7df570c`](https://github.com/o2sh/onefetch/commit/7df570cbb660ee4f83abf869f8b42aa5b6357e96))
    - Bump serde from 1.0.120 to 1.0.123 ([`90fdec7`](https://github.com/o2sh/onefetch/commit/90fdec765fa38a25ca7399695980257a9aed9585))
    - Merge pull request #383 from o2sh/dependabot/cargo/git2-0.13.17 ([`aac3444`](https://github.com/o2sh/onefetch/commit/aac3444ea264fb899e0a4f6e14a6fba47140719c))
    - Bump git2 from 0.13.16 to 0.13.17 ([`4584e38`](https://github.com/o2sh/onefetch/commit/4584e38d5a6de419fd9e1fffeaa3ddb3ac26dd47))
    - Merge pull request #379 from o2sh/dependabot/cargo/git2-0.13.16 ([`39ffd10`](https://github.com/o2sh/onefetch/commit/39ffd10a230061c2281073aa9b7839d7f4038fd4))
    - Merge pull request #378 from o2sh/dependabot/cargo/serde-1.0.120 ([`39b0bc3`](https://github.com/o2sh/onefetch/commit/39b0bc3fc712ca74f64eedf2206c83acd112b698))
    - Fix cache step ([`31c43fc`](https://github.com/o2sh/onefetch/commit/31c43fcd442cd4a91410d93aa6fa036bde1faab7))
    - Update README.md ([`c5f725e`](https://github.com/o2sh/onefetch/commit/c5f725e274f324e49594bac0cbb686e76812ec0b))
    - Bump git2 from 0.13.15 to 0.13.16 ([`8e1d284`](https://github.com/o2sh/onefetch/commit/8e1d284f4a87025c79fb79f2b631d405784d65d5))
    - Bump serde from 1.0.119 to 1.0.120 ([`c524155`](https://github.com/o2sh/onefetch/commit/c524155cc3a4c1f7ad95bc1e37aae1c444e33d96))
    - fix #377: fix type Typo --> now ago when using to_text_en with accuracy rough ([`ce4fe5b`](https://github.com/o2sh/onefetch/commit/ce4fe5bbbee68feacd8521c309eda4ad0976e105))
</details>

## v2.9.1 (2021-01-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#375](https://github.com/o2sh/onefetch/issues/375), [#376](https://github.com/o2sh/onefetch/issues/376)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#375](https://github.com/o2sh/onefetch/issues/375)**
    - Update dependencies and fix serde related build failure ([`0c36fe6`](https://github.com/o2sh/onefetch/commit/0c36fe6f41c355680a04353f9cce8903c52cb091))
 * **[#376](https://github.com/o2sh/onefetch/issues/376)**
    - fixed spelling mistake ([`5b10724`](https://github.com/o2sh/onefetch/commit/5b10724039f35a62d92a8c2fa586258bef27cb0f))
 * **Uncategorized**
    - prepare release v2.9.1 ([`60e554f`](https://github.com/o2sh/onefetch/commit/60e554f8e560ba12409f819d7765b4f06d464983))
    - reorder --true-color flag ([`5445659`](https://github.com/o2sh/onefetch/commit/5445659ce58c724b128f647f0cef5c1f5ed0a54e))
    - reorder -z flag ([`171c099`](https://github.com/o2sh/onefetch/commit/171c09976e4988d27c8816ec3d76183c02c7e081))
    - bump serde version ([`fbfe1e9`](https://github.com/o2sh/onefetch/commit/fbfe1e933be195a2a55312422557d32987aa74dd))
</details>

## v2.9.0 (2021-01-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 67 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#360](https://github.com/o2sh/onefetch/issues/360), [#362](https://github.com/o2sh/onefetch/issues/362), [#370](https://github.com/o2sh/onefetch/issues/370), [#373](https://github.com/o2sh/onefetch/issues/373)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#360](https://github.com/o2sh/onefetch/issues/360)**
    - Add new language issue template ([`8b540b8`](https://github.com/o2sh/onefetch/commit/8b540b8291d85064b829b49a2341696366516194))
 * **[#362](https://github.com/o2sh/onefetch/issues/362)**
    - add qml support ([`f28c3ce`](https://github.com/o2sh/onefetch/commit/f28c3ce20ca4aca96e391f75e49f1129aba81b12))
 * **[#370](https://github.com/o2sh/onefetch/issues/370)**
    - Add YAML output (-y) and ISO 8601 date format option (-z) ([`3e9cd24`](https://github.com/o2sh/onefetch/commit/3e9cd2466007cf12df0629872d0e428831297594))
 * **[#373](https://github.com/o2sh/onefetch/issues/373)**
    - Add CLI option to switch true colors on/off ([`503d82f`](https://github.com/o2sh/onefetch/commit/503d82f81f6311002e106c5c19bdc5d129817ddc))
 * **Uncategorized**
    - prepare v2.9.0 release ([`6071814`](https://github.com/o2sh/onefetch/commit/60718142bb6ed142e70adca137ab360e4112dd62))
    - Merge pull request #374 from o2sh/dependabot/cargo/tokei-12.1.2 ([`5f55d7b`](https://github.com/o2sh/onefetch/commit/5f55d7b14dae13eeba0b010db0c73226a4160e8a))
    - Bump tokei from 12.1.1 to 12.1.2 ([`75e45f4`](https://github.com/o2sh/onefetch/commit/75e45f4e8c188a95cb5a11c0f6781e56b0fc6da5))
    - append new line when printing in json/yaml format ([`e500690`](https://github.com/o2sh/onefetch/commit/e500690c2b2b48e9b9e91d62bd143444c1fecbf2))
    - Merge pull request #371 from o2sh/dependabot/cargo/regex-1.4.3 ([`a4840ec`](https://github.com/o2sh/onefetch/commit/a4840ec460a0483267140c69ef81f8a4290ca78d))
    - Bump regex from 1.4.2 to 1.4.3 ([`ff03fb9`](https://github.com/o2sh/onefetch/commit/ff03fb9361b6074bf9f72b2dba1fbf8ad36e03a0))
    - Update README.md ([`9d8b818`](https://github.com/o2sh/onefetch/commit/9d8b818922365472b10b015a74a68e40c78aa51f))
    - rustfmt and update README ([`fca326b`](https://github.com/o2sh/onefetch/commit/fca326b748d912a1aea7f3ed44c6e45e5aa5120c))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`30afb06`](https://github.com/o2sh/onefetch/commit/30afb06b411fe1a5444c0f08d60146de0322e510))
    - move man page to docs ([`7701111`](https://github.com/o2sh/onefetch/commit/7701111ae85e76065096014d746420d8bbc89762))
    - Merge pull request #368 from o2sh/dependabot/cargo/libc-0.2.82 ([`d8a0c5b`](https://github.com/o2sh/onefetch/commit/d8a0c5b381d0c14b9c942926d66cc0c53ecde223))
    - Merge pull request #367 from o2sh/dependabot/cargo/yaml-rust-0.4.5 ([`fc3cfa3`](https://github.com/o2sh/onefetch/commit/fc3cfa3c93e2600464b14c60fc87465de840c7e0))
    - Bump libc from 0.2.81 to 0.2.82 ([`aa99c64`](https://github.com/o2sh/onefetch/commit/aa99c647c096be7d3e8704b337da3d024c44ef1e))
    - Bump yaml-rust from 0.4.4 to 0.4.5 ([`a577484`](https://github.com/o2sh/onefetch/commit/a577484b86915e6ae39157e5359a7e5f83c0f0d5))
    - Merge pull request #366 from o2sh/dependabot/cargo/tokei-12.1.1 ([`0d5a9e3`](https://github.com/o2sh/onefetch/commit/0d5a9e3723efe6e93b472a2e930de0d2bac85837))
    - Bump tokei from 12.1.0 to 12.1.1 ([`6f2c326`](https://github.com/o2sh/onefetch/commit/6f2c3261d9c4bafdc74bd8747537b3f430ad7d08))
    - Merge pull request #364 from o2sh/dependabot/cargo/serde_json-1.0.61 ([`538a651`](https://github.com/o2sh/onefetch/commit/538a6514d92b5cf0ff251666b406b8d61150b053))
    - Merge pull request #363 from o2sh/dependabot/cargo/git2-0.13.15 ([`61f49ff`](https://github.com/o2sh/onefetch/commit/61f49ff6e884686c45bf9b228102480cd6966a48))
    - update qml ascii logo + fix type in CONTRIBUTING.md ([`a677544`](https://github.com/o2sh/onefetch/commit/a677544b9f372b28374f46e9d2f07b1ce5e916c8))
    - Bump serde_json from 1.0.60 to 1.0.61 ([`1e1f3d3`](https://github.com/o2sh/onefetch/commit/1e1f3d3b196e5978632aca885edf675fd9f583ce))
    - Bump git2 from 0.13.14 to 0.13.15 ([`aaa9beb`](https://github.com/o2sh/onefetch/commit/aaa9beba0a16498f7a197e1a24f7ac46f04e39d4))
    - Merge pull request #359 from o2sh/dependabot/cargo/tokei-12.1.0 ([`a3ab8fa`](https://github.com/o2sh/onefetch/commit/a3ab8fa0b0b0abb7f7e017db49126a2a69ac2c53))
    - Bump tokei from 12.0.4 to 12.1.0 ([`8cd8378`](https://github.com/o2sh/onefetch/commit/8cd83784f2d180a1fd244e1827ffd34871588563))
    - Merge pull request #160 from erikgaal/patch-homebrew-action ([`76da698`](https://github.com/o2sh/onefetch/commit/76da69840c7d9a9963c952c25d6caf0810b5de19))
    - Merge pull request #356 from o2sh/merry-christmas ([`597f016`](https://github.com/o2sh/onefetch/commit/597f0168eab7658332b35a7656f1f99084e0c049))
    - Merge pull request #358 from o2sh/dependabot/cargo/git2-0.13.14 ([`771c055`](https://github.com/o2sh/onefetch/commit/771c0556e4bc246a24417c3098f986eb58033ed2))
    - Bump git2 from 0.13.13 to 0.13.14 ([`41ba3d1`](https://github.com/o2sh/onefetch/commit/41ba3d1cd5c80b317b27bc195622819063761ca9))
    - Update workflow for new brew command ([`769eb76`](https://github.com/o2sh/onefetch/commit/769eb7607f0004aed1dea1c0785b8e4f9338fe38))
    - Add homebrew.yml ([`4837e15`](https://github.com/o2sh/onefetch/commit/4837e15c81ad82950409c98be29938311a8b03e1))
    - Merge pull request #357 from geeseven/args-chars ([`93bf57c`](https://github.com/o2sh/onefetch/commit/93bf57c33bedf58285ca50ffdcebcd099bca8655))
    - Merge pull request #353 from geeseven/graphql ([`dd2c43b`](https://github.com/o2sh/onefetch/commit/dd2c43b0915f29f2a0bdf9f41a06850e65553d57))
    - add option to change character set ([`1a5c53e`](https://github.com/o2sh/onefetch/commit/1a5c53e84fb24817af4893e4be5d2c3709cc8182))
    - Add Special Thanks section to CONTRIBUTING.md ([`9b254de`](https://github.com/o2sh/onefetch/commit/9b254de933099f9b276985964335f0609abcf40e))
    - Update README.md ([`ddc6e43`](https://github.com/o2sh/onefetch/commit/ddc6e43f83ce14f2e7091035eb0e5ce7cd2dfbf1))
    - Merge pull request #354 from o2sh/dependabot/cargo/toml-0.5.8 ([`443fc13`](https://github.com/o2sh/onefetch/commit/443fc13906ccbff918206380564bb297d020ef8b))
    - Merge pull request #355 from o2sh/dependabot/cargo/git2-0.13.13 ([`d1a6328`](https://github.com/o2sh/onefetch/commit/d1a6328f8f20dc24d0778efd84a9efb04946a4d8))
    - make graphql logo more symetrical ([`b69e84e`](https://github.com/o2sh/onefetch/commit/b69e84e750323641bbfbe985c06cdf871877d46c))
    - Bump git2 from 0.13.12 to 0.13.13 ([`6c9905d`](https://github.com/o2sh/onefetch/commit/6c9905dbba5d1654c790ae0c00ece03b33e5eb2d))
    - Bump toml from 0.5.7 to 0.5.8 ([`7305612`](https://github.com/o2sh/onefetch/commit/7305612a527e5e418cad80df676d95021f571f61))
    - add support for GraphQL ([`a5f58ca`](https://github.com/o2sh/onefetch/commit/a5f58ca985c5a7935eea6fd673ac5acbfe63f4d8))
    - Update README.md ([`ebc1277`](https://github.com/o2sh/onefetch/commit/ebc1277d02d0edf879d837236161f5405ea7389a))
    - Merge pull request #352 from geeseven/emojicode ([`2967891`](https://github.com/o2sh/onefetch/commit/29678913006cc0bc1568692074cb81245010d697))
    - brighter colors for emojicode ([`7728237`](https://github.com/o2sh/onefetch/commit/772823722d732b52a071428f591bf205e3498759))
    - add support for Emojicode ([`7f1ae19`](https://github.com/o2sh/onefetch/commit/7f1ae192806b43a6e478a927e59bc0c999056b23))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`9a12773`](https://github.com/o2sh/onefetch/commit/9a127730c554f84543ad1cadaeb6e315e374fb0d))
    - fix rustfmt ([`c44489a`](https://github.com/o2sh/onefetch/commit/c44489a35fddcef7d9013f780b44f0c7c143bd7a))
    - Update CONTRIBUTING.md ([`ee813b5`](https://github.com/o2sh/onefetch/commit/ee813b57c0fc73e2cb65cb2fb194bd445d579225))
    - rename deps folder ([`ff6dadc`](https://github.com/o2sh/onefetch/commit/ff6dadcb6c846d3c05ab1d2a9bc680f7c1ed371d))
    - Merge pull request #350 from o2sh/feature/macro_package_managers ([`7de7cc2`](https://github.com/o2sh/onefetch/commit/7de7cc26bf5e2079f16eb36f91807c3ca47b8ce0))
    - Update CONTRIBUTING.md ([`db61e8e`](https://github.com/o2sh/onefetch/commit/db61e8edea8b6a5419297c4ece1d6349e2aced93))
    - Update README.md ([`20172d9`](https://github.com/o2sh/onefetch/commit/20172d9fa30735cdc6a7010c8e7939739a8f31ce))
    - Merge branch 'feature/macro_package_managers' of github.com:o2sh/onefetch into feature/macro_package_managers ([`d48decc`](https://github.com/o2sh/onefetch/commit/d48decc730ab87f19f7238ce3836aaef7a3417bf))
    - add support for multiple parsers per package manager ([`43206c4`](https://github.com/o2sh/onefetch/commit/43206c42648db9f7879dde909a296446e3ca0268))
    - Update CONTRIBUTING.md ([`5099d88`](https://github.com/o2sh/onefetch/commit/5099d88891c0f47e15b8f894a380a9755882383c))
    - Update CONTRIBUTING.md ([`e7d712d`](https://github.com/o2sh/onefetch/commit/e7d712d062c61f0375ce1c249ee199468e1c49aa))
    - Update CONTRIBUTING.md ([`6936665`](https://github.com/o2sh/onefetch/commit/6936665e79fd578e0ed80350c6bf029c7a4cc20b))
    - Update CONTRIBUTING.md ([`151f094`](https://github.com/o2sh/onefetch/commit/151f0942771fcedd5d03f290626886b9264e8057))
    - Update README.md ([`1cca41d`](https://github.com/o2sh/onefetch/commit/1cca41de45a1e1966ab858211930bbbd85c35728))
    - Update README.md ([`41f8c67`](https://github.com/o2sh/onefetch/commit/41f8c6704c261719c67b5ab428e56f19e981dcb3))
    - fix clippy warn ([`a7055f1`](https://github.com/o2sh/onefetch/commit/a7055f15145d04b1370d8f98864f65ee1243db1b))
    - create define_package_managers macro ([`dbb88e8`](https://github.com/o2sh/onefetch/commit/dbb88e8e8ac0889fd469bdc25829278058166406))
    - use tuple deconstruction for get_git_info_field ([`966f598`](https://github.com/o2sh/onefetch/commit/966f598748fdf0ee80716628e83f3ca0789db8e8))
    - Update README.md ([`7e9c0fc`](https://github.com/o2sh/onefetch/commit/7e9c0fc33561969a505d8e809e2d4bac34a833ec))
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

 - 75 commits contributed to the release over the course of 24 calendar days.
 - 24 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version to 2.8.0 ([`f3702d2`](https://github.com/o2sh/onefetch/commit/f3702d24c5e207c663b3d1f30abfbbc3b5e04416))
    - fix get_git_info_field when version or username is empty ([`b6705bb`](https://github.com/o2sh/onefetch/commit/b6705bbf625646887d7135696546db7013f1aa72))
    - Merge pull request #346 from HallerPatrick/master ([`8fd575a`](https://github.com/o2sh/onefetch/commit/8fd575a2a11fdc4ea1b90e1ebd1ff757f0456659))
    - Added pub as another package manager for dart projects ([`1e46f4f`](https://github.com/o2sh/onefetch/commit/1e46f4f0031815ba7646ae98064d14352beaff06))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`bf2ffa5`](https://github.com/o2sh/onefetch/commit/bf2ffa5b936259ddfd6700d31acd9f042ee4008c))
    - fix JSON serializer if Git is not installed ([`e01bc38`](https://github.com/o2sh/onefetch/commit/e01bc38142fb69c06abd2cbf735641d2485f47ec))
    - Typo README ([`be899e6`](https://github.com/o2sh/onefetch/commit/be899e687bc16e951a153d1d215fab8fd2c01d07))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`fb22d80`](https://github.com/o2sh/onefetch/commit/fb22d80336afe106ab8368176b9579d5d2ef2509))
    - update nix logo ([`e77557c`](https://github.com/o2sh/onefetch/commit/e77557c9ad28f3bc0a5908e99c92f9530fcc7ece))
    - Update README.md ([`45b86c3`](https://github.com/o2sh/onefetch/commit/45b86c37e2479f441439d6f8244605a3b87fedc5))
    - Update README.md ([`c1fc5eb`](https://github.com/o2sh/onefetch/commit/c1fc5eb0c12371c248e0d75ed0975a6d8575b4f5))
    - Update README.md ([`0902bde`](https://github.com/o2sh/onefetch/commit/0902bde72eceef33c6946c3a991b446b8b394def))
    - update readme ([`0da1c02`](https://github.com/o2sh/onefetch/commit/0da1c02af49835f2aea6cfbe1a57072a8d80cd22))
    - rename format flag to output and other changes ([`667897e`](https://github.com/o2sh/onefetch/commit/667897e63be8f15f7d32775052240cb502049e4f))
    - Merge pull request #341 from HallerPatrick/master ([`80424f4`](https://github.com/o2sh/onefetch/commit/80424f4f32cb572513bd50c305f0c8616e295a58))
    - Code style change ([`ae68b53`](https://github.com/o2sh/onefetch/commit/ae68b53aa23de8e303ea7abffad333d9b472a17f))
    - don't panic when commit not found in get_logs ([`9207c6c`](https://github.com/o2sh/onefetch/commit/9207c6c79d71548a36682871cc15ca5bf27f0daf))
    - clean python logo ([`a2c6c10`](https://github.com/o2sh/onefetch/commit/a2c6c10119f99aeb50c7769686fbcbd95040f095))
    - clean python logo ([`5486d08`](https://github.com/o2sh/onefetch/commit/5486d08ec09f0ad9e3aa9a1902827b75a28d2b95))
    - Multiple changes linked to git2-rs migration ([`e99c56f`](https://github.com/o2sh/onefetch/commit/e99c56f0fe9b03c92e1f3234a322bba6599a4a31))
    - Removed time sorting for faster RT ([`826ac93`](https://github.com/o2sh/onefetch/commit/826ac93e68e24eb1769dc95044be1cb7c520784e))
    - Minor refactoring, now using a more 'safe' conversion of autho mail and name due to invalid utf8 ([`3943231`](https://github.com/o2sh/onefetch/commit/3943231cc23d1d8f60606ebfafd9952180156125))
    - Merge pull request #2 from o2sh/master ([`764dea5`](https://github.com/o2sh/onefetch/commit/764dea5f9381c9148223a62571264d546c8bfc88))
    - Put all git call functions into a GitClient impl, minor refactoring ([`d673169`](https://github.com/o2sh/onefetch/commit/d673169cafdcbac59871c233d1bd51eda26c480f))
    - better cropping for onefetch logo ([`4920dbf`](https://github.com/o2sh/onefetch/commit/4920dbf68da38c970c5ec34634fc509929015d36))
    - little darker onefetch logo #339 ([`224965a`](https://github.com/o2sh/onefetch/commit/224965a0073e0e3f8278c8d251552f99bb76fb06))
    - even brighter onefetch logo #339 ([`58a2cbb`](https://github.com/o2sh/onefetch/commit/58a2cbb9e087825a1a3d4f0e9738e8f90c8cfde1))
    - fix bad cropping of onefetch logo ([`6abbac8`](https://github.com/o2sh/onefetch/commit/6abbac885bd9470cc1cdcc0f6d036448ce0c63b0))
    - brighter onefetch logo #339 ([`a368af8`](https://github.com/o2sh/onefetch/commit/a368af8ba871c28a26c815ca7093c889c4158ee8))
    - add white border to onefetch logo #339 ([`2e466b9`](https://github.com/o2sh/onefetch/commit/2e466b9c8a1eafb6a8479be46636c32d4ecd1c9f))
    - add white border to onefetch logo #339 ([`1c96eef`](https://github.com/o2sh/onefetch/commit/1c96eefa314c7e96b9066f7a644062e624344367))
    - Merge pull request #340 from o2sh/dependabot/cargo/paste-1.0.4 ([`fa45601`](https://github.com/o2sh/onefetch/commit/fa45601a066a030acdde9b974c7957d36517abb3))
    - Bump paste from 1.0.3 to 1.0.4 ([`6ebb9c4`](https://github.com/o2sh/onefetch/commit/6ebb9c4ca8992809a42d3302243da52691ef9658))
    - Merge pull request #1 from o2sh/master ([`929ea96`](https://github.com/o2sh/onefetch/commit/929ea967460422d6a7a08d84ad8b347d8aa1fad9))
    - Replaced os git command with git2 ([`cc1f29c`](https://github.com/o2sh/onefetch/commit/cc1f29ca4a2624fba7eaaed7d61067e85adb2803))
    - Merge pull request #337 from HallerPatrick/master ([`4be57b5`](https://github.com/o2sh/onefetch/commit/4be57b5249aeda8ec017d1d81c399b1bfefad231))
    - Merge branch 'master' of https://github.com/HallerPatrick/onefetch into master ([`ffcb73e`](https://github.com/o2sh/onefetch/commit/ffcb73e7b790f98ba5f14c606cb5e7c01e262a01))
    - Using possible_values instead of validator for format arg ([`2e3760a`](https://github.com/o2sh/onefetch/commit/2e3760ac6ec918e0db3485a20510306b105d49eb))
    - Update src/onefetch/cli.rs ([`9cb465c`](https://github.com/o2sh/onefetch/commit/9cb465cd4777e25a69f57b5d6c903fa64b443b15))
    - Removed serde_test, use of format flag instead of json flag, chang of string concat to format macro in git_utils ([`219190b`](https://github.com/o2sh/onefetch/commit/219190ba1963dabd841cfee3e46e024c3c53ab09))
    - Custom impl of serializer for Info struct, also some refactoring of git_utils functions for more granual information for json ([`33ec3cd`](https://github.com/o2sh/onefetch/commit/33ec3cdb3b6a0f923a693583374c340a3d172b04))
    - Commit of missing changes of last commit ([`9993dcb`](https://github.com/o2sh/onefetch/commit/9993dcb0ec503f3a5729a7bd6401299f8aace111))
    - Implementation of json flag, wich allows to print out a json representation of onefetch output ([`65c7a54`](https://github.com/o2sh/onefetch/commit/65c7a545108c7443e5086aafcfb895d8bd50115b))
    - Merge pull request #336 from o2sh/dependabot/cargo/libc-0.2.81 ([`b48efa3`](https://github.com/o2sh/onefetch/commit/b48efa3246c3590b7546f91c5d425737a5d98b1a))
    - Bump libc from 0.2.80 to 0.2.81 ([`136851f`](https://github.com/o2sh/onefetch/commit/136851f2592269d2db7ed0df4d1e0f900c66a623))
    - Merge pull request #333 from o2sh/dependabot/cargo/bytecount-0.6.2 ([`0757dec`](https://github.com/o2sh/onefetch/commit/0757dec9ca3ec9787b906389e64b9767545d66b4))
    - Bump bytecount from 0.6.1 to 0.6.2 ([`5257420`](https://github.com/o2sh/onefetch/commit/5257420b97ee3ada727a6bd4732e8b2bf44550ac))
    - Update CONTRIBUTING.md ([`4d3a370`](https://github.com/o2sh/onefetch/commit/4d3a370447377817ca5dee91ad294d1a1f1a3886))
    - Update Contributing.md #323 ([`17c0c8e`](https://github.com/o2sh/onefetch/commit/17c0c8efa51fb2d7cd14a25ad2e91a3374ea850d))
    - reorder cli flags ([`2b5b5ef`](https://github.com/o2sh/onefetch/commit/2b5b5ef5d754b65063ea0b2a4f55739b43afd32b))
    - inverse logic for hide-logo flag #330 ([`33ad7de`](https://github.com/o2sh/onefetch/commit/33ad7dede9b8d05191f11878fce89bcfb0d1cbce))
    - Merge pull request #330 from Luke-zhang-04/master ([`a4698dc`](https://github.com/o2sh/onefetch/commit/a4698dc83325b53692991db5aa634f2535d195f8))
    - remove never option for hide-logo flag ([`2979a77`](https://github.com/o2sh/onefetch/commit/2979a7756450450eb2109f860f2cf85ce9ee8461))
    - auto, always, never options for hide-logo with always as default ([`51a7a4b`](https://github.com/o2sh/onefetch/commit/51a7a4bd58f741fd9fb025e863d7d2201b3f2192))
    - Merge pull request #331 from geeseven/image_to_ascii-cleanup ([`940c478`](https://github.com/o2sh/onefetch/commit/940c4786b6615010268233a26e93bb2cedd7e9ee))
    - file name changes and import linting ([`765f1d9`](https://github.com/o2sh/onefetch/commit/765f1d9c60bd2c5e88cd94c514403b23be5dca8a))
    - Modified CLI (See full commit msg for details) ([`c72dbbe`](https://github.com/o2sh/onefetch/commit/c72dbbe116d038cc455ba0c94df57f5ff95eeefb))
    - cargo fmt ([`5762b15`](https://github.com/o2sh/onefetch/commit/5762b151703bdf1ba1f68896ff498315fb3babfd))
    - remove `off` option, allow user to define termianl width ([`711e41e`](https://github.com/o2sh/onefetch/commit/711e41eab1194b050f782e481f775e75ae5fd6a3))
    - use `std::eprintln` ([`bce8f07`](https://github.com/o2sh/onefetch/commit/bce8f07932c63c8ae0d9848dbe1fef2acb8d59fe))
    - get rid of duplicate code ([`39e6f61`](https://github.com/o2sh/onefetch/commit/39e6f61ed251cce1cd7b2350d20f2283f9605263))
    - default to auto ([`9616e79`](https://github.com/o2sh/onefetch/commit/9616e799f0e3e7d7bba1e4284df4b194c4f3d209))
    - fix typo in help message ([`d4ce353`](https://github.com/o2sh/onefetch/commit/d4ce3532fca2cd77d7a80edd4d1cf3cd172843c7))
    - hide logo if terminal size is too wide ([`add14cf`](https://github.com/o2sh/onefetch/commit/add14cfcb25873733af99b952bf351c719fd0c99))
    - move check for valid Git repo further down #329 ([`72e3682`](https://github.com/o2sh/onefetch/commit/72e36822bc00095c1eb7a19976eb770a2ef86642))
    - Merge pull request #326 from Luke-zhang-04/master ([`fb8a387`](https://github.com/o2sh/onefetch/commit/fb8a38714dcc3007699be9a2ec6bb65acaa2ddef))
    - clean processing ascii logo ([`423c357`](https://github.com/o2sh/onefetch/commit/423c357eff5e2871e43fd8a8dba18a15222e37c4))
    - Merge pull request #324 from geeseven/vala ([`8730e15`](https://github.com/o2sh/onefetch/commit/8730e15e91b4c26c35051e835c351a86675f5fb5))
    - clean vala ascii logo ([`56d6e9b`](https://github.com/o2sh/onefetch/commit/56d6e9b2147b629cce706478a26a00c37a7af002))
    - Merge pull request #325 from o2sh/dependabot/cargo/strum-0.20.0 ([`8e57142`](https://github.com/o2sh/onefetch/commit/8e57142b93f0abec9dcfbcf107501d8aafc28cdc))
    - add processing support ([`bc51dd5`](https://github.com/o2sh/onefetch/commit/bc51dd5b67b70c27c1dc197259abab37504967cc))
    - Bump strum from 0.19.5 to 0.20.0 ([`18baf84`](https://github.com/o2sh/onefetch/commit/18baf84e2b54d091481fe74bc062b2b4d4422120))
    - add support for Vala ([`b596d6c`](https://github.com/o2sh/onefetch/commit/b596d6c89de5b310195607ce3eb0eaf3c7c39622))
    - Merge pull request #322 from geeseven/vimscript ([`d546f20`](https://github.com/o2sh/onefetch/commit/d546f20d245414215063c75961b8f930ed5b36b0))
    - add support for VimScript #321 ([`8d03557`](https://github.com/o2sh/onefetch/commit/8d03557c930d16209b3938d06fdf91a5d39fed5e))
</details>

## v2.7.3 (2020-11-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump versionn to 2.7.3 ([`b15b32a`](https://github.com/o2sh/onefetch/commit/b15b32ae5af689c46d17548a2e2a4804c587b739))
    - if user.name is not set print unknown instead of crashing ([`6ff85ef`](https://github.com/o2sh/onefetch/commit/6ff85efb5f227cf73b43a37210a5b8f6087bc4dc))
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

 - 50 commits contributed to the release over the course of 13 calendar days.
 - 13 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version to 2.7.2 ([`09eda66`](https://github.com/o2sh/onefetch/commit/09eda665025f5ec62f3d4a1c89ea12e9ded64136))
    - fix image backend detection ([`7d92b48`](https://github.com/o2sh/onefetch/commit/7d92b48ff0431f3a84c06f0c1229be87f6c6e2f7))
    - Merge pull request #319 from o2sh/dependabot/cargo/bytecount-0.6.1 ([`8397b9d`](https://github.com/o2sh/onefetch/commit/8397b9d18403f08f6d54c7cb67ea4561ec1d5c92))
    - Bump bytecount from 0.6.0 to 0.6.1 ([`5712b97`](https://github.com/o2sh/onefetch/commit/5712b97c63da402822c676e665331bec89cdc19c))
    - Merge pull request #318 from o2sh/dependabot/cargo/image-0.23.12 ([`ed1359c`](https://github.com/o2sh/onefetch/commit/ed1359c186acb7b2acf6dbd59a261c5969e4b47f))
    - cargo fmt ([`8b3cd82`](https://github.com/o2sh/onefetch/commit/8b3cd82502d8f973dc0114c24ec3d194aa5b8b81))
    - fix deprecated call to_rgba --> to_rgba8 ([`0428be2`](https://github.com/o2sh/onefetch/commit/0428be2d4961b294f589fee235c8d6757f58bbf9))
    - Bump image from 0.23.11 to 0.23.12 ([`d470578`](https://github.com/o2sh/onefetch/commit/d470578e2531093a051fad3513044e3b6608a2f1))
    - add check for empty rep_name and repo_url ([`43e540f`](https://github.com/o2sh/onefetch/commit/43e540f7b4f2f01958a02f88632271089b69b89e))
    - remove UnknownField from InfoField ([`2d17efa`](https://github.com/o2sh/onefetch/commit/2d17efa6c16c8d10fbb7d5a2e3ce43ebce28fc1e))
    - refacto info_fields ([`e121796`](https://github.com/o2sh/onefetch/commit/e1217962e95de6c5bab5a995aa2050667038fe1b))
    - better coloring for typescript and lua ([`050e693`](https://github.com/o2sh/onefetch/commit/050e693c187ea60899500375c3563b794660c001))
    - better coloring for c, c# and c++ ([`e50723e`](https://github.com/o2sh/onefetch/commit/e50723e3523628fb2c96d5c213f7de6db8212790))
    - better coloring for holyc ascii logo ([`1b71fc0`](https://github.com/o2sh/onefetch/commit/1b71fc0cbca9cbcd99f19387322468d7ad0793c5))
    - Merge pull request #313 from Ferryistaken/holyc-support ([`0af1dcf`](https://github.com/o2sh/onefetch/commit/0af1dcf3187061f1ac0684874bb8fc20731b238f))
    - clean holyC ascii design ([`1bbbcaa`](https://github.com/o2sh/onefetch/commit/1bbbcaa8d8cecee2d05593a2d64d299c468e7cae))
    - new logo ([`4e4b4a4`](https://github.com/o2sh/onefetch/commit/4e4b4a411e30a177a7160565f60e217c306ba427))
    - clean some ascii logo ([`efc9692`](https://github.com/o2sh/onefetch/commit/efc969210ee9bf7df04636bad9b788ce5beebb43))
    - fix raku ascii logo ([`f3cbcf7`](https://github.com/o2sh/onefetch/commit/f3cbcf70b96e4c4d9b77a32a0a029b642bf30795))
    - fix cargo clippy ([`cad3ab3`](https://github.com/o2sh/onefetch/commit/cad3ab31ae2d71a4a876bb611aad4b0db556ece8))
    - fix max-width CI tests ([`b4fb7a6`](https://github.com/o2sh/onefetch/commit/b4fb7a6a12a7c72c4608e59f78d16fdc5c3203b8))
    - fix ration in xaml ascii logo #317 ([`d744a58`](https://github.com/o2sh/onefetch/commit/d744a58c1b590b40555d9172d08244ddf0a5436c))
    - fix xaml ascii logo #317 ([`f3283ed`](https://github.com/o2sh/onefetch/commit/f3283edec81bd373b2cda37d12a707b57b69cced))
    - add support for Xaml #317 ([`05fec5a`](https://github.com/o2sh/onefetch/commit/05fec5af40ecc223591ff344f03b4b6e87252aef))
    - Merge pull request #315 from o2sh/replace-git-sys-calls ([`cff720a`](https://github.com/o2sh/onefetch/commit/cff720a7f442ca31cd911d8c7ab11cdbed51eff4))
    - extract remaining git sys calls into seperate file ([`b8859a4`](https://github.com/o2sh/onefetch/commit/b8859a490753d36126bc5d103711a7688ca46b76))
    - move Printer into its own file ([`a87dc8f`](https://github.com/o2sh/onefetch/commit/a87dc8f5842a406d9b7fafeb7b4b87e108e5b2a3))
    - move is_git_installed in cli_utils.rs ([`96e3a55`](https://github.com/o2sh/onefetch/commit/96e3a551c40a7780ab7eaa896f22e50c269773b6))
    - move get_git_version in cli_utils.rs ([`1fecaea`](https://github.com/o2sh/onefetch/commit/1fecaea0eb4c023e317e964182c05047cbddd417))
    - extract get_ascii_colors from info.rs ([`0c9fcbb`](https://github.com/o2sh/onefetch/commit/0c9fcbb3122e5b4249151e9249fc6d6a40b0db27))
    - Merge pull request #316 from o2sh/dependabot/cargo/paste-1.0.3 ([`8dd4255`](https://github.com/o2sh/onefetch/commit/8dd4255e28446e83149df58f57c0a31dfa96f1f4))
    - fix underflow on get_number_of_branches ([`427029f`](https://github.com/o2sh/onefetch/commit/427029f32a2c96b1039dd379d553257835e3d739))
    - Update src/onefetch/repo.rs ([`d159306`](https://github.com/o2sh/onefetch/commit/d15930630b2a01ffeb724efbee15258ecc23454c))
    - Update src/onefetch/repo.rs ([`597814a`](https://github.com/o2sh/onefetch/commit/597814a131c1912e53af5737594d8c85bbf56291))
    - migrate get_number_of_tags_branches ([`e0dd9ef`](https://github.com/o2sh/onefetch/commit/e0dd9ef950ee46031e31d8b1e9765f391094a385))
    - filter out tags from HEAD refs ([`c28404d`](https://github.com/o2sh/onefetch/commit/c28404d53e4b57c5d47acaaecc42b399d73de824))
    - Bump paste from 1.0.2 to 1.0.3 ([`ebf5797`](https://github.com/o2sh/onefetch/commit/ebf5797b1213af8e7e248facc65579b2fdd72972))
    - remove releases_number ([`59cb0d5`](https://github.com/o2sh/onefetch/commit/59cb0d5e0e5548817a6c0672c8b42b2e7e45ea5d))
    - cargo fmt ([`b491f3a`](https://github.com/o2sh/onefetch/commit/b491f3a655e06be7b5ede856ed1d964ea9a3adcb))
    - create Repo struct and migrate get_version ([`f303b2a`](https://github.com/o2sh/onefetch/commit/f303b2a5558eecca5123e872c0684cb2255d6b29))
    - migrate get_pending_changes ([`c995f3e`](https://github.com/o2sh/onefetch/commit/c995f3e6b97945202ab739d4db6daa90fc611f85))
    - get_repo_name_and_url defaults to remote.origin.url #314 ([`a183a32`](https://github.com/o2sh/onefetch/commit/a183a32daae1b2ada1f52a21a734f950dea614f0))
    - Halo is now centered ([`b0d1259`](https://github.com/o2sh/onefetch/commit/b0d1259169d1274e659d1d90b0dd4515b65c2e87))
    - New HolyC logo ([`fa8d175`](https://github.com/o2sh/onefetch/commit/fa8d1752d32f021a608d29b78cf71b1b1d7fe4ac))
    - extract som git related fn into separate file ([`e1b7027`](https://github.com/o2sh/onefetch/commit/e1b7027f9b02b7214dce59e483e4f62d54159767))
    - cargo fmt and cargo clippy ([`4f221f7`](https://github.com/o2sh/onefetch/commit/4f221f7179cbdc2bc0b0952314a6c1fe09a757bd))
    - add is_valid_repo check in main ([`a3e0a5d`](https://github.com/o2sh/onefetch/commit/a3e0a5da1f5be974dea40fcd72e90ca29b4c5ef9))
    - update CONTRIBUTING to fix max-width value #313 ([`71649ab`](https://github.com/o2sh/onefetch/commit/71649abc02dbeeee5f0dd1f5f4ebb7cc1d6f60d5))
    - fixed ascii logo ([`585a51d`](https://github.com/o2sh/onefetch/commit/585a51de31b6b7e17c1c028d65e7c923f5de63db))
    - Added Support for HolyC ([`ed4c5f1`](https://github.com/o2sh/onefetch/commit/ed4c5f1e6d2ba4bc8171e5b39c6d1d5851586863))
</details>

## v2.7.1 (2020-11-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version to 2.7.1 ([`e769ca2`](https://github.com/o2sh/onefetch/commit/e769ca2295aafdea4543a29d269fd0dc2c0a9609))
    - remove language::unknown ([`9849aad`](https://github.com/o2sh/onefetch/commit/9849aad6b39f5a8d18797b5829c2d19e01994cc7))
    - update man page ([`597e17d`](https://github.com/o2sh/onefetch/commit/597e17dacc5e7144261fa924d0ecbdc5660bf0ad))
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

 - 43 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update assets and bump version ([`4ee1c1e`](https://github.com/o2sh/onefetch/commit/4ee1c1ed3a3fa29fa369c2fd6660bde85edf31e9))
    - better var and fn naming ([`d9e8184`](https://github.com/o2sh/onefetch/commit/d9e81846d8c2b66a999b4c351ecf2d3c377018c7))
    - add print-package-mangers cli option ([`6c4f409`](https://github.com/o2sh/onefetch/commit/6c4f4094ff54357ecd6d9b13346b18c60f89df56))
    - fix cargo clippy and go module parser and remove yarn logic ([`9415090`](https://github.com/o2sh/onefetch/commit/9415090a1cce3afd3a9ef34ffe9908ccfc1ab8d6))
    - Merge pull request #304 from Luke-zhang-04/master ([`1a31c98`](https://github.com/o2sh/onefetch/commit/1a31c988d1830efd7517ee714e9c306a8d317907))
    - updat assets ([`72a2c0b`](https://github.com/o2sh/onefetch/commit/72a2c0b96e35b3cd450c5cd8ea16a513bd94c68c))
    - Merge pull request #311 from yoichi/remove-unnecessary-newline ([`25babe8`](https://github.com/o2sh/onefetch/commit/25babe8c5f464b743b02e868ad1711a9fae463a2))
    - check for `=>` in go.mod ([`ba97550`](https://github.com/o2sh/onefetch/commit/ba97550c74cd87cff4b86da730ae1137d71e55fc))
    - update CONTRIBUTING.md ([`2677401`](https://github.com/o2sh/onefetch/commit/26774012f313563bb336f304c50641325871d06b))
    - Merge branch 'master' of https://github.com/o2sh/onefetch Update branch for merging ([`aa62542`](https://github.com/o2sh/onefetch/commit/aa6254291a9796bd344c55099e63c542a61ee3df))
    - Merge branch 'master' of https://github.com/Luke-zhang-04/onefetch Changes were made to CONTRIBUTING.md ([`483faa6`](https://github.com/o2sh/onefetch/commit/483faa6073ddf2744e2e10f5c2d43fd009e3863e))
    - handle Cargo.toml without dependency field ([`1d6d95b`](https://github.com/o2sh/onefetch/commit/1d6d95b2d42fffa297a19552c193f7d59220d850))
    - Update CONTRIBUTING.md ([`978c9c8`](https://github.com/o2sh/onefetch/commit/978c9c88e4ac3f1e4e5692d6328807492a8c1979))
    - update assets ([`c94676c`](https://github.com/o2sh/onefetch/commit/c94676c19d516560894bcd6e1b3a4e82b8220958))
    - make contributing clearer ([`8824206`](https://github.com/o2sh/onefetch/commit/88242067d3aa68cbfe4b97c822a7725d71a5b212))
    - add a comment ([`3d94170`](https://github.com/o2sh/onefetch/commit/3d94170377545529a6db0b5c50dfa59c71cca621))
    - use `map.contains_key()` instead of iterating ([`0b8caa0`](https://github.com/o2sh/onefetch/commit/0b8caa0b52bdd1eddb585ecef8eaa8f8b73d4e88))
    - move `is_package_file` to Detector impl` ([`6fdb61d`](https://github.com/o2sh/onefetch/commit/6fdb61d9187308f568833227ac2a8a1c358d66b9))
    - catch dependencies instead of `.unwrap()` ([`d60ecfe`](https://github.com/o2sh/onefetch/commit/d60ecfe3dda5b74531d77aaff6b648eac6be7799))
    - simplify regex in package parsers ([`a19ced8`](https://github.com/o2sh/onefetch/commit/a19ced85921a17a42ad2d76b744ba765f1038341))
    - detect `yarn.lock` with absolute directory ([`9311d49`](https://github.com/o2sh/onefetch/commit/9311d49cd4637508c19bccd2c10883fb45b3de84))
    - change i32 in `package_parsers` to uint ([`afc7ef0`](https://github.com/o2sh/onefetch/commit/afc7ef086bcf24687735969d69afa12fa9ef090d))
    - adding a new package manager ([`5bb1e05`](https://github.com/o2sh/onefetch/commit/5bb1e05899962b3edf704d69e6c18caa5c23c8cd))
    - split deps into multiple files and replace Option with Result for better error handling ([`369506c`](https://github.com/o2sh/onefetch/commit/369506c03c3989962f635a1c6c731cafb40991d5))
    - fix trailing white space in LOC ([`a466b06`](https://github.com/o2sh/onefetch/commit/a466b06032883be38d57678a21b8e3448f2137cc))
    - colon is white by default ([`50f2b31`](https://github.com/o2sh/onefetch/commit/50f2b3113bb9b882298b513cf8407f898cda11e1))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`a8d99c9`](https://github.com/o2sh/onefetch/commit/a8d99c97eb49754bc786e546edf457d5513c7368))
    - update assets ([`1877978`](https://github.com/o2sh/onefetch/commit/187797892218e14c9207e63cc0da337482db380d))
    - Update README.md ([`b4da888`](https://github.com/o2sh/onefetch/commit/b4da888c3579019a18310315e54381636dbbd064))
    - Don't output unnecessary newline in supported() methods ([`44e47eb`](https://github.com/o2sh/onefetch/commit/44e47eb4cb69ae8ebef062939e4b13baf8d63e9c))
    - cargo fmt ([`b966cc5`](https://github.com/o2sh/onefetch/commit/b966cc5125ecf2b66a5028d1278c6716e3c20997))
    - add support for Cargo ([`f5ea1f7`](https://github.com/o2sh/onefetch/commit/f5ea1f7c4befcd5d0c9983b887761c24a0fc59de))
    - add support for yarn ([`deefcb5`](https://github.com/o2sh/onefetch/commit/deefcb53631635fa76d0f331d852b5c18ba3a487))
    - rust fmt ([`23042b4`](https://github.com/o2sh/onefetch/commit/23042b48811e54f6b8323c3901cb336b34abac72))
    - Merge branch 'master' of https://github.com/o2sh/onefetch Upstream was ahead of origin, and new .rustfmt.toml was added ([`774d2e5`](https://github.com/o2sh/onefetch/commit/774d2e56e503dc7307a7e2fdee767c1892cd8a3e))
    - update assets ([`8dfca02`](https://github.com/o2sh/onefetch/commit/8dfca024683c08168b15b3d6b2dc771c73549efb))
    - update readme assets ([`04ec670`](https://github.com/o2sh/onefetch/commit/04ec6705554725cb494d53938b8ec879f5959a07))
    - update readme assets ([`532788b`](https://github.com/o2sh/onefetch/commit/532788b8f4404babf584a6d1ff509214351ab538))
    - rust fmt ([`bd98e3a`](https://github.com/o2sh/onefetch/commit/bd98e3a42918e3d9edaef5937bafd3b355075a6f))
    - add support for pip ([`d79f447`](https://github.com/o2sh/onefetch/commit/d79f447a47cbc4e4e7d591471cb217807ffc8827))
    - add support for go modules ([`582b246`](https://github.com/o2sh/onefetch/commit/582b24609fde4a1a568cb85ee60e475d0ea8ede9))
    - add dependency insights field ([`1c62703`](https://github.com/o2sh/onefetch/commit/1c627037f007ab217504d39e89bb076749de8960))
    - editorconfig did stuff ([`230088d`](https://github.com/o2sh/onefetch/commit/230088d5a72010ccf43abea949e136eeb93e839f))
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

 - 145 commits contributed to the release over the course of 15 calendar days.
 - 15 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version to 2.6.0 ([`2c3932c`](https://github.com/o2sh/onefetch/commit/2c3932cc81852ce71f5cf472f9dca12a3703301c))
    - fix colors on java logo and add truecolors to jupyter ([`b55cc16`](https://github.com/o2sh/onefetch/commit/b55cc16295315a386c8fa943c40b493690427dcf))
    - Merge pull request #309 from o2sh/remove_tokio ([`bdd3867`](https://github.com/o2sh/onefetch/commit/bdd386799b24d784c518da71ad016591361dc8f6))
    - remove async/await ([`fad08bd`](https://github.com/o2sh/onefetch/commit/fad08bd68f370db9b1742bbc1a7d606bb6029a4f))
    - Update README.md ([`654b5ef`](https://github.com/o2sh/onefetch/commit/654b5ef059b39962c6fb2e9939f35917884c1a1e))
    - Merge pull request #308 from o2sh/dependabot/cargo/tokio-0.3.3 ([`ef17061`](https://github.com/o2sh/onefetch/commit/ef1706180626eeda241f1df99e02f89a9597637f))
    - fix #307: remove CheckSupportedBackend and Result type for add_image ([`f2bb5b0`](https://github.com/o2sh/onefetch/commit/f2bb5b0f211061bc9059649b0bf2f8a9d8d1ff6b))
    - clean java ascii logo ([`e7c57ad`](https://github.com/o2sh/onefetch/commit/e7c57ade5295437d8f9b1d8bf57e38183e7e1c01))
    - Bump tokio from 0.3.2 to 0.3.3 ([`c1b68d9`](https://github.com/o2sh/onefetch/commit/c1b68d98a84ecae87750d8db0a57ccf6bf81ffd2))
    - fix clippy warnings and rename iterm2 to iterm ([`ce00e62`](https://github.com/o2sh/onefetch/commit/ce00e62a5dcf22623edaf9c5d34a3cf37abd049b))
    - add rustfmt.toml ([`71230ce`](https://github.com/o2sh/onefetch/commit/71230ce111fa2763b268fd39be979099407ed674))
    - Merge pull request #305 from yoichi/iterm2-support ([`492cb5f`](https://github.com/o2sh/onefetch/commit/492cb5fc0b989ce13cb13c763aa108e7b3b7d029))
    - cargo fmt ([`47dbab3`](https://github.com/o2sh/onefetch/commit/47dbab33654a0037db45c63c53683c23e2b822dd))
    - Don't cause error if --image-backend=iterm2 is specified ([`d821bc3`](https://github.com/o2sh/onefetch/commit/d821bc350841b1c6fe3b683ba0eaae5a2b947c80))
    - Fix error message ([`832e4c5`](https://github.com/o2sh/onefetch/commit/832e4c50b1bbcb5441f52882bb19f983bb92cf9c))
    - cargo fmt ([`a0f10cd`](https://github.com/o2sh/onefetch/commit/a0f10cdbb813caf3724ee25b62263007e414268d))
    - Merge pull request #306 from o2sh/dependabot/cargo/regex-1.4.2 ([`8f7dcf6`](https://github.com/o2sh/onefetch/commit/8f7dcf65b91d41709717904b86169c1635607f1e))
    - Bump regex from 1.4.1 to 1.4.2 ([`09ef819`](https://github.com/o2sh/onefetch/commit/09ef819354ff8ae7e58ddc25a6a64d40d956000f))
    - check for KITTY_WINDOW_ID for kitty support ([`b6b5f49`](https://github.com/o2sh/onefetch/commit/b6b5f496fe424e322455b9aadcba324753c00d7e))
    - fix typo ([`6581979`](https://github.com/o2sh/onefetch/commit/65819792268ead7ab5f834bdb8827ea4d322e098))
    - rename dependencies --> deps ([`596f2cf`](https://github.com/o2sh/onefetch/commit/596f2cfb92caa5d8a9b97faeea9b3477bf30d5f8))
    - match on Option after parsing number of deps ([`cd63dae`](https://github.com/o2sh/onefetch/commit/cd63dae64ff5da27089294326bca6c05eb2d8570))
    - fix get_deps() implementation --WIP ([`1c65108`](https://github.com/o2sh/onefetch/commit/1c65108270c195a79b2997c9dcc4e1d6be93a4ef))
    - Add iTerm2 image protocol support ([`9b10bf6`](https://github.com/o2sh/onefetch/commit/9b10bf6e7ff7e53969da14d189b4fcf21ab6d0b5))
    - begin dependency feature ([`5a9ec69`](https://github.com/o2sh/onefetch/commit/5a9ec6946f5ac677fb9a9a027a40b23f276ea6cf))
    - add truecolors to dart and haskell ([`9011596`](https://github.com/o2sh/onefetch/commit/9011596022e797b854b5eeb07b945868078cce8b))
    - clean java ascii logo ([`e92ae3a`](https://github.com/o2sh/onefetch/commit/e92ae3a0a3c5252688ec99d72fa3a6993e258ead))
    - mv snapcraft.yaml into snap folder ([`357efba`](https://github.com/o2sh/onefetch/commit/357efba55d16e1423589607e617fba3d210880af))
    - Merge pull request #303 from Cogitri/raku ([`7a08f9b`](https://github.com/o2sh/onefetch/commit/7a08f9b26fcadf16cbf9d6611531478df5f5f9d1))
    - more cleaning of raku ascii ([`f88e378`](https://github.com/o2sh/onefetch/commit/f88e378eb83598572c45ab68029b42505db2f427))
    - clean raku ascii + use of truecolors ([`b4b4dd1`](https://github.com/o2sh/onefetch/commit/b4b4dd1813232c7f20783c8397c6069042f5260f))
    - add Roku (Perl 6) ASCII logo ([`ebb0229`](https://github.com/o2sh/onefetch/commit/ebb02297832b8ea6f2ea428bc670c5a832b7d79f))
    - Update CONTRIBUTING.md ([`1f4fb64`](https://github.com/o2sh/onefetch/commit/1f4fb64444b8b86f8daf735e2444d3bb31b5bd5c))
    - Update README.md ([`a2a3f78`](https://github.com/o2sh/onefetch/commit/a2a3f7825be92cccc809c925257005ca21caa82a))
    - update go.png asset ([`8fb7b95`](https://github.com/o2sh/onefetch/commit/8fb7b9568ff2e16e49ddb3c1d33b0317f6e543c9))
    - update go.png asset ([`1dd359d`](https://github.com/o2sh/onefetch/commit/1dd359d46809a0a0a7b35ae38ebff95e14fab6f5))
    - update assests ([`c133f7a`](https://github.com/o2sh/onefetch/commit/c133f7a41531de44d3ab78863da79493063a91c9))
    - enable automatic snap publish ([`4d884c2`](https://github.com/o2sh/onefetch/commit/4d884c26614043d84c3695512671f7ba6d2bf158))
    - clean scala ascii logo ([`2685c69`](https://github.com/o2sh/onefetch/commit/2685c6976783d229dae22343d2f0262f797a3d9f))
    - Merge pull request #301 from o2sh/dependabot/cargo/tokio-0.3.2 ([`87da33b`](https://github.com/o2sh/onefetch/commit/87da33b0cc2030e3fb4283ac362acdb2408a8909))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`c04b79b`](https://github.com/o2sh/onefetch/commit/c04b79b24d1d6fad90e526d6c55379aebf283509))
    - improved help section ([`249b5b3`](https://github.com/o2sh/onefetch/commit/249b5b3a32c86d074e074e231aa441d84a05088a))
    - Update CONTRIBUTING.md ([`8ff15ac`](https://github.com/o2sh/onefetch/commit/8ff15ac0c2cd69b1bafc4d1d1f46bd697fd50c62))
    - Bump tokio from 0.3.1 to 0.3.2 ([`8c06232`](https://github.com/o2sh/onefetch/commit/8c06232ecc8804fba4cf52c32e10c54a62987cd2))
    - Merge pull request #300 from o2sh/refacto-info-display ([`690965b`](https://github.com/o2sh/onefetch/commit/690965bac528ce94a75beca472e6b3f7dd2be93b))
    - tuple deconstruction in get_author_field ([`a51b2f7`](https://github.com/o2sh/onefetch/commit/a51b2f7c8248adee664b66f8042408240c97298b))
    - simplify get_formatted_subtitle_label ([`acc68d9`](https://github.com/o2sh/onefetch/commit/acc68d99fcc70069ed75f6cff7324ef10975f45a))
    - improve get_language_field ([`aaa80b1`](https://github.com/o2sh/onefetch/commit/aaa80b131076fc8533f764417fa0dd308806a9bf))
    - refacto info::display ([`5e8867a`](https://github.com/o2sh/onefetch/commit/5e8867aef9618900db45bfb9a6877cbb6137a458))
    - clean c, c++, c# ascii logos ([`6197db5`](https://github.com/o2sh/onefetch/commit/6197db5187160a038db33c1dcd3bbc7ca749ea45))
    - add man page ([`689f936`](https://github.com/o2sh/onefetch/commit/689f936efb210cd8a141eadb6246998e1b407f75))
    - clean rust ascii logo ([`4e8abd3`](https://github.com/o2sh/onefetch/commit/4e8abd3b3e8667160449cbacce3caf43df86b103))
    - clean rust, javascript, typescript ascii logos ([`6c4620a`](https://github.com/o2sh/onefetch/commit/6c4620a3d6fcc3745586ba3436c4e01b9f4ae7a3))
    - Merge pull request #295 from rogercyyu/feature/text-coloring ([`a1cdcad`](https://github.com/o2sh/onefetch/commit/a1cdcad14a6795b3ed0925154876c9caa3b0e8b3))
    - addressed PR changes ([`a23926c`](https://github.com/o2sh/onefetch/commit/a23926c41e6ba24805dc6cc9e9825a577ae94cbf))
    - Merge pull request #297 from o2sh/dependabot/cargo/futures-0.3.7 ([`af02010`](https://github.com/o2sh/onefetch/commit/af02010ebebaa4a5935de2664be26bcfabc26c96))
    - Merge pull request #298 from o2sh/dependabot/cargo/libc-0.2.80 ([`309003e`](https://github.com/o2sh/onefetch/commit/309003e68eff3788ec194f60e63c3722002b759e))
    - Bump futures from 0.3.6 to 0.3.7 ([`11e1243`](https://github.com/o2sh/onefetch/commit/11e1243c975a35e8b6b941cbf844269344595273))
    - Merge pull request #296 from o2sh/dependabot/cargo/image-0.23.11 ([`0df388c`](https://github.com/o2sh/onefetch/commit/0df388c9c9a88f5877f391c76fa0cb982b09c1cf))
    - Merge pull request #299 from o2sh/ci-fail-on-deprecated ([`e14437f`](https://github.com/o2sh/onefetch/commit/e14437f02818a08130c17d49ea6444912ad1c948))
    - rust fmt ([`f6e2430`](https://github.com/o2sh/onefetch/commit/f6e24301fc2cc4f31f638dea99aa5319f67ee53a))
    - fixed incorrect scope for custom_color ([`a8af107`](https://github.com/o2sh/onefetch/commit/a8af107b3e0325972ee8ca62591b0559669edf60))
    - make build fail on deprecated ([`b542311`](https://github.com/o2sh/onefetch/commit/b5423111e7d7934a372131aed72a706e8e046ff9))
    - fixed colors for default use ([`aee65ec`](https://github.com/o2sh/onefetch/commit/aee65ec0e52777832ce271897fe5a0e301d490b7))
    - Use color_quant::NeuQuant ([`e8cfcfe`](https://github.com/o2sh/onefetch/commit/e8cfcfe8112e34f782a4537ff6214762c8744b4a))
    - Update README.md ([`25f85c5`](https://github.com/o2sh/onefetch/commit/25f85c517764a173c93b28d308d09715a17df631))
    - Update README.md ([`f8963f4`](https://github.com/o2sh/onefetch/commit/f8963f4786a4532f0664a28dc1e79e67105f1553))
    - Update README.md ([`6222ce0`](https://github.com/o2sh/onefetch/commit/6222ce040531c6a13b677270ca387ee5b27ad936))
    - Update README.md ([`03d7499`](https://github.com/o2sh/onefetch/commit/03d7499464b80d8b731c6ecbcfba23828b281bd5))
    - Update README.md ([`07d4ab8`](https://github.com/o2sh/onefetch/commit/07d4ab8e7e3b8512fbecdd059a3d45190a6a819d))
    - Update README.md ([`13cc9bf`](https://github.com/o2sh/onefetch/commit/13cc9bf61b0bfa747d9acf746accd4b21b9f3dbb))
    - Update README.md ([`71f04ff`](https://github.com/o2sh/onefetch/commit/71f04ff5c98e0a0360b97e164ac252dde0745759))
    - Update README.md ([`325ed34`](https://github.com/o2sh/onefetch/commit/325ed343b509a75d43ca2005c040aa0bfc2fcb08))
    - Update README.md ([`73d1832`](https://github.com/o2sh/onefetch/commit/73d1832e8ff1caed64383118afaa998a9aeff43e))
    - Update README.md ([`0c2c179`](https://github.com/o2sh/onefetch/commit/0c2c1790178751771cdd548621790d0e3c8120a9))
    - Update README.md ([`ef5ad45`](https://github.com/o2sh/onefetch/commit/ef5ad4505ce5ce48938c076272590dd0f0f662b3))
    - Update README.md ([`14dccc7`](https://github.com/o2sh/onefetch/commit/14dccc7edd4bcdf5f5091ff8c65beab5ed132b62))
    - Update README.md ([`d6b1941`](https://github.com/o2sh/onefetch/commit/d6b19416d1b6dbd45de7a77554eb5918c1dab512))
    - Update README.md ([`8e48514`](https://github.com/o2sh/onefetch/commit/8e48514db59c518b17890dad14beb4d99f81935a))
    - Bump libc from 0.2.79 to 0.2.80 ([`9065f18`](https://github.com/o2sh/onefetch/commit/9065f18c50a348155f153bff10e57703182bd499))
    - Bump image from 0.23.10 to 0.23.11 ([`24a9539`](https://github.com/o2sh/onefetch/commit/24a953962e07eb5cb1389c843a3b1621d23f7b63))
    - cleaner c c++ c# ascii logos ([`c58d6a6`](https://github.com/o2sh/onefetch/commit/c58d6a6211185eae57047c209878bd4c2ecae77f))
    - cleaner c c++ c# ascii logos ([`5cd9558`](https://github.com/o2sh/onefetch/commit/5cd9558bd68e35cd62ab570eb86b0c34cbad7945))
    - fixed color bug in langauges ([`419f917`](https://github.com/o2sh/onefetch/commit/419f917262dd25514a03e3fb1b389b52c84a32b4))
    - Merge branch 'feature/text-coloring' of github.com:rogercyyu/onefetch into feature/text-coloring ([`c34fb6f`](https://github.com/o2sh/onefetch/commit/c34fb6f977befd4dd7089d4c7cc55ba21143f275))
    - Add feature: text coloring ([`f9dac47`](https://github.com/o2sh/onefetch/commit/f9dac4751357cbaeef3fef4b3ae0e0b497b887d5))
    - Merge remote-tracking branch 'upstream/master' into feature/text-coloring ([`813daea`](https://github.com/o2sh/onefetch/commit/813daea0cf934b61c602635b5af65d80f0aecaad))
    - formatting ([`3c89824`](https://github.com/o2sh/onefetch/commit/3c89824e4a10f9939398003a8c0a18ada066480e))
    - initial text coloring for review ([`29d41f8`](https://github.com/o2sh/onefetch/commit/29d41f84f3af9ba4b0e24b3a7c39b8af3f238ed3))
    - initial coloring work ([`50b9808`](https://github.com/o2sh/onefetch/commit/50b98086d818b74ea7b85b162b61c976877a0664))
    - Merge pull request #293 from geeseven/ASCII-script ([`1e29965`](https://github.com/o2sh/onefetch/commit/1e299655fe161f6d5ab4c46ede0f21242fdc654a))
    - Merge pull request #284 from Luke-zhang-04/master ([`0180a36`](https://github.com/o2sh/onefetch/commit/0180a36a15c4695c69b807226055616c475d20cd))
    - Merge pull request #289 from o2sh/remove-panic-from-info-fmt ([`ab2e9f3`](https://github.com/o2sh/onefetch/commit/ab2e9f369a32c532569780a7d8f5994b06fbec4f))
    - Merge branch 'master' of https://github.com/o2sh/onefetch This branch is behind on same changes. ([`c92c3d4`](https://github.com/o2sh/onefetch/commit/c92c3d45983ef4d3b2b2ba63fef210946efaa408))
    - resize ASCII art ([`5bffdc9`](https://github.com/o2sh/onefetch/commit/5bffdc998f25f666ef32cc3c69607a8de8899ecf))
    - make JSX yellow ([`2773c32`](https://github.com/o2sh/onefetch/commit/2773c324a3ef8fe600cea979c3d28a4a2bfe3cf9))
    - Merge branch 'master' into remove-panic-from-info-fmt ([`949db39`](https://github.com/o2sh/onefetch/commit/949db3971a89461ba8f24c76ae4382138a52f7cc))
    - missed end quote ([`e0576af`](https://github.com/o2sh/onefetch/commit/e0576afddadb59af1507b5561ee65c3078934830))
    - exclude tools directory ([`4f1cf4b`](https://github.com/o2sh/onefetch/commit/4f1cf4b79a660197ca7819c3b1141d390ef8d962))
    - Original script from the wiki located at: https://github.com/o2sh/onefetch/wiki/ASCII-Art-From-Image-Using-Python-Image-Library/9c454b390273ffedd60db9d525fb001f89d581b1 ([`6c027e5`](https://github.com/o2sh/onefetch/commit/6c027e5efa7dfbb14108cd55ff0fdbe4dd62b929))
    - add requires param to color-resolution flag ([`2b2aeaa`](https://github.com/o2sh/onefetch/commit/2b2aeaa225b4075b2969db737a416f5e6427d8b8))
    - Merge branch 'master' into remove-panic-from-info-fmt ([`4fcc458`](https://github.com/o2sh/onefetch/commit/4fcc45867a96be8102c64f91113a2c30027cbc40))
    - rust fmt ([`1c07cfb`](https://github.com/o2sh/onefetch/commit/1c07cfb497c690c31ae7f55190eaf5608a2d5494))
    - fix useless break line ([`61596a6`](https://github.com/o2sh/onefetch/commit/61596a6f096395ee8ee4ef80643b9ea8bfd83c9b))
    - better clap arg for --color-resolution ([`42a64dd`](https://github.com/o2sh/onefetch/commit/42a64dd2b6278626856a53c56504fed359dceacf))
    - rust fmt ([`11f86be`](https://github.com/o2sh/onefetch/commit/11f86be42871f91ea68f7b97565f6c60e2998050))
    - Merge pull request #271 from yoichi/sixel-more-color ([`69306bd`](https://github.com/o2sh/onefetch/commit/69306bd1ef30657b18567d8df41ce78a11ee451d))
    - Merge branch 'master' into sixel-more-color ([`3648aaa`](https://github.com/o2sh/onefetch/commit/3648aaa13409eae472837eed7135d5cae3ccbc88))
    - fix --off flag after bad merge ([`c2883d1`](https://github.com/o2sh/onefetch/commit/c2883d1c2de8b31d369d10e8ae9c523bb725543e))
    - Merge pull request #288 from akrantz01/disable-ascii-art ([`522514d`](https://github.com/o2sh/onefetch/commit/522514d04c65c615f5ae458bca94b59202179c45))
    - Merge branch 'master' into disable-ascii-art ([`3bf51c5`](https://github.com/o2sh/onefetch/commit/3bf51c5b3d989803c17177da6eeec0e279a9fe72))
    - fix cargo clippy warn in language.rs ([`0b37765`](https://github.com/o2sh/onefetch/commit/0b37765138a5bd9d2423ae567bd227add7c0e62c))
    - Merge pull request #291 from o2sh/refacto/printer ([`a72dab5`](https://github.com/o2sh/onefetch/commit/a72dab5b32c1ceabf51b1b7553205b6b2481ed69))
    - Update src/onefetch/cli_utils.rs ([`16db642`](https://github.com/o2sh/onefetch/commit/16db642c21f5253543538691ee84b6ef0c6ec071))
    - refacto info.rs by adding printer ([`59a2033`](https://github.com/o2sh/onefetch/commit/59a203348e34ed15661ef39ad4e76820f6b7b1eb))
    - Implement suggestions ([`f705b01`](https://github.com/o2sh/onefetch/commit/f705b010ee3651858ca9351ba44460712dfddc87))
    - fix windows build ([`d8d0ff7`](https://github.com/o2sh/onefetch/commit/d8d0ff78da8e66ca3e9e8e34b4e2861a9e65c75b))
    - rust fmt ([`84895ab`](https://github.com/o2sh/onefetch/commit/84895abe6ca21c34048dab3449d2af0d394ae958))
    - if windows: no supported image backend ([`84cb871`](https://github.com/o2sh/onefetch/commit/84cb871bd72c49f819f9702c8ea335d9798e20bc))
    - return Err when image but no image backend ([`8035b97`](https://github.com/o2sh/onefetch/commit/8035b9758733af51f3a85ecfd4c63ec5ee3230f0))
    - Add CLI option to disable image ([`0d4aa2f`](https://github.com/o2sh/onefetch/commit/0d4aa2f49fd468c8a424f17082e5d81ad871b27f))
    - Merge pull request #286 from o2sh/dependabot/cargo/tokio-0.3.1 ([`fab30cc`](https://github.com/o2sh/onefetch/commit/fab30cc02ede0fc047c1d74cb50770219b19eb75))
    - Merge pull request #285 from o2sh/dependabot/cargo/paste-1.0.2 ([`7a619dd`](https://github.com/o2sh/onefetch/commit/7a619ddc7878e6508b148608eb20384f0ac7f98c))
    - Bump tokio from 0.3.0 to 0.3.1 ([`ed9b602`](https://github.com/o2sh/onefetch/commit/ed9b602b80717b6a946a902728c8bbed69df389c))
    - Bump paste from 1.0.1 to 1.0.2 ([`a75cd27`](https://github.com/o2sh/onefetch/commit/a75cd27a38a667702c7cbdb748454910093b1b86))
    - makes the grammar among the option arg help more consistent ([`0a3eadb`](https://github.com/o2sh/onefetch/commit/0a3eadbee806f6839728df4daef5396d61f69678))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`4c559ab`](https://github.com/o2sh/onefetch/commit/4c559ab1b86c0b1842236f235b11144a29d25fd8))
    - add support for JSX and TSX ([`6d84935`](https://github.com/o2sh/onefetch/commit/6d84935fa2c2b24dfdcbf86981d4d310f3a7e96b))
    - Restrict values of image colors ([`1226801`](https://github.com/o2sh/onefetch/commit/122680113ee60fb529b2c43c31eed92223a95c66))
    - move snap to .gitignore ([`c6908bd`](https://github.com/o2sh/onefetch/commit/c6908bdc38a95c7c4b0465aae68f53a1c60ddc47))
    - Merge pull request #279 from KaindlJulian/ascii-flag ([`1ba074a`](https://github.com/o2sh/onefetch/commit/1ba074a8d58fb4d98c6c94bfc9e692bb0ae62abe))
    - merge declaration of logo_lines with test on ascii_input ([`46dea25`](https://github.com/o2sh/onefetch/commit/46dea2502ebcf6bb32319afd46df591f324e4bcc))
    - add validations and long help for ascii flag ([`cbfe831`](https://github.com/o2sh/onefetch/commit/cbfe8313cfe840432caecbe8f4d559ebb1844d5c))
    - avoid unnecessary call ([`a72b1ae`](https://github.com/o2sh/onefetch/commit/a72b1aeb33a5e3e14eefb4fb7a4acba0a8a32aed))
    - fix go logo and bash ([`ac74497`](https://github.com/o2sh/onefetch/commit/ac74497aadb21a6e21734b2c93f5cdc87f6df94c))
    - Merge pull request #276 from Luke-zhang-04/master ([`0e11538`](https://github.com/o2sh/onefetch/commit/0e11538250cb27bd8050fd0b45ea03e6cc1d91b3))
    - merge with upstream ([`d8506af`](https://github.com/o2sh/onefetch/commit/d8506afc84c712604ba241243ae21eadd49a6a38))
    - make ZSH it's own language and add ASCII ([`6d50660`](https://github.com/o2sh/onefetch/commit/6d50660cfbdb6009787c09d1329b20ac730762ed))
    - Merge pull request #252 from atluft/235-truecolor-define-color ([`7f0c08c`](https://github.com/o2sh/onefetch/commit/7f0c08c8c5374f391819a9d69cbed94cefcdcc24))
    - make BASH it's own language and add ASCII ([`a7b872c`](https://github.com/o2sh/onefetch/commit/a7b872c8595bf400d9df8efc67246ba8f779f018))
    - Add ascii flag ([`2e48d98`](https://github.com/o2sh/onefetch/commit/2e48d98e0923fa0233066c90831937ef40afd401))
    - Merge branch 'master' into 235-truecolor-define-color ([`c465eeb`](https://github.com/o2sh/onefetch/commit/c465eeb02fa5ced3050c7344001c5689e3ad80f7))
    - update README assets ([`b7d51f3`](https://github.com/o2sh/onefetch/commit/b7d51f32cf0068515000d9f33b5b81ac28158a92))
    - update README assets ([`9f70652`](https://github.com/o2sh/onefetch/commit/9f7065255f6dd347ef848b119666b1612c5a66c4))
    - Update src/onefetch/language.rs ([`1d73e42`](https://github.com/o2sh/onefetch/commit/1d73e42c5c9f97eb7d8b2014eb806789c4219c2a))
</details>

## v2.5.0 (2020-10-19)

### New Features

 - <csr-id-aebf7434302d6884532f3427ec6b6021e2fe4adb/> show number of tags and branches
   This simply counts and displays the number of branches and tags that the repository has locally.
 - <csr-id-10fd491eec3eefee0ebcbfaceff95a29783cc192/> add zsh and bash support

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 166 commits contributed to the release over the course of 15 calendar days.
 - 15 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version to 2.5.0 ([`31d11fb`](https://github.com/o2sh/onefetch/commit/31d11fbdfcc78816e1b5d31c59a80b4809c2713c))
    - add zsh and bash support ([`10fd491`](https://github.com/o2sh/onefetch/commit/10fd491eec3eefee0ebcbfaceff95a29783cc192))
    - rust fmt ([`b18d459`](https://github.com/o2sh/onefetch/commit/b18d459d656f5204ff2bdb0dda1d03d819b61c20))
    - add number validator for --authors flag ([`2e6f308`](https://github.com/o2sh/onefetch/commit/2e6f308d02b964c6299023ad06064fd0dc8a1b99))
    - Merge pull request #268 from nguyenquannnn/master ([`b4cf5ec`](https://github.com/o2sh/onefetch/commit/b4cf5eca043f04f9bfabec70fd521c92195dcb2e))
    - #235 accepted all review suggestions ([`36a9e9c`](https://github.com/o2sh/onefetch/commit/36a9e9c0f28905efc24f375c45bdb778c6a1c708))
    - Merge pull request #270 from yoichi/fix-layout-with-sixel ([`37e6fa3`](https://github.com/o2sh/onefetch/commit/37e6fa34e80a15a65fe88c025f0172fd835a2d53))
    - fix nose on GO ASCII logo ([`8f79979`](https://github.com/o2sh/onefetch/commit/8f79979a115dd9ca92c719496de27ffef891def2))
    - add third color for Go ASCII logo ([`f2a5767`](https://github.com/o2sh/onefetch/commit/f2a5767979b5474d7b6f467fc196e1e0964e5eb9))
    - 233 Update Go ascii art ([`8e24d6e`](https://github.com/o2sh/onefetch/commit/8e24d6e0e3587d607f119525042626b9218eae3d))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`6aea8a1`](https://github.com/o2sh/onefetch/commit/6aea8a1af71185b9a708f25093b37518545553b9))
    - 233 Update Go ascii art ([`2ad5ca7`](https://github.com/o2sh/onefetch/commit/2ad5ca7736b4d191625ec94e6c43640f648e5cae))
    - New option --image-colors to specify colors used in image backends ([`83459a8`](https://github.com/o2sh/onefetch/commit/83459a886530e42ff488aed191c823735dfd782f))
    - Update README.md ([`2ee7b54`](https://github.com/o2sh/onefetch/commit/2ee7b541873682f140a3f491ae8d47ebfb837978))
    - Update README.md ([`3f3a245`](https://github.com/o2sh/onefetch/commit/3f3a2455eb0d5db897ff1eddf871100730346f66))
    - move is_true_color_terminal() in cli.rs ([`e8fdd42`](https://github.com/o2sh/onefetch/commit/e8fdd422cdf503a3e4cfebddd32500a0e0c746ac))
    - Update README.md ([`150ddd5`](https://github.com/o2sh/onefetch/commit/150ddd5427b53897558a613f41639d62d99e056e))
    - Update README.md ([`ac98bb2`](https://github.com/o2sh/onefetch/commit/ac98bb28a03dea0100091e15efc907b3eb9370e5))
    - Update README.md ([`5576651`](https://github.com/o2sh/onefetch/commit/5576651649e324d95cf72e7a924758c9b7355e4f))
    - Update README.md ([`8f3e073`](https://github.com/o2sh/onefetch/commit/8f3e073e0f6099bd868e34c02af6689e8e955031))
    - Update README.md ([`5c7f668`](https://github.com/o2sh/onefetch/commit/5c7f6680c589aa2a1213a8765c64d3146543018c))
    - Update README.md ([`6dfe307`](https://github.com/o2sh/onefetch/commit/6dfe307c1dd0ede4228ab44a4c96098a891d3844))
    - #235 moves use into test module for build ([`e667b23`](https://github.com/o2sh/onefetch/commit/e667b2320648a35f31651cb38b5e0ca4f0bc19e5))
    - #235 adds tests for equal array size and basic colors ([`fca7e74`](https://github.com/o2sh/onefetch/commit/fca7e7461ac7802241f8ead78b2a1b2463d51d08))
    - Use more color (16 -> 128) in sixel graphics ([`7ccc949`](https://github.com/o2sh/onefetch/commit/7ccc9493fcc5d25028a310f52b32945693ff0440))
    - Avoid calculation mistakes in full screen display ([`ce36353`](https://github.com/o2sh/onefetch/commit/ce36353bfce4a5ebb42234cd99213066317d6d49))
    - Avoid moving too much upwards ([`c370cb4`](https://github.com/o2sh/onefetch/commit/c370cb4139a6aeca8834798208a0ec36250c06ce))
    - trim ocaml ASCII logo ([`34d4743`](https://github.com/o2sh/onefetch/commit/34d474356a1a48c72697311e36fa58b06fff2670))
    - Merge pull request #269 from o2sh/dependabot/cargo/tokio-0.3.0 ([`ac445b2`](https://github.com/o2sh/onefetch/commit/ac445b21ee78979ca0e4a54bbf4c75ace186090f))
    - refacto info.rs ([`8bcab7b`](https://github.com/o2sh/onefetch/commit/8bcab7b09dc1dc4612bfdad15446cd06835e15d6))
    - remove future::join! inside get_number_of_tags_branches + exclude non async func from future::join! #269 ([`43db0dc`](https://github.com/o2sh/onefetch/commit/43db0dc95064dda88a13f02053586f0a27ccc6a0))
    - Merge pull request #267 from o2sh/enable-ascii-size-test ([`69e6452`](https://github.com/o2sh/onefetch/commit/69e6452a0b9d9c2d1773e0037037a33482d92d37))
    - Bump tokio from 0.2.22 to 0.3.0 ([`b8bb67d`](https://github.com/o2sh/onefetch/commit/b8bb67dd0630452f3650ce10ba6d7365a7fba12a))
    - #235 fix ([`410b849`](https://github.com/o2sh/onefetch/commit/410b849e4bc3069457cea7ba4a5f3c1cb87cd8b1))
    - #235 improving colorterm test by separating into a function ([`c0735e1`](https://github.com/o2sh/onefetch/commit/c0735e1b38135ca764aa739799d2d32387db75f8))
    - #235 define colors from define languages invocation ([`074e238`](https://github.com/o2sh/onefetch/commit/074e238026c7fa5a96b957005c49e628865f682d))
    - \#233 Update Go ascii art ([`6b300af`](https://github.com/o2sh/onefetch/commit/6b300af8a5bb348ab822fa02ef366f789e566c19))
    - Merge pull request #266 from yoichi/enable-image-backend-on-macos ([`792850e`](https://github.com/o2sh/onefetch/commit/792850eb6cb30847dc507d17ed4f827db1c38093))
    - Replace condition in Cargo.toml: target_os = "windows" -> windows ([`fdffb65`](https://github.com/o2sh/onefetch/commit/fdffb6584fc6dc77dcc91cf53390a3a952bac14b))
    - Replace condition: target_os = "windows" -> windows ([`0cf7398`](https://github.com/o2sh/onefetch/commit/0cf7398bdb84dd281d888f970e88a47ace4bbebf))
    - enable-ascii-size-test ([`6f638a0`](https://github.com/o2sh/onefetch/commit/6f638a02b5cee900bd1963075f9683ad0b877950))
    - reduce size of Prolog ASCII logo, fix #261 ([`85fc5e3`](https://github.com/o2sh/onefetch/commit/85fc5e3eb1d1c952ce8f015aff1f6d890ab0e292))
    - improve ASCII logo for javascript, typescript, lisp, python ([`49d11f6`](https://github.com/o2sh/onefetch/commit/49d11f6d3d541821b0ca8bb0f94df978fdcc02aa))
    - reduce size of groovy ASCII logo fix #257 ([`46ae4b9`](https://github.com/o2sh/onefetch/commit/46ae4b931ccca93df8b017c6205ebf5a01014415))
    - reduce size of forth ASCII logo #249 ([`4867f2c`](https://github.com/o2sh/onefetch/commit/4867f2cdc904e1e8b120ed3dcdabb3baca46b995))
    - Merge pull request #265 from o2sh/fix/markdown-jupyter-stats ([`2acd7e9`](https://github.com/o2sh/onefetch/commit/2acd7e93d219ffebf20aa19eaf6147e262b2db80))
    - refacto total == 0 condition ([`9bd6e89`](https://github.com/o2sh/onefetch/commit/9bd6e89ceac828d4c66d2fd971ee5cf2cc670105))
    - Enable image backends on macOS ([`f82011e`](https://github.com/o2sh/onefetch/commit/f82011e76af9210c1c55b7e202df25eb22389942))
    - small refacto ([`149ede0`](https://github.com/o2sh/onefetch/commit/149ede083b7926c0df4b43be72142f343ff2efaa))
    - fix calculation of language distribution ([`e41f33e`](https://github.com/o2sh/onefetch/commit/e41f33ed077cdfa0ac9f42044d70c65163452047))
    - takes into account language.children ([`0a39044`](https://github.com/o2sh/onefetch/commit/0a390442d523e091f905709709dd9255a4dcca58))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`37379a5`](https://github.com/o2sh/onefetch/commit/37379a5c8760905948b56840889808a21af0c34c))
    - renable -l flag ([`462e63a`](https://github.com/o2sh/onefetch/commit/462e63a8f9c5ea1b14d5e42985c6a7e3f647b58e))
    - Merge pull request #263 from o2sh/dependabot/cargo/regex-1.4.1 ([`9e89677`](https://github.com/o2sh/onefetch/commit/9e896775093116c76e3714c498f3e8e4df856058))
    - Merge pull request #264 from o2sh/dependabot/cargo/git2-0.13.12 ([`619fecb`](https://github.com/o2sh/onefetch/commit/619fecb34208795552f15809a2bc6d12e1fb9247))
    - remove empty lines from Rust ASCII logo #234 ([`6c8ffad`](https://github.com/o2sh/onefetch/commit/6c8ffadb1bb71634270cdf5fd8af2a9df93761a4))
    - Merge pull request #262 from nguyenquannnn/master ([`fdb6729`](https://github.com/o2sh/onefetch/commit/fdb6729a246451fbe549ee2a1501b3d9fcdb47e9))
    - Bump git2 from 0.13.11 to 0.13.12 ([`76f713c`](https://github.com/o2sh/onefetch/commit/76f713c3506e6ab253d1f67e2dce3cd4a74398a0))
    - Bump regex from 1.4.0 to 1.4.1 ([`a527545`](https://github.com/o2sh/onefetch/commit/a527545033ea33c40bfc4137e2c1c94c8146d549))
    - 234 Update Rust ascii art ([`d9d9a87`](https://github.com/o2sh/onefetch/commit/d9d9a877f373cc312472f1c566e7f0fb5e73de32))
    - Merge pull request #260 from o2sh/chore/dependabot/check-actions ([`072c421`](https://github.com/o2sh/onefetch/commit/072c421152b80b91ffcafeb2d8587808d286c08f))
    - Merge pull request #253 from o2sh/refacto-main ([`a2b0bfd`](https://github.com/o2sh/onefetch/commit/a2b0bfd5112a7baad0fa1ab449325231eaba9188))
    - use CARGO_MANIFEST_DIR instead of relative path ([`a8f345d`](https://github.com/o2sh/onefetch/commit/a8f345d4092e5f234e8b821d1b3322a2c6b5a00e))
    - Use actions/checkout v2 for Rustfmt ([`d98485d`](https://github.com/o2sh/onefetch/commit/d98485d0d3174b9c1ab6dad3fa5f6f42f04bd4bc))
    - Check GitHub Actions versions ([`52a4848`](https://github.com/o2sh/onefetch/commit/52a484817f024a2941dd7416c0f850439df45fb5))
    - simplify no_bold logic ([`be96551`](https://github.com/o2sh/onefetch/commit/be96551501d4db0324ee937aa827e7663ecaa532))
    - merge Options with cli ([`8419edd`](https://github.com/o2sh/onefetch/commit/8419eddffe62c97fd3ed6d17b4a51dd55c890aee))
    - Merge pull request #259 from o2sh/dependabot/cargo/regex-1.4.0 ([`784b7a2`](https://github.com/o2sh/onefetch/commit/784b7a28fdb37785fa80ab3e0b6b7d0d8c59976c))
    - Bump regex from 1.3.9 to 1.4.0 ([`d7d3970`](https://github.com/o2sh/onefetch/commit/d7d397058a08465451b2654a3f706842b1eac69b))
    - extract info_fields into its own module ([`5e595a5`](https://github.com/o2sh/onefetch/commit/5e595a5ddfea01ca850c58d8bb60eebaaa379d63))
    - fix possible values for -a flag ([`4d69c11`](https://github.com/o2sh/onefetch/commit/4d69c11a3d028c14adf27db89f5a4c94ec280bfd))
    - rearrange files ([`7fd3b56`](https://github.com/o2sh/onefetch/commit/7fd3b565462799555f1771f0df47304aceee11f7))
    - rustfmt ([`f9e86a0`](https://github.com/o2sh/onefetch/commit/f9e86a0be402dfbb489e142a0b7509353abb42c0))
    - use error_chain for error handling ([`444f3b2`](https://github.com/o2sh/onefetch/commit/444f3b299842afa2b0c20b085c5aee7c51503108))
    - reduce size of d ASCII Logo ([`743de8a`](https://github.com/o2sh/onefetch/commit/743de8afebe8b1a73b0e5712c45f5a3788ae11c9))
    - fix error handling ([`24a3666`](https://github.com/o2sh/onefetch/commit/24a36668790b8a052c403cd4ba9b42d8ea426051))
    - try fix build ([`a2350af`](https://github.com/o2sh/onefetch/commit/a2350af350b1280daed3adfc3e7217dabfd94403))
    - further refactoring of main.rs ([`2b9f425`](https://github.com/o2sh/onefetch/commit/2b9f425bdaa5556d72909119c9b70bf6e1d4dd12))
    - refacto declaration of option struct ([`71787c3`](https://github.com/o2sh/onefetch/commit/71787c3883e33d44ce46d194b4eac6c3832e1f46))
    - Merge pull request #254 from tianlangwu/issue-250 ([`0630765`](https://github.com/o2sh/onefetch/commit/0630765b31ddb7c59ad8074e7b234aa2ed0f3d84))
    - add possible values for ascii_colors ([`038b3c2`](https://github.com/o2sh/onefetch/commit/038b3c2d501a25604354bca5b336f2ac553a60ba))
    - Reduce size of Nim's ASCII logo ([`41ad627`](https://github.com/o2sh/onefetch/commit/41ad627e6e59eaa2e9b66ca496b835805ac6c40d))
    - split/refacto main module ([`62697c4`](https://github.com/o2sh/onefetch/commit/62697c41aeba26bbcfa60eb593164c52b0403aa5))
    - Trigger CI on PR #252 ([`7608f92`](https://github.com/o2sh/onefetch/commit/7608f929de7097c64bf700295640ab8f7f6919e6))
    - #235 true colors using color define macro ([`bee2bf7`](https://github.com/o2sh/onefetch/commit/bee2bf7957e05932405e20a68239fe9634163c2b))
    - Merge pull request #246 from o2sh/hotfix/fix-number-of-branches ([`e334176`](https://github.com/o2sh/onefetch/commit/e3341760b9e2c93a1d0f3ad1e02ffbc57e0e0062))
    - fix underflow when no remote branch ([`fa63290`](https://github.com/o2sh/onefetch/commit/fa63290295548565bcb77da2cd020239ba971020))
    - Merge pull request #245 from o2sh/hotfix/remote-branch ([`ec8a9e4`](https://github.com/o2sh/onefetch/commit/ec8a9e43c19bd10d14053d28168ffda92dba2a2c))
    - Match on literal . ([`75def6c`](https://github.com/o2sh/onefetch/commit/75def6c74e0a2e0595e78f776a7a694e887690b7))
    - Merge pull request #248 from maash3r/ascii/zig ([`fc718b9`](https://github.com/o2sh/onefetch/commit/fc718b9680eae885383335df124d51ccf30df767))
    - Updated Zig's ASCII logo ([`5dd6778`](https://github.com/o2sh/onefetch/commit/5dd67785f37103f26ee45f1e35e8855924d732f0))
    - Merge pull request #247 from maash3r/ascii/tex ([`78f8c90`](https://github.com/o2sh/onefetch/commit/78f8c906b387ee1b593fa05b61c49eff3274e846))
    - Updated Tex's ASCII logo ([`857bdee`](https://github.com/o2sh/onefetch/commit/857bdee3e6f61bb269c6ebfc90ea2377b4bb1d33))
    - #235 true color using define_colors macro from define_language ([`56dd1c1`](https://github.com/o2sh/onefetch/commit/56dd1c178b3c86af5f3bbe91b08d95518256ce5a))
    - switch order tags/branches ([`5afe7b2`](https://github.com/o2sh/onefetch/commit/5afe7b245b3caa476f9499e0e9b955033017eb70))
    - fix number of branches to ignore HEAD ([`cc122c4`](https://github.com/o2sh/onefetch/commit/cc122c41cb5c49391e63f2a2c1ff90cb41000d9a))
    - Regex matching for remote.url ([`c94ebe9`](https://github.com/o2sh/onefetch/commit/c94ebe9737d18b73e34e82026b9fd48c56a3174a))
    - Merge pull request #244 from o2sh/chore/cargo-update ([`239b73b`](https://github.com/o2sh/onefetch/commit/239b73b2618bed87ab9782d57984c7dc154d4e33))
    - Run cargo update ([`a0b0ebc`](https://github.com/o2sh/onefetch/commit/a0b0ebc41630b0ee0de995610279c7ffd3f46424))
    - fix rust fmt ([`e710018`](https://github.com/o2sh/onefetch/commit/e710018c0badb98386cef567226df3f42771d963))
    - Merge pull request #243 from yoichi/identify-author-by-email ([`ca58871`](https://github.com/o2sh/onefetch/commit/ca588716efdd4c73151b18ad6a230fa766422910))
    - Clarify variable names ([`c68e021`](https://github.com/o2sh/onefetch/commit/c68e021046d468777cb424dbad36ad40e880307d))
    - Identify author by email ([`5044179`](https://github.com/o2sh/onefetch/commit/5044179251365426b3cb28465ee160ee94ef0821))
    - small refactoring of #237 ([`13b9727`](https://github.com/o2sh/onefetch/commit/13b9727814d575456b374f3142c72dae2fd31200))
    - Merge pull request #237 from reime005/feat-show-tags-branches ([`55d0498`](https://github.com/o2sh/onefetch/commit/55d0498f5d2709e54cc37fcf0e6250373d28b746))
    - show number of tags and branches ([`aebf743`](https://github.com/o2sh/onefetch/commit/aebf7434302d6884532f3427ec6b6021e2fe4adb))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`0319a54`](https://github.com/o2sh/onefetch/commit/0319a54d781afc37ca3afa688f6a5119ae56ff24))
    - resize r.png ([`f7fd680`](https://github.com/o2sh/onefetch/commit/f7fd6807baa613305bfea0d6f11df6ab3c67690d))
    - Merge pull request #241 from CodeLongAndProsper90/patch-1 ([`425d80f`](https://github.com/o2sh/onefetch/commit/425d80f4c101db616d8ca28fa5bdb029a8e501d8))
    - Fix grammar error in README.md ([`c872763`](https://github.com/o2sh/onefetch/commit/c8727637e72e6bd54c0b455d520d386fc4fa356a))
    - Merge pull request #238 from o2sh/dependabot/cargo/strum-0.19.5 ([`b0fb294`](https://github.com/o2sh/onefetch/commit/b0fb2940fe84a2cf892c629be9a56eafa8e5ac55))
    - update README assets ([`0280ee6`](https://github.com/o2sh/onefetch/commit/0280ee6c3a71063d1235716b1aaa3ed3cebc3f4b))
    - reduce to 2 languages per line ([`812283b`](https://github.com/o2sh/onefetch/commit/812283b4a41431d3639f19f148f5505e4446535c))
    - Bump strum from 0.19.2 to 0.19.5 ([`5ea7aa4`](https://github.com/o2sh/onefetch/commit/5ea7aa40cc1a9beeff3a9e1cfc3c9ae36b83a09d))
    - white spheres in jupyter-notebook ASCII logo ([`020409b`](https://github.com/o2sh/onefetch/commit/020409b2aa541de7a9e044403d559a48f4d252d8))
    - Merge pull request #231 from maash3r/ascii/jupyter ([`db92791`](https://github.com/o2sh/onefetch/commit/db9279138a28f2c23fa2375f4822e275fcd912d9))
    - resize r.png ([`9ab044a`](https://github.com/o2sh/onefetch/commit/9ab044a55a6d1996f87b4273390cc01e3a60b5b4))
    - resize r.png ([`7d37179`](https://github.com/o2sh/onefetch/commit/7d371797432395b0285fc43f5b983cb92572d271))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`d88253f`](https://github.com/o2sh/onefetch/commit/d88253f5abc9eac2e1f7205e34408863b6ac70c8))
    - resize r.png ([`3c68c12`](https://github.com/o2sh/onefetch/commit/3c68c12f4f5470713376a096957bc36febfd7e40))
    - Update README.md ([`4414d24`](https://github.com/o2sh/onefetch/commit/4414d2420f83340db3e3bc3a63d7a730be599c82))
    - update README assets ([`ac89736`](https://github.com/o2sh/onefetch/commit/ac8973669c9a83a83f227c6dc6c812f0b8fa6c72))
    - update README assets ([`5a49684`](https://github.com/o2sh/onefetch/commit/5a4968481c7929071529e76d63d6e085711a19bd))
    - Changed jupyter ASCII color from black to blue ([`920958d`](https://github.com/o2sh/onefetch/commit/920958dcb7c252402ac8ba297ae3b9fdd91a5275))
    - Merge pull request #232 from rootEnginear/master ([`1f09bcc`](https://github.com/o2sh/onefetch/commit/1f09bcc8cde94d58bbf8e5755fda6766d4b76f6e))
    - Merge pull request #229 from atluft/222-new-objective-c-logo ([`120200d`](https://github.com/o2sh/onefetch/commit/120200dc21d7ce38cf46ee741aba834bb64859c8))
    - fixes #222 reduced width ([`9e9b610`](https://github.com/o2sh/onefetch/commit/9e9b610823ae37bc3482734061c2809cd445c726))
    - Remove escaping backslash ([`9fbd1f9`](https://github.com/o2sh/onefetch/commit/9fbd1f9232d6ac6108698e10652e5e0caec72db0))
    - Edit ASCII modifier, Update PHP pallette ([`b194754`](https://github.com/o2sh/onefetch/commit/b194754a58e12d73ddbc8d8dd171243e643d15fa))
    - Change PHP logo ([`802d883`](https://github.com/o2sh/onefetch/commit/802d8835f681dcbf3ee37ab38d6ceeaf1a0ddf5d))
    - Merge pull request #230 from o2sh/dependabot/cargo/futures-0.3.6 ([`33ea0a2`](https://github.com/o2sh/onefetch/commit/33ea0a2c29d6e642e03432bedbb79162aa235f64))
    - Fixed color for Jupyter in language.rs ([`8af3fd3`](https://github.com/o2sh/onefetch/commit/8af3fd34b5c67cf866701d2b2a71d83e3f086a22))
    - Updated colors for Jupyter in language.rs ([`143d066`](https://github.com/o2sh/onefetch/commit/143d06669b46829ec87631ef2366d0a45a42a24e))
    - Changed Jupyter ASCII ([`f8331f4`](https://github.com/o2sh/onefetch/commit/f8331f425b5bfa117eb45f877108396020327fdc))
    - Bump futures from 0.3.5 to 0.3.6 ([`e531b24`](https://github.com/o2sh/onefetch/commit/e531b24dbba79ca5cb166c362d2249924dd610f4))
    - fixes #222 new objective-c logo ([`14c48fa`](https://github.com/o2sh/onefetch/commit/14c48fa417445d193cee4c682e2d4d64e5b5ac8b))
    - fixes #222 ([`450f1ff`](https://github.com/o2sh/onefetch/commit/450f1fffeede5354660c6cc4ba25bef1ca7ca912))
    - Merge pull request #227 from maash3r/ascii/clojure ([`d626b7d`](https://github.com/o2sh/onefetch/commit/d626b7db9518c1efa9225cc734e5c2fb18733981))
    - replace png logo with svg ([`c65be54`](https://github.com/o2sh/onefetch/commit/c65be54102cf1bcd7cff144bdf6b4a546a83fc32))
    - Updated clojure ascii ([`97dc33e`](https://github.com/o2sh/onefetch/commit/97dc33e4707cbcdcdff820a472728145f1e1c979))
    - replace png logo with svg ([`5221c9c`](https://github.com/o2sh/onefetch/commit/5221c9c08418caae530cbee742e5ca86a03b18bf))
    - replace png logo with svg ([`d6eaec2`](https://github.com/o2sh/onefetch/commit/d6eaec25d0049f6892aa4f6cd8f6abea7ed99f76))
    - crop onefetch logo ([`74b4e07`](https://github.com/o2sh/onefetch/commit/74b4e07244efb6fdcacf2a8f1c8e1b2a2132afba))
    - Update README.md ([`f1248c7`](https://github.com/o2sh/onefetch/commit/f1248c7710f056c2e6e17dca81da57007a55bc32))
    - Update README.md ([`8ebb1e8`](https://github.com/o2sh/onefetch/commit/8ebb1e8a251d41d79327f6188d67e2f967f917b2))
    - Update README.md ([`261b09c`](https://github.com/o2sh/onefetch/commit/261b09c71fe71bbf619147bfccf84469c15989e6))
    - Update README.md ([`f2d809c`](https://github.com/o2sh/onefetch/commit/f2d809c13de6eba20e28d35df22b611f94469f69))
    - Update README.md ([`c5f9ece`](https://github.com/o2sh/onefetch/commit/c5f9ece7fdb0bc1af16759b0ec7cecbc638806ed))
    - Update README.md ([`edfecff`](https://github.com/o2sh/onefetch/commit/edfecff65498433755235c793cc95508555ffdc5))
    - Merge pull request #224 from atluft/220-new-swift-logo ([`ca72cf9`](https://github.com/o2sh/onefetch/commit/ca72cf9a8945cbdadba8aa4e793245d7fb46dd87))
    - improves #220 by adding link to wiki article about python tool ([`cfbd0e2`](https://github.com/o2sh/onefetch/commit/cfbd0e26e2136ccf37109af7e8b9f0fa70c9a633))
    - less bulky swift logo improves #220 ([`3aeba48`](https://github.com/o2sh/onefetch/commit/3aeba48e4a5e9762f2bc01521c151adaa41507d3))
    - refacto get_repo_name function ([`2719c63`](https://github.com/o2sh/onefetch/commit/2719c6373fc1d9979f99335bd03224ba0795d237))
    - refacto get_repo_name function ([`87453b2`](https://github.com/o2sh/onefetch/commit/87453b28e7713c3494a4a6bf124192dfe9a50093))
    - Merge pull request #225 from o2sh/dependabot/cargo/libc-0.2.79 ([`aadd475`](https://github.com/o2sh/onefetch/commit/aadd475c33f18124894f84c3f88257dd40259b09))
    - Bump libc from 0.2.78 to 0.2.79 ([`ea52652`](https://github.com/o2sh/onefetch/commit/ea526520ea57bedbf29367744cd5d4f04c94c086))
    - fixes #220 ([`3b69250`](https://github.com/o2sh/onefetch/commit/3b692502109b4003538ccc565282a6796970b3d1))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`e40f3f0`](https://github.com/o2sh/onefetch/commit/e40f3f08d3a7604c662edc171b7e00304cdecec7))
    - fix get_repo_name when cloning from url that ends with forward slash ([`82cc9a2`](https://github.com/o2sh/onefetch/commit/82cc9a29d7fface8c828f4bef81d1f58b1ab2dd9))
    - Update README.md ([`7eb9d3d`](https://github.com/o2sh/onefetch/commit/7eb9d3dff56f2d2e2fdc100f821161cc30234ca1))
    - trim ASCII logo lines before checking for max width ([`8c9ccca`](https://github.com/o2sh/onefetch/commit/8c9ccca614092847df4ce4ca0bf092749e139b95))
    - Merge pull request #218 from maash3r/asciis ([`064f7e1`](https://github.com/o2sh/onefetch/commit/064f7e117303406aac9b269e47d5bb9cb2ab689b))
    - Updated language.rs for Lua having 2 colors ([`b2fb340`](https://github.com/o2sh/onefetch/commit/b2fb34017a16ad61f306baa5497e3b28aa2d1412))
    - Changed to have dual color, fixed '2' ([`4f44f62`](https://github.com/o2sh/onefetch/commit/4f44f6283b651b9f3aa752c1d004b165667e80ed))
    - Changed to a smaller Lua logo ([`f4d2696`](https://github.com/o2sh/onefetch/commit/f4d2696dce079798ccbd3508ca6cfc679b65fbf3))
    - Update README.md ([`4e58a92`](https://github.com/o2sh/onefetch/commit/4e58a921634460b6038b21d88b7867518207ca21))
</details>

## v2.4.0 (2020-10-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 91 commits contributed to the release over the course of 61 calendar days.
 - 61 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#189](https://github.com/o2sh/onefetch/issues/189), [#201](https://github.com/o2sh/onefetch/issues/201), [#210](https://github.com/o2sh/onefetch/issues/210)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#189](https://github.com/o2sh/onefetch/issues/189)**
    - Bump base64 from 0.11.0 to 0.12.3 ([`0e2aebf`](https://github.com/o2sh/onefetch/commit/0e2aebf937de9b77c725520b1745870166253a8b))
 * **[#201](https://github.com/o2sh/onefetch/issues/201)**
    - Bump image from 0.23.8 to 0.23.9 ([`cac47ca`](https://github.com/o2sh/onefetch/commit/cac47ca1068b3194f52dbf978aaf698ae3a6e06f))
 * **[#210](https://github.com/o2sh/onefetch/issues/210)**
    - Bump paste from 1.0.0 to 1.0.1 ([`235aa70`](https://github.com/o2sh/onefetch/commit/235aa7030ea66506152e599df85ba1447263ac53))
 * **Uncategorized**
    - bump onefetch version ([`d14606a`](https://github.com/o2sh/onefetch/commit/d14606af201494003f7186751d9796cdcdf880b9))
    - improved cd action ([`264267c`](https://github.com/o2sh/onefetch/commit/264267ca1420ee3b7b50b1cb094803358e6e2330))
    - Merge pull request #215 from o2sh/dependabot/cargo/libc-0.2.78 ([`2687cb9`](https://github.com/o2sh/onefetch/commit/2687cb9971b1572255f709e318fbcab9f2759e25))
    - Bump libc from 0.2.77 to 0.2.78 ([`3103f7b`](https://github.com/o2sh/onefetch/commit/3103f7b7cc6873b70b51e778b93de7ad6239e003))
    - Merge pull request #216 from MarkusPettersson98/lang/crystal ([`879e57c`](https://github.com/o2sh/onefetch/commit/879e57c1cbc33307345d26a6f9c6d20d77eb6ce1))
    - Inverse black and white in Crystal ascii art Increase readability in a black terminal. ([`9927d95`](https://github.com/o2sh/onefetch/commit/9927d95b63c795db96576391db8182c11eaa06a3))
    - Merge pull request #214 from o2sh/dependabot/cargo/base64-0.13.0 ([`e087fb7`](https://github.com/o2sh/onefetch/commit/e087fb76fce404c7a45d508f3862575d8a3b0eab))
    - Add support for the Crystal programming language. ([`a6665f6`](https://github.com/o2sh/onefetch/commit/a6665f60565b7b3e351cab248111d5eb0e4e9c7d))
    - Bump base64 from 0.12.3 to 0.13.0 ([`6116f73`](https://github.com/o2sh/onefetch/commit/6116f73f7dbbe7d54b6ee8575056ff1fbe5037d7))
    - improved ci ([`a85a907`](https://github.com/o2sh/onefetch/commit/a85a907058d0235dd68bf67cf633d50e0c227de2))
    - Merge pull request #213 from o2sh/dependabot/cargo/image-0.23.10 ([`f035e02`](https://github.com/o2sh/onefetch/commit/f035e02a5385bb4a37417379497f8d3fbea05df3))
    - Merge pull request #212 from o2sh/dependabot/cargo/askalono-0.4.3 ([`409d8b3`](https://github.com/o2sh/onefetch/commit/409d8b3b0e0aefb47661b228acbf2304b50876e3))
    - Bump image from 0.23.9 to 0.23.10 ([`316c16a`](https://github.com/o2sh/onefetch/commit/316c16a5abdb528c2ffbdd3a26aeb52625bcfe45))
    - Bump askalono from 0.4.2 to 0.4.3 ([`106d301`](https://github.com/o2sh/onefetch/commit/106d3011bc5805f8481d2504ae5add94a350274e))
    - better naming for some functions ([`96ef592`](https://github.com/o2sh/onefetch/commit/96ef592df952a2a280b209e136da875bce042cc7))
    - Merge pull request #211 from o2sh/feature/single-git-log ([`5482a10`](https://github.com/o2sh/onefetch/commit/5482a105eb8deb2f1fb08f90b742f905646526aa))
    - Apply suggestions by spenserblack ([`ede6c92`](https://github.com/o2sh/onefetch/commit/ede6c92b31166c721b8b101f687f1719a5763706))
    - re-use git history in get_last_change() ([`91ce698`](https://github.com/o2sh/onefetch/commit/91ce69838af0f721e45028bb2ad357aa35b5415c))
    - use git log once ([`f3d7335`](https://github.com/o2sh/onefetch/commit/f3d733577da0709e63b62cdc72980f709a2900b9))
    - Merge pull request #208 from o2sh/dependabot/cargo/git2-0.13.11 ([`661d70a`](https://github.com/o2sh/onefetch/commit/661d70aa65ade0827921d09362f5613754b4e5a5))
    - Bump git2 from 0.13.10 to 0.13.11 ([`c695bdf`](https://github.com/o2sh/onefetch/commit/c695bdf5b58b0f9d46422caca9cf6648b73f833c))
    - Merge pull request #209 from o2sh/dependabot/cargo/libc-0.2.77 ([`b9cb8c8`](https://github.com/o2sh/onefetch/commit/b9cb8c80982756f9f9d409a1cc81b55feeed64c2))
    - Bump libc from 0.2.76 to 0.2.77 ([`cab4066`](https://github.com/o2sh/onefetch/commit/cab40666ff72527f6cdad39443d7e803d8d89216))
    - Update CONTRIBUTING.md ([`310fc39`](https://github.com/o2sh/onefetch/commit/310fc39facc8df5c5600021e73e527a6e954fcff))
    - Update CONTRIBUTING.md ([`bec4194`](https://github.com/o2sh/onefetch/commit/bec419454be1211e7b232c9c57a38663c73be295))
    - Merge pull request #206 from o2sh/feature/language-def-macro ([`7cafe85`](https://github.com/o2sh/onefetch/commit/7cafe853e6eeffb6c6ff12dfd162e4f38ec1051a))
    - cargo fmt ([`5d438f8`](https://github.com/o2sh/onefetch/commit/5d438f81fda7ade99c6d8fbbf197f1aa70e01117))
    - Resolve Cargo.lock conflict ([`ba8a001`](https://github.com/o2sh/onefetch/commit/ba8a001c0b0743f54109c0b8ca28b60c2f0c4237))
    - Ignore ASCII size assertions by default ([`152678c`](https://github.com/o2sh/onefetch/commit/152678c4c53ac193db7ccf99683a0a8379a67ab8))
    - Enforce ASCII size with tests ([`fc3d8a1`](https://github.com/o2sh/onefetch/commit/fc3d8a13f34fb105af763c520d9cfd146be60d2e))
    - Define available languages with macro ([`100d770`](https://github.com/o2sh/onefetch/commit/100d770825b08f9adbbb36d78ae77320e4626f09))
    - Define language colors in macro ([`536bf76`](https://github.com/o2sh/onefetch/commit/536bf76b08eb36ec5e94a8119129e8c17a546f41))
    - Enforce trailing comma ([`e76b8ec`](https://github.com/o2sh/onefetch/commit/e76b8ec85bd44ac1475aaf629a30cff26c6c5c5f))
    - Define languages with macro ([`800d998`](https://github.com/o2sh/onefetch/commit/800d998abf5b2c451ec5ce8a22b3b7e0cd3c9744))
    - Merge pull request #203 from o2sh/chore/stale-bot ([`c05bb94`](https://github.com/o2sh/onefetch/commit/c05bb943c534a16522a58f835beecea035dc181c))
    - Exempt "help wanted" from stale :robot: ([`9f73da5`](https://github.com/o2sh/onefetch/commit/9f73da5ff8f54d8d159ac1b6d4d0d099d71aec25))
    - Merge pull request #199 from o2sh/feature/tokio-command ([`ebe5b5b`](https://github.com/o2sh/onefetch/commit/ebe5b5bca9ee200a27c5d84169d300345e3bcf34))
    - remove unused use ([`b1c831d`](https://github.com/o2sh/onefetch/commit/b1c831df1b877d9f478ea021435b179eada945bb))
    - Info ctor marked as async ([`1f0bb64`](https://github.com/o2sh/onefetch/commit/1f0bb64c38da3f34513524c264b2fae0487f903a))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`1a201e6`](https://github.com/o2sh/onefetch/commit/1a201e6f568028bb4afbadd65677bcc789ba8d32))
    - update assets ([`ce7fe0d`](https://github.com/o2sh/onefetch/commit/ce7fe0d6488cb66f4c2426f0ba542c7d07bc829e))
    - Update CONTRIBUTING.md ([`afea709`](https://github.com/o2sh/onefetch/commit/afea709107882885b7e358ab60816a9ded2fea9c))
    - Update CONTRIBUTING.md ([`793d8e1`](https://github.com/o2sh/onefetch/commit/793d8e1a87881b17a51c6a2f66c1ecaaac286b1a))
    - Update README.md ([`29dd0c7`](https://github.com/o2sh/onefetch/commit/29dd0c771301fe04e43b8bfae491efccef329333))
    - add CONTRIBUTING file ([`281c24c`](https://github.com/o2sh/onefetch/commit/281c24ccb1a490ad9730dbfde354ec01993f9d58))
    - Update README.md ([`9849bd2`](https://github.com/o2sh/onefetch/commit/9849bd22c9d1fe80638490565898d2f7a78defc3))
    - Update README.md ([`f2550c8`](https://github.com/o2sh/onefetch/commit/f2550c8fc2af2162a10e292a5d601cde07e46322))
    - Update README.md ([`c2e6e6a`](https://github.com/o2sh/onefetch/commit/c2e6e6ac999bcf1e2edf9cbd956c3392a44038d6))
    - update assets ([`1cc1428`](https://github.com/o2sh/onefetch/commit/1cc14286f7bdcfa1bfcceefacee091671258c1c6))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`c6a3561`](https://github.com/o2sh/onefetch/commit/c6a35617adb241223ab25656776e7fe8675d3484))
    - update assets ([`d95aed2`](https://github.com/o2sh/onefetch/commit/d95aed26b087f632695c4958d9ed4fadcb910f99))
    - Update README.md ([`019c977`](https://github.com/o2sh/onefetch/commit/019c977828d5fdc81933ca5f8a2d4ef3e1e159ef))
    - Update README.md ([`24ebefc`](https://github.com/o2sh/onefetch/commit/24ebefc1713d4c7e19a04670736fd866940b9240))
    - update assets ([`1a3e00f`](https://github.com/o2sh/onefetch/commit/1a3e00fe1b8477b54edf85da41e97653bebdef2e))
    - update assets ([`8989d14`](https://github.com/o2sh/onefetch/commit/8989d14f8590a366a59ff59b80386ec57aef0227))
    - tokio command for non blocking sys call ([`e9d7d7c`](https://github.com/o2sh/onefetch/commit/e9d7d7c0f6530525fed52906008cef2108793e1f))
    - fix cargo clippy warnings ([`a5c3c93`](https://github.com/o2sh/onefetch/commit/a5c3c938a7d9b691fa200b53cf03ef50d352e1fc))
    - Merge pull request #196 from o2sh/dependabot/cargo/image-0.23.8 ([`e3c87c3`](https://github.com/o2sh/onefetch/commit/e3c87c3f3722ca08a7adef9b239c113a5c47cfdd))
    - Bump image from 0.23.5 to 0.23.8 ([`4432d1d`](https://github.com/o2sh/onefetch/commit/4432d1dbfc7cf15fc13ceacfa2d3118fb7b490b8))
    - Merge pull request #193 from o2sh/dependabot/cargo/colored-2.0.0 ([`873fc45`](https://github.com/o2sh/onefetch/commit/873fc453957f5fa7ae7631b0760bd9b5ac49b272))
    - Merge branch 'master' into dependabot/cargo/colored-2.0.0 ([`2f9587f`](https://github.com/o2sh/onefetch/commit/2f9587f0b860bbba7a4989447e8c08620a4b9c48))
    - Merge pull request #191 from o2sh/dependabot/cargo/git2-0.13.10 ([`53b25ed`](https://github.com/o2sh/onefetch/commit/53b25ed8be53cfd94547768154c4848091779baf))
    - Merge pull request #187 from o2sh/dependabot/cargo/strum-0.19.2 ([`547bf25`](https://github.com/o2sh/onefetch/commit/547bf25201794adafcaf60c28097e4e880ba3a83))
    - Fix strum breaking changes ([`366e26d`](https://github.com/o2sh/onefetch/commit/366e26dc77c856fac4a75f176cbeffb1d0fe97e4))
    - Get strum_macros as feature ([`ef75b49`](https://github.com/o2sh/onefetch/commit/ef75b497ff86999592e4e9c1424694ceab9cdf68))
    - Force colored to attempt colorization in test ([`39af779`](https://github.com/o2sh/onefetch/commit/39af779554c2c544297f142079c65ed7babd5232))
    - Bump colored from 1.8.0 to 2.0.0 ([`becb948`](https://github.com/o2sh/onefetch/commit/becb948aef88d6a4498dd5d227c10b5a491422a9))
    - Bump strum from 0.18.0 to 0.19.2 ([`e485f7f`](https://github.com/o2sh/onefetch/commit/e485f7f9f9062c24ac79f2406015ce6268a84108))
    - Bump git2 from 0.13.6 to 0.13.10 ([`ddf4b94`](https://github.com/o2sh/onefetch/commit/ddf4b9451a995add39de0703b7a327e0771b29b6))
    - Merge pull request #194 from o2sh/dependabot/cargo/image-0.23.5 ([`6fac4ca`](https://github.com/o2sh/onefetch/commit/6fac4ca9d59925fad8bc4fd65adf43028451f53e))
    - Bump image from 0.22.3 to 0.23.5 ([`cdbdee6`](https://github.com/o2sh/onefetch/commit/cdbdee6e85afb36eafb27cb917ad3c89d91f6b87))
    - Merge pull request #190 from o2sh/dependabot/cargo/askalono-0.4.2 ([`92998be`](https://github.com/o2sh/onefetch/commit/92998be3b2c363d1d9dd31ce6f3457615831b319))
    - Bump askalono from 0.4.0 to 0.4.2 ([`ec615a4`](https://github.com/o2sh/onefetch/commit/ec615a47aa91de2cc21d11a317d1f5679e891265))
    - Merge pull request #192 from o2sh/dependabot/cargo/libc-0.2.76 ([`02b36a9`](https://github.com/o2sh/onefetch/commit/02b36a9786ef2f99316f54ce35ed1e735eae2bda))
    - Merge pull request #188 from o2sh/dependabot/cargo/clap-2.33.3 ([`a81edaa`](https://github.com/o2sh/onefetch/commit/a81edaab300561506b0608c4376ec6c9732fda0a))
    - Bump libc from 0.2.65 to 0.2.76 ([`f595637`](https://github.com/o2sh/onefetch/commit/f5956376772af81951d33e12a4ae3a1748989486))
    - Bump clap from 2.33.1 to 2.33.3 ([`dbc9ba6`](https://github.com/o2sh/onefetch/commit/dbc9ba6a16b8197b40cbe19173e2b8a19d360af8))
    - Create dependabot config ([`6b3e6fc`](https://github.com/o2sh/onefetch/commit/6b3e6fc7e6d467cc36235d88e71ccd60b43d18ed))
    - Merge pull request #186 from o2sh/feature/async-await ([`b1d9c22`](https://github.com/o2sh/onefetch/commit/b1d9c22dc8a2d5721e7940a95928cda8c1348877))
    - async block ruturns Result for better error handling ([`1160e88`](https://github.com/o2sh/onefetch/commit/1160e88222c0850a047ec83959e9e4bd8f058616))
    - refacto use stmts and replace get_info_lines with async block ([`bcb4e64`](https://github.com/o2sh/onefetch/commit/bcb4e649d30f65d75e39cc33fbb8135feb191cf3))
    - async get_info_line ([`42f56c1`](https://github.com/o2sh/onefetch/commit/42f56c16d00dd5dbd620d2cb9449647c1514aaaf))
    - enable stale bot #184 ([`675c5c0`](https://github.com/o2sh/onefetch/commit/675c5c045dc7bda66e35db287969fd8b32f546db))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`97f4e55`](https://github.com/o2sh/onefetch/commit/97f4e554643a95f93a7bdb9834bd82398511b323))
    - bump to edition 2018 to enable async/await syntax #185 ([`99bff66`](https://github.com/o2sh/onefetch/commit/99bff6636c915d115ef2fd9faff024056223fbe2))
    - Merge pull request #182 from SuperSandro2000/cargo-deb ([`75e9d93`](https://github.com/o2sh/onefetch/commit/75e9d930a0bbb551d0e4543e3f1fc9220d401d2d))
    - Add info for cargo-deb ([`767fef6`](https://github.com/o2sh/onefetch/commit/767fef670a1328b19b499b1b662302712767fc77))
    - update Cargo.lock ([`bbc0359`](https://github.com/o2sh/onefetch/commit/bbc035928b2e1ee4dcd20e43f1c5d3bb2da2885a))
</details>

## v2.3.0 (2020-08-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 211 calendar days.
 - 211 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update changelog ([`075ace3`](https://github.com/o2sh/onefetch/commit/075ace3a6e146b468217d6c0f1b1be3ffcfb5156))
    - prepare version 2.3.0 ([`c5effb8`](https://github.com/o2sh/onefetch/commit/c5effb8a64d1e0248914f7116c26bc3547a8971a))
    - create man page ([`5137b40`](https://github.com/o2sh/onefetch/commit/5137b4017a23d1735e53a4fae153caa55020c2a0))
    - Remove Bright Colors #179 ([`8de4835`](https://github.com/o2sh/onefetch/commit/8de48350aa33491af226a138a946b248d7c2b6bb))
    - fix trailing slash in exclude pattern #178 ([`b415cc2`](https://github.com/o2sh/onefetch/commit/b415cc2597d0ee007b501d57676c995ca62aa981))
    - fix exclude OPTION to work wiht absolute paths #178 ([`c8cf756`](https://github.com/o2sh/onefetch/commit/c8cf756fe55d7d26b8540fa0fb4d6fbf1693fe68))
    - fix typo in cli help ([`f7023d7`](https://github.com/o2sh/onefetch/commit/f7023d7d96a9ea5ffb5f1621e36a34ee02a7dc46))
    - replace directory OPTIONS with input ARGS ([`a9b775a`](https://github.com/o2sh/onefetch/commit/a9b775afefa2d7c024a551f555c145b4fa087dd7))
    - improve jupyter ascii art #151 ([`81fd2b1`](https://github.com/o2sh/onefetch/commit/81fd2b1d69ec92165d8b92ada3181ddd0e8ed15c))
    - jupyter notebook support #151 ([`65e6ac4`](https://github.com/o2sh/onefetch/commit/65e6ac4c25c297277277ab10a8ddc2da382d08c2))
    - Merge branch 'master' into feature/exclude-option ([`e06395a`](https://github.com/o2sh/onefetch/commit/e06395a4e23d25c69bbd7d0516fb3657f8968c97))
    - bump version of bytecount and git2  to latest ([`1add7bc`](https://github.com/o2sh/onefetch/commit/1add7bcb34444971ba0678b2e4de7ee47dac45a9))
    - bump version of strum to latest ([`d88d8ee`](https://github.com/o2sh/onefetch/commit/d88d8ee48d3adf1054acfdaa6f9a8dbc29ddbbef))
    - bump version of tokei to latest ([`29f2180`](https://github.com/o2sh/onefetch/commit/29f218076c9d5aba7c7658eef14bacab6f41faad))
    - Update README.md ([`dc9bb4b`](https://github.com/o2sh/onefetch/commit/dc9bb4bffd9e6b96b81554f72a4dd3622f63b47f))
    - Update README.md ([`f2989e5`](https://github.com/o2sh/onefetch/commit/f2989e5c99c153b380f0f859df7221c3485deb42))
    - remove is_root bool from get_language ([`a217f1f`](https://github.com/o2sh/onefetch/commit/a217f1f452a9d21ccc93dff319e6cb3471420b3b))
    - update ci.yml ([`a254e46`](https://github.com/o2sh/onefetch/commit/a254e460ee83792e8619d2e50b972475b4e4b654))
    - upgrade tokei dependency to latest ([`92c9afe`](https://github.com/o2sh/onefetch/commit/92c9afe016d501f8ad5efd41ce07c959b20f61ea))
    - add --exlude option ([`6003e04`](https://github.com/o2sh/onefetch/commit/6003e0454e563f34d3557dc1f90670cda41cc6a1))
    - add possible values to ascii colors option ([`81e5334`](https://github.com/o2sh/onefetch/commit/81e53347b7a43f23bd7043591220819e9579e308))
    - Merge branch 'master' of https://github.com//o2sh/onefetch ([`a7f25a9`](https://github.com/o2sh/onefetch/commit/a7f25a9385163d73213de12173e89912f384af56))
    - Better descriptions for flags and options ([`5cfdb2a`](https://github.com/o2sh/onefetch/commit/5cfdb2aae108c09445a373cfd7147ab3cc7a6888))
    - Merge pull request #177 from ebroto/licence ([`65fe707`](https://github.com/o2sh/onefetch/commit/65fe707c713a969a9db0e9f1b5ddac0581f284a2))
    - Accept LICENCE... as license file name ([`52dedba`](https://github.com/o2sh/onefetch/commit/52dedba48e246e3fcd20b23b0be0040588c097b7))
    - Fix detection of Racket language #174 ([`a64ad56`](https://github.com/o2sh/onefetch/commit/a64ad567dfbbe4d9ef3f304ae02ae0b028cdf11d))
    - add support for DockerFile #173 ([`159b934`](https://github.com/o2sh/onefetch/commit/159b934d9b613f86d09d65c143d7b29a482e591a))
    - Cargo.lock ([`b69fe66`](https://github.com/o2sh/onefetch/commit/b69fe660d72b65d7efac99ac5db3b03a82d8667f))
    - update README ([`1552db1`](https://github.com/o2sh/onefetch/commit/1552db1a7e0856ac3cf7eae66c47ee9828cd9592))
</details>

## v2.2.0 (2020-01-04)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 55 calendar days.
 - 55 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - preparation for release 2.2.0 ([`f462f29`](https://github.com/o2sh/onefetch/commit/f462f2961908df0b37b8a205382a07b922fd5dfb))
    - remove unused files ([`c1eff19`](https://github.com/o2sh/onefetch/commit/c1eff19743558fe6cc3dd508b787868f85b09ca4))
    - travis no more required ([`9f9472e`](https://github.com/o2sh/onefetch/commit/9f9472e6ee3aecce507cebbc6dc18a3b1e88cd02))
    - Update README.md ([`3e4853e`](https://github.com/o2sh/onefetch/commit/3e4853ea328126020051dd71533e629210cdce29))
    - Update ci.yml ([`993e746`](https://github.com/o2sh/onefetch/commit/993e746626a28c0dffb6cad0b80b3a21522169e4))
    - Update ci.yml ([`d291813`](https://github.com/o2sh/onefetch/commit/d29181339dbd2e132a6c6e2c630cf54b80c89e68))
    - Update ci.yml ([`3800418`](https://github.com/o2sh/onefetch/commit/3800418b3b42ad3f28c1013d01f56e65bf6302ad))
    - Update ci.yml ([`9ce3089`](https://github.com/o2sh/onefetch/commit/9ce308979ae93b2bcc8983efcb458343f8756ac2))
    - Update ci.yml ([`5f8fd68`](https://github.com/o2sh/onefetch/commit/5f8fd68dda7bece141940c8eaddb2a2f15ec8f15))
    - Create ci.yml ([`8140d49`](https://github.com/o2sh/onefetch/commit/8140d49ef238c03ff17a7c8d230d31cdcda7a05f))
    - cleaned php ascii logo ([`a5e03fd`](https://github.com/o2sh/onefetch/commit/a5e03fda454ddb2037fa090307e4e4dd5a959275))
    - Merge pull request #169 from Sh3mm/master ([`fb0ad81`](https://github.com/o2sh/onefetch/commit/fb0ad81a6472845b10e13c8c5d1efcd5a0ec1b2d))
    - Corrected the protruding part ([`098ebc6`](https://github.com/o2sh/onefetch/commit/098ebc6e13ad99ba2abb0f3165a9176c46789ca5))
    - Merge pull request #168 from Sh3mm/patch-1 ([`640057d`](https://github.com/o2sh/onefetch/commit/640057d3e3b970eac44c771d4425f5dbc475cbea))
    - changed the php logo to a better one ([`a93bb98`](https://github.com/o2sh/onefetch/commit/a93bb98e71ad7435b5a883ace5ab50c576654825))
    - Merge branch 'patch-1' of https://github.com/Sh3mm/onefetch into patch-1 ([`7edeefb`](https://github.com/o2sh/onefetch/commit/7edeefb2f511a12b4ba596019b088d8ca671a108))
    - addition of an authors-number parameter ([`cae296c`](https://github.com/o2sh/onefetch/commit/cae296c8ee0f488e2dfb5e2b3af2658fbcf89490))
    - trim left pad in csharp ascii ([`b4d4514`](https://github.com/o2sh/onefetch/commit/b4d4514166c736b378182185a0685f07ec547912))
    - Merge pull request #167 from Sh3mm/patch-1 ([`a275398`](https://github.com/o2sh/onefetch/commit/a2753986e9ab84a834fe65a06dad4b2e4f5e7d97))
    - removed the superfluous {0} ([`859b276`](https://github.com/o2sh/onefetch/commit/859b27654d44ea587897abd123d69a888999cc35))
    - changed the color to real ones ([`cc6d005`](https://github.com/o2sh/onefetch/commit/cc6d00533fe6da5ec940fa424afad674d965bc8f))
    - trim leading spaces in Pending info_name ([`753885c`](https://github.com/o2sh/onefetch/commit/753885c3cfc681d194e09d5c1f3526f925eed7a2))
    - Update language.rs ([`b6f3f7f`](https://github.com/o2sh/onefetch/commit/b6f3f7f7074c440b6492895a01e435cb7ab0cc40))
    - Merge pull request #166 from axdiamond/show-pending-changes ([`28cab16`](https://github.com/o2sh/onefetch/commit/28cab1649df4f6476fd5c0808b264bfa83180899))
    - Update csharp.ascii ([`e1e9a2d`](https://github.com/o2sh/onefetch/commit/e1e9a2d3854617b525a0f70cd2be7be8d0ea74a0))
    - Dont show zeros ([`4a3c136`](https://github.com/o2sh/onefetch/commit/4a3c136f3d80e83255d2c1455e7c9a05d58a7ced))
    - Add another missing case for renamed files ([`876181e`](https://github.com/o2sh/onefetch/commit/876181e665680d3af0da178085c97647d40524d2))
    - Add missing match ([`6c74e8f`](https://github.com/o2sh/onefetch/commit/6c74e8fa86eca44b85a9798902c89972225a3426))
    - Show file level changes ([`ffabfdb`](https://github.com/o2sh/onefetch/commit/ffabfdb33f6ee7bef7fa7c1c09a21f18a6fef346))
    - Move pending under Head in output ([`887b65d`](https://github.com/o2sh/onefetch/commit/887b65da959226748075e34431a9e04226324ff6))
    - Change 'changes' to pending ([`0cd4e35`](https://github.com/o2sh/onefetch/commit/0cd4e3518d415d8d534fb9923e7ad0f869d96685))
    - Add changes line ([`30b5d01`](https://github.com/o2sh/onefetch/commit/30b5d01e080d79e8bb0c5e03aaa6cb27e6d06638))
    - add support for Groovy #163 ([`90bdc4e`](https://github.com/o2sh/onefetch/commit/90bdc4ecb48d71e23871e4266a96e5d381c7d226))
    - update color profile for cpp ([`869d4f1`](https://github.com/o2sh/onefetch/commit/869d4f1830351d5a75823929e2ee1d5e6de8ae5a))
    - Merge pull request #162 from Sh3mm/patch-1 ([`a5c874f`](https://github.com/o2sh/onefetch/commit/a5c874f4d5c1c5350a187e5ff5aacf93b3b80700))
    - Update cpp.ascii ([`3f84e2c`](https://github.com/o2sh/onefetch/commit/3f84e2ca7080fd178e68c21af61fd808db31c329))
    - Merge pull request #158 from Phundrak/master ([`7ed1c6a`](https://github.com/o2sh/onefetch/commit/7ed1c6a545900d936b58bb3f3b57844209b82a51))
    - Removed strum line that shouldn't have been there ([`36fcc86`](https://github.com/o2sh/onefetch/commit/36fcc8624ce8789f1c54e583e34deabe65b6a8cb))
    - Fix release-assets workflow ([`3d4e4c2`](https://github.com/o2sh/onefetch/commit/3d4e4c22f069b2c462ffda768a556afda9b410c7))
    - Merge pull request #161 from erikgaal/build-release-assets ([`e3548d4`](https://github.com/o2sh/onefetch/commit/e3548d46db809f10d63f84d887b13b65722aefd5))
    - Add release.yml ([`c2d81f1`](https://github.com/o2sh/onefetch/commit/c2d81f11c8a828ac2e0fb2ff657aaa5ae07386e8))
    - update AUTHORS file ([`daabe80`](https://github.com/o2sh/onefetch/commit/daabe809ffcb9b0696755a83f7bcfdaadc5d601e))
    - Merge branch 'master' of github.com:o2sh/onefetch ([`2aef81f`](https://github.com/o2sh/onefetch/commit/2aef81f83caa95a88c40bb8df09c4dc1261ad283))
</details>

## v2.1.0 (2019-11-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare v2.1.0 release ([`2eb2b11`](https://github.com/o2sh/onefetch/commit/2eb2b112be5bbeaaf70ce31fc0d9f866c9b63eab))
    - Added support for Nix ([`4475bf4`](https://github.com/o2sh/onefetch/commit/4475bf4a935f352d0a6c90f80457a6f9cb9f1074))
    - Added CMake support ([`7c58540`](https://github.com/o2sh/onefetch/commit/7c58540514e8f5083b1229185da551928ec4f886))
    - Added support for the fish shell language ([`029cc0c`](https://github.com/o2sh/onefetch/commit/029cc0cab14f16c3b4360dbdc9712f731655515b))
    - fixed issue with option `-a emacslisp`, moved elisp.ascii to emacslisp.ascii ([`0639324`](https://github.com/o2sh/onefetch/commit/06393244c9bcbb416da93a6b2afd9d7b68500989))
    - renamed elisp.ascii to emacslisp.ascii ([`b3bd63e`](https://github.com/o2sh/onefetch/commit/b3bd63ec262dcd894a61f1decd0a546fdaff4007))
    - Merge pull request #157 from ebroto/update-to-askalono-0.4.0 ([`a24c22b`](https://github.com/o2sh/onefetch/commit/a24c22b63032ed56d8d1fc0612b359a335e5079a))
    - [strum(serialize = org-mode)] for Org ([`e451104`](https://github.com/o2sh/onefetch/commit/e45110411b3407c1de714c62949e3bd863ea8856))
    - Fixed display name for Org ([`096ced1`](https://github.com/o2sh/onefetch/commit/096ced1cabc23e3cb559c48b501c94f25577e2f5))
    - Added Emacs Lisp support ([`6ac0b5f`](https://github.com/o2sh/onefetch/commit/6ac0b5fb18999881a6351d3268cd4e358d5839af))
    - Merge pull request #155 from Phundrak/master ([`8e4369a`](https://github.com/o2sh/onefetch/commit/8e4369a053197f39c1f005bd6271cc5a9ea766c5))
    - Update to askalono 0.4.0 and use a more strict confidence threshold ([`6ae318c`](https://github.com/o2sh/onefetch/commit/6ae318c8222ce03a71ab037e6a56354ec0002df0))
    - Merge pull request #154 from CephalonRho/sixel-backend ([`6b145a5`](https://github.com/o2sh/onefetch/commit/6b145a5a2aeef585ba583a5b55bd1a9a0f30ca94))
    - Added org-mode support ([`c7d82e4`](https://github.com/o2sh/onefetch/commit/c7d82e422e1cfdc8fa1d6361ca19304995a39751))
    - Fix sixel support detection ([`745982d`](https://github.com/o2sh/onefetch/commit/745982dbc19924398177836ec5c324e03d0cb80f))
    - Fix windows build ([`1194f71`](https://github.com/o2sh/onefetch/commit/1194f71a867b665bca3ac6cb46c79a75cf823073))
    - add --no-color-blocks flag #153 ([`f74f741`](https://github.com/o2sh/onefetch/commit/f74f7410a8661ce675cb47d8d459c5d56d55dcc7))
    - Add image-backend argument ([`c5a1e2c`](https://github.com/o2sh/onefetch/commit/c5a1e2c97013f7ab70f85a410061c099667b8cec))
    - Fix color introducer string ([`cb8225f`](https://github.com/o2sh/onefetch/commit/cb8225f63db6f88169c47e9601226445474bcaf7))
    - Update Cargo.lock ([`e14aa15`](https://github.com/o2sh/onefetch/commit/e14aa15de0e3572ed807753f2f4d7494c0ee1e53))
    - Add sixel backend ([`dc4e360`](https://github.com/o2sh/onefetch/commit/dc4e360256c330e2cac6e7a8fa66e9f0a131c509))
    - Fix the kitty backend reading more bytes than it should ([`a64db30`](https://github.com/o2sh/onefetch/commit/a64db30f1b8fbc35371a46a107af4d35ab21e30c))
</details>

## v2.0.1 (2019-11-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - patch release for #152 ([`7baa41a`](https://github.com/o2sh/onefetch/commit/7baa41af124eafba04fd9bb6821b39711f14fe0b))
    - Merge pull request #152 from ebroto/bugfix/isc-license-not-recognised ([`575b3ed`](https://github.com/o2sh/onefetch/commit/575b3ed1f01ad31e9b8dbcb8170b37ad390d9710))
    - Use a newer version of askalono so ISC is detected ([`4dfc4e5`](https://github.com/o2sh/onefetch/commit/4dfc4e55d70b0b734694e8ced760ccb37b9fe752))
    - push Cargo.lock ([`13993b5`](https://github.com/o2sh/onefetch/commit/13993b58ad617dbbc38a60039375e6731b2b253a))
</details>

## v2.0.0 (2019-11-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 52 commits contributed to the release over the course of 6 calendar days.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - preparation for 2.0.0 release ([`c645f25`](https://github.com/o2sh/onefetch/commit/c645f254ec072a61f8c18ae610f1befe0f96f940))
    - limit language stat to one decimal ([`8b710bd`](https://github.com/o2sh/onefetch/commit/8b710bd558cf3aa2a789021a2ecf9147f512eea2))
    - Merge pull request #150 from KasraF/ocaml_support ([`e44d736`](https://github.com/o2sh/onefetch/commit/e44d736b76b76e48fe586a4af36b50eb32d2bd7c))
    - Merge branch 'master' into ocaml_support ([`3ba0020`](https://github.com/o2sh/onefetch/commit/3ba00208a419f8ffe60c03ae77e4bef2de8e1049))
    - add [strum(serialize = fortran)] for FortranModern ([`a133771`](https://github.com/o2sh/onefetch/commit/a133771d5c16754939f7be794e0aa428a8f5feb0))
    - rollback to static license cache ([`5455efe`](https://github.com/o2sh/onefetch/commit/5455efeeb055a8964fc7714da38ada729b2360ae))
    - Merge pull request #149 from ebroto/feature/license-with-askalono ([`a2f6352`](https://github.com/o2sh/onefetch/commit/a2f63528648e02c303a5364b8e54f5c0036958d4))
    - Merge pull request #142 from ZapAnton/fix_language_name_mismatch ([`ce4756e`](https://github.com/o2sh/onefetch/commit/ce4756e462d6fd0b07c091845209b72adf97b270))
    - Removed the default value for the 'ascii_language' argument ([`e419f93`](https://github.com/o2sh/onefetch/commit/e419f93a655352a81d326de9dd498c84b958b9eb))
    - fixed whitespace ([`7909622`](https://github.com/o2sh/onefetch/commit/790962244fb96584f8421a8d5341dfb3f8fedaa1))
    - Added support for the OCaml language ([`77febb2`](https://github.com/o2sh/onefetch/commit/77febb2c4eb623250f5d4de439a4beb295a454c9))
    - Made the '-a' flag accept language names with special characters ([`13928fa`](https://github.com/o2sh/onefetch/commit/13928fa86f43eabdcf0a703369f8c32f9d8539ff))
    - Add spdx licenses as a submodule ([`018c9c2`](https://github.com/o2sh/onefetch/commit/018c9c2d87a5353c43c8f0ca4ba1c74efefb1b5b))
    - Generate license cache at build time ([`a1243fa`](https://github.com/o2sh/onefetch/commit/a1243fa038d7aad501a8ff4049e6192849751b8d))
    - Rework license module to avoid loading cache for each license ([`dd1e480`](https://github.com/o2sh/onefetch/commit/dd1e480d643272675cc59f401f71543daabe93b5))
    - Replace license crate with askalono ([`4d35e5a`](https://github.com/o2sh/onefetch/commit/4d35e5a965696f9255f5917b1674728df77c45bd))
    - missing ref #148 ([`493a07f`](https://github.com/o2sh/onefetch/commit/493a07fc55ca5bdf5946c64646d1ae08afaed1ad))
    - fix markdown ascii ([`b18e8ce`](https://github.com/o2sh/onefetch/commit/b18e8ce00027fe48066675405705ebc7a3007687))
    - add support for markdown #148 ([`a698aaa`](https://github.com/o2sh/onefetch/commit/a698aaa24646582a5a081e2d3ab2461962e120ef))
    - update fortran asset ([`1bbc83f`](https://github.com/o2sh/onefetch/commit/1bbc83f7b4040be56e6fa625b5889ca516031590))
    - update readme image ([`5bf83c3`](https://github.com/o2sh/onefetch/commit/5bf83c3a7d49be9b76fcf42374e3d912ad62f1b5))
    - add support for Fortran 90 #138 ([`01fd813`](https://github.com/o2sh/onefetch/commit/01fd813edf2c88c5b35162d027412739a549ea38))
    - Moved the possible language values from the 'help' method to the 'possible_values' method ([`46e70b3`](https://github.com/o2sh/onefetch/commit/46e70b33a3494e661a2f52d678977adc92ef485d))
    - Merge pull request #139 from rockisch/master ([`9382549`](https://github.com/o2sh/onefetch/commit/93825493fbc17b9b4e4a56ae32215cfe7f84ec62))
    - Improve functions ordering ([`f5adb95`](https://github.com/o2sh/onefetch/commit/f5adb958dc317dde363758f98f19d66577f7222f))
    - Improve running command from subfolder ([`790a85c`](https://github.com/o2sh/onefetch/commit/790a85c2b1ca5f16de04c3598e8e51fdb237b231))
    - add support for D #145 ([`7ce913e`](https://github.com/o2sh/onefetch/commit/7ce913e3d68e62ce028e9d31b6024e9ffade5c4b))
    - Added the possible language list to the '--ascii_language' flag help message ([`a9e6184`](https://github.com/o2sh/onefetch/commit/a9e61843f7ede08ee81faae8c8b0e59f1e365bcf))
    - Changed the string representation of the languages , that contain special symbols ([`c110481`](https://github.com/o2sh/onefetch/commit/c11048131c3fc2dc46a4cd560fbe3df76e6760ad))
    - Merge pull request #141 from o2sh/feature/no-merges ([`654a726`](https://github.com/o2sh/onefetch/commit/654a726a5d8ba05e26c7a6a28a6eeb5fdd45dfaa))
    - Add -n flag for no-merges ([`f09d556`](https://github.com/o2sh/onefetch/commit/f09d5569a8da4d813e9fb58ff17cdd7e73fd8293))
    - Merge remote-tracking branch 'origin/master' into feature/no-merges ([`f6df4dc`](https://github.com/o2sh/onefetch/commit/f6df4dc67c977564f2c45ce8d345070d09759dee))
    - Merge pull request #140 from ZapAnton/fix_clippy_warnings ([`1e10bd2`](https://github.com/o2sh/onefetch/commit/1e10bd2648734fdfc4d0718fda52ee945b8a8703))
    - Prevent merge author count on --no-merges ([`5672098`](https://github.com/o2sh/onefetch/commit/5672098e4a19e70d8cfe448aea7600065d0c978f))
    - Add --no-merges flag for total commit count ([`efe9d70`](https://github.com/o2sh/onefetch/commit/efe9d704c65196801a343f8afe280fd5888b9de2))
    - Applied  'cargo fmt' ([`615eecd`](https://github.com/o2sh/onefetch/commit/615eecdc91220b5c55784a93fc134cc96455689e))
    - Fixed the 'needless_collect' clippy warning ([`0ea794b`](https://github.com/o2sh/onefetch/commit/0ea794b5c7f3da1b9015952b5d3794ddde7d3d8c))
    - Fixed the 'clone_on_copy' clippy warning ([`cc59305`](https://github.com/o2sh/onefetch/commit/cc593054154d131b8973f1ed603e1682160e3c6f))
    - Fixed the redundant_pattern_matching clippy warning ([`8e85b53`](https://github.com/o2sh/onefetch/commit/8e85b53503507c9e0ca8c3cba89d42875cff1805))
    - Fixed the 'len_zero' clippy warning ([`b214dc2`](https://github.com/o2sh/onefetch/commit/b214dc2cb31390abf6397b316ee37a2c3d4361ab))
    - Fixed the 'identity_conversion' clippy warning ([`090493e`](https://github.com/o2sh/onefetch/commit/090493e54a266f839892e1df9def6e12ec2ca97e))
    - Fixed the needless_lifetimes clippy warning ([`95b5b75`](https://github.com/o2sh/onefetch/commit/95b5b755437ebc7ba42b1b648d69790736c760cb))
    - Fixed the 'single_char_pattern' clippy warning ([`ddb100c`](https://github.com/o2sh/onefetch/commit/ddb100c4587545ce400b49bb48dcf7922b09c63c))
    - Fixed the 'ptr_arg' clippy warning ([`065d65a`](https://github.com/o2sh/onefetch/commit/065d65a2191c987dcfb4aba8d5008eae7e2324b6))
    - Fixed the 'write_literal' clippy warning ([`d868275`](https://github.com/o2sh/onefetch/commit/d8682752aca78bc5d596916f9d464e9835225ebb))
    - Fixed the 'op_ref' clippy warning ([`cf815c0`](https://github.com/o2sh/onefetch/commit/cf815c029b1e8c3fdb7b9939e7eb450f290b34a6))
    - Fixed the 'cast_lossless' clippy warning ([`3582a88`](https://github.com/o2sh/onefetch/commit/3582a88210c986b332b960eb4291bf886ab30f85))
    - Fixed the 'char_lit_as_u8' clippy warning ([`54d9951`](https://github.com/o2sh/onefetch/commit/54d99513c7f78487137cde35df0e81e708a77ece))
    - Fixed the 'redundant field names in struct initialization' clippy warning. ([`cb97ca1`](https://github.com/o2sh/onefetch/commit/cb97ca13f5748a347e7cc128eb7683b318d1e84d))
    - add support for Julia #136 ([`0815e8a`](https://github.com/o2sh/onefetch/commit/0815e8a138f6ed1bc69c0572a06f17da880f1799))
    - Merge pull request #132 from rockisch/master ([`110811a`](https://github.com/o2sh/onefetch/commit/110811aaaa0caeeaa65af12c3060403dc715ca80))
    - Allow command to run from subfolder ([`2dd7ff3`](https://github.com/o2sh/onefetch/commit/2dd7ff30ec0187d9e6fcb9fd582ee4081d40ccf0))
</details>

## v1.7.0 (2019-10-29)

<csr-id-f0285a06a1779fc1871345d3d1d3fad090ca004c/>

### Documentation

 - <csr-id-73bdadb1488e73a1e2360b01297a86780e9aa63a/> update

### Other

- <csr-id-f0285a06a1779fc1871345d3d1d3fad090ca004c/> fixing missing comma

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 140 commits contributed to the release over the course of 17 calendar days.
 - 19 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update README.md ([`cf068a3`](https://github.com/o2sh/onefetch/commit/cf068a3f788d1ec284afbd73b129d98aef930d24))
    - Merge pull request #127 from portgasd666/master ([`06d58db`](https://github.com/o2sh/onefetch/commit/06d58db7d9fa6e0575d110381c95fcf22e9e7335))
    - Trim *all* trailing blanklines ([`3bfe441`](https://github.com/o2sh/onefetch/commit/3bfe441925caec730f84b40a39b130b3117b13fc))
    - Bump license from 0.7.1 to 0.8.1 Use new from_text_ext API instead of the Kind Enum ([`fd5f13c`](https://github.com/o2sh/onefetch/commit/fd5f13c97ab8e3be55df50ad3b060124afaafc2f))
    - Update CHANGELOG.md ([`2cb04b1`](https://github.com/o2sh/onefetch/commit/2cb04b1b6c0bf1f88c14b4d810586396b2c46e9e))
    - new Cargo.lock ([`99c2868`](https://github.com/o2sh/onefetch/commit/99c2868ed72dcaa5927d4fdb0041271389b543b0))
    - update ([`73bdadb`](https://github.com/o2sh/onefetch/commit/73bdadb1488e73a1e2360b01297a86780e9aa63a))
    - target_os instead of targt ([`9769425`](https://github.com/o2sh/onefetch/commit/97694253d550a4fc0cb3aa5e123f244f3cfa6f36))
    - target_os instead of target, fix #86 ([`941a830`](https://github.com/o2sh/onefetch/commit/941a830661effd6d73b8ecda48ef844fc2a13c33))
    - Set color override to true if enabled ([`d548783`](https://github.com/o2sh/onefetch/commit/d5487830b5cd646116fb824f92db8bb3bd60d5ac))
    - Merge pull request #124 from astynax/master ([`99ff814`](https://github.com/o2sh/onefetch/commit/99ff814585e41168a5eb16f9011513c78a0e1891))
    - Add Racket logo ([`b2d528c`](https://github.com/o2sh/onefetch/commit/b2d528c4c0ad2d8403b84f8a877c79b1ab6c03be))
    - rollback travis.yml and mv build before test #117 and #109 ([`37fddaa`](https://github.com/o2sh/onefetch/commit/37fddaa787c96b5af93518ab4532585f6cb576fc))
    - Merge pull request #113 from CephalonRho/display-image ([`d66e76f`](https://github.com/o2sh/onefetch/commit/d66e76f875eae6c7981d4f6626d321d8dd064dda))
    - Merge pull request #110 from Kloenk/master ([`a87ad8b`](https://github.com/o2sh/onefetch/commit/a87ad8b1df9216f100bfd4de831ab68a8d334538))
    - update README images ([`aa1b9bc`](https://github.com/o2sh/onefetch/commit/aa1b9bc57734a7a4ac6c673fe13dedada7f2eb21))
    - update README images ([`2ee63ba`](https://github.com/o2sh/onefetch/commit/2ee63ba2e0b8d26a076afc96b9e4d0818f9270a4))
    - update README images ([`e28cc0d`](https://github.com/o2sh/onefetch/commit/e28cc0d271278409ae7d8a03c8de67a65e9ed3b9))
    - update README images ([`e749ec2`](https://github.com/o2sh/onefetch/commit/e749ec27acacd790f694e58af462bc28305e35b7))
    - Merge pull request #114 from CephalonRho/limit-lang ([`08d9130`](https://github.com/o2sh/onefetch/commit/08d913062549f2c7d69d76a4bd457d48e7ef5389))
    - Remove unused mut ([`9f080e4`](https://github.com/o2sh/onefetch/commit/9f080e4fb8b20da4130e901c4c39875c8822878c))
    - Always show other language stat last ([`3dabf7b`](https://github.com/o2sh/onefetch/commit/3dabf7b150bcd72c0adabec7a768ea916bac4456))
    - Merge branch 'master' of https://github.com/Kloenk/onefetch ([`b494a18`](https://github.com/o2sh/onefetch/commit/b494a1803f5b54cd1f35b77251a418108c59a121))
    - remove the platform meta data ([`addc42d`](https://github.com/o2sh/onefetch/commit/addc42d5d1125d66b2ddd5edea84641cbd0d979f))
    - Avoid unnecessary allocation ([`710f1eb`](https://github.com/o2sh/onefetch/commit/710f1ebd15edacbaffc8aa81264934bd1632b478))
    - Limit shown languages to 6 ([`8f07135`](https://github.com/o2sh/onefetch/commit/8f07135299545cb233456e93c9a22172e6a77bb7))
    - Fix missing function for non-Linux targets ([`c8ed05f`](https://github.com/o2sh/onefetch/commit/c8ed05f8334ee3983dde6c41dd999887e228740f))
    - Fix center pad sometimes being added multiple times ([`7d25da0`](https://github.com/o2sh/onefetch/commit/7d25da049fece279b855bb2c7420200abeb0c549))
    - Add support for displaying a custom image instead of ascii art ([`dad9449`](https://github.com/o2sh/onefetch/commit/dad94491984d31e0bfa2235d82eb8d10424c69b6))
    - fixed regression - order languages by loc ([`f5296f2`](https://github.com/o2sh/onefetch/commit/f5296f2bd338c9060017438bef5ca28b7823142a))
    - Delete rust.yml ([`35c4ebe`](https://github.com/o2sh/onefetch/commit/35c4ebef83561d8ac168566fc4637276f33b35a1))
    - Update rust.yml ([`a3af741`](https://github.com/o2sh/onefetch/commit/a3af7413964d2434caa880cf3c5875cd8c420f4c))
    - change nix script ([`05625b7`](https://github.com/o2sh/onefetch/commit/05625b7e2dea26f3fc7938199558f6e7c693f3aa))
    - Fix travis ci ([`098dcfc`](https://github.com/o2sh/onefetch/commit/098dcfce80e75e35f8c3dce6a814279e276b623c))
    - add nix testing for travis ([`2bf4531`](https://github.com/o2sh/onefetch/commit/2bf45316d2fef13e04890714c1dce79c5ca72723))
    - Merge pull request #108 from Emanon42/master ([`c527767`](https://github.com/o2sh/onefetch/commit/c5277677d67304c2759fc8c07dbd7ee872696853))
    - Update fsharp.ascii ([`b52cc18`](https://github.com/o2sh/onefetch/commit/b52cc1893706a41a38356893e6e2e3f448e364fc))
    - Update language.rs ([`4ed8a3e`](https://github.com/o2sh/onefetch/commit/4ed8a3e5fe13c1f5634cb6dd613cd8cf00ad117e))
    - Update fsharp.ascii ([`29d0b84`](https://github.com/o2sh/onefetch/commit/29d0b84aa506ae6f9b1864ef16551596493ad270))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`768e86f`](https://github.com/o2sh/onefetch/commit/768e86feca3da665d4fb24a4a4485ad23037bf21))
    - slight refacto of #107 and change separator ([`c0d1689`](https://github.com/o2sh/onefetch/commit/c0d168992405af6c04aae8902e5949e230880293))
    - Merge pull request #94 from GooseDB/prolog ([`4121af5`](https://github.com/o2sh/onefetch/commit/4121af5f6eee78335991564bd4ba2fe1ee30d47a))
    - colors ([`8331d04`](https://github.com/o2sh/onefetch/commit/8331d04c6d9d0cf770662581c17dc4f0d43fedc6))
    - add new lang in code ([`e95582a`](https://github.com/o2sh/onefetch/commit/e95582a3f13fdbaba2f76c887fdf1fe1452e70c6))
    - Merge branch 'master' of https://github.com/o2sh/onefetch into prolog ([`46a2cad`](https://github.com/o2sh/onefetch/commit/46a2cad7962f45fd9df5405384dae1c0181c8302))
    - Merge pull request #111 from spenserblack/master ([`f617b49`](https://github.com/o2sh/onefetch/commit/f617b491f67fd04e30daabda4cf037c2891c46af))
    - a new ascii ([`8d801e3`](https://github.com/o2sh/onefetch/commit/8d801e34469ddc91e4991f96638ec72e563283a1))
    - Merge pull request #107 from pablodiegoss/master ([`908c866`](https://github.com/o2sh/onefetch/commit/908c86668103a112e26761764cb5102a26ee2b97))
    - [ImgBot] Optimize images ([`6089e04`](https://github.com/o2sh/onefetch/commit/6089e041cf25fdaaceea7db9c35f871989cd77f4))
    - Fix separator color ([`ae95e95`](https://github.com/o2sh/onefetch/commit/ae95e9552492b67d172c0a12d50c6c57e70a698b))
    - add default.nix ([`ae1cf8b`](https://github.com/o2sh/onefetch/commit/ae1cf8bab0e88a952caf60f175383122d66ed2e8))
    - Update usage of write_buf and colors ([`cc6a9c8`](https://github.com/o2sh/onefetch/commit/cc6a9c8a9e114c3665f02f302bf8c38392f4443c))
    - Merge remote-tracking branch 'o2sh/master' ([`419a32b`](https://github.com/o2sh/onefetch/commit/419a32bab15769475e02785b130721e6a7240816))
    - Removing redundancies in string usage ([`2b5ddf4`](https://github.com/o2sh/onefetch/commit/2b5ddf4e4d7f0d945ef61dc30189b009f5ddaee1))
    - Create fsharp.ascii ([`09fe396`](https://github.com/o2sh/onefetch/commit/09fe3966105bd33ccdeadc3f8f8e189e6c12d299))
    - Update language.rs ([`c763738`](https://github.com/o2sh/onefetch/commit/c763738e4e6d388bf1433609c1a9ef19d034de34))
    - disable cache in travis ([`e15894c`](https://github.com/o2sh/onefetch/commit/e15894c84e147ba09dbb45723c1465527a73dc7f))
    - Merge pull request #99 from ccmetz/bold-flag ([`793bc43`](https://github.com/o2sh/onefetch/commit/793bc431493b89fed7618072c89fe41f8c135c5c))
    - Ignore username when empty or unset ([`daa59b4`](https://github.com/o2sh/onefetch/commit/daa59b460ad217c0201e061127c98ad12ebf93f7))
    - Format separator based on git info length ([`1fb086f`](https://github.com/o2sh/onefetch/commit/1fb086fcbd0823134ce920338b6f7f1c7dd8e335))
    - Fix directory access on get git info ([`50b8763`](https://github.com/o2sh/onefetch/commit/50b8763b8044ca5745039890d2ebdcb03005989e))
    - Add git info for user and version ([`e9f4f8d`](https://github.com/o2sh/onefetch/commit/e9f4f8d5d71475b493feaa3d9f2fc7d798046881))
    - Minor refactoring ([`bcce1ca`](https://github.com/o2sh/onefetch/commit/bcce1cae2175fdb65e9579fb1a185cf4fbe2a7d8))
    - Change bold option to a flag ([`0b97ed6`](https://github.com/o2sh/onefetch/commit/0b97ed6a6ed75223dc8afd13031e8bd2c944b390))
    - Merge branch 'master' into bold-flag ([`f3f90d6`](https://github.com/o2sh/onefetch/commit/f3f90d60293b595bc1653858c379287fcaf8d0a6))
    - small changes in #103 ([`d74351d`](https://github.com/o2sh/onefetch/commit/d74351d8f042db42a68a51f5f8f05570bbfc4655))
    - Merge pull request #103 from vypxl/master ([`1164959`](https://github.com/o2sh/onefetch/commit/116495923b312ed340ee170a7ef653ac827afd58))
    - change flag name from list to supported ([`ece4a65`](https://github.com/o2sh/onefetch/commit/ece4a658a895bc36abaaae73b011d006951ebd7b))
    - format ([`0f3b82e`](https://github.com/o2sh/onefetch/commit/0f3b82e33cf169a53bacd3eedcdb576de36061a0))
    - add --list, -l flag to list supported languages ([`2e0ac38`](https://github.com/o2sh/onefetch/commit/2e0ac38bcfc03c855cfc469117a8f2e5ad0b740b))
    - Fix Created field ([`91b554c`](https://github.com/o2sh/onefetch/commit/91b554c918e485bf4ad56ea75794fa0fb64717af))
    - Update usage ([`759e41f`](https://github.com/o2sh/onefetch/commit/759e41f78c97245a588004618d786a7b5ba9568a))
    - Merge pull request #101 from andymac-2/center-padding ([`98eba71`](https://github.com/o2sh/onefetch/commit/98eba71d90864c1e634b093fb208529783556c14))
    - Included center pad and double newline at end. ([`faeda05`](https://github.com/o2sh/onefetch/commit/faeda05c5f0ac31f3d6e2ae9dafc5da4c3232c65))
    - Merge pull request #98 from andymac-2/refactor-modules ([`ae693b3`](https://github.com/o2sh/onefetch/commit/ae693b3c9f60fceb99ae9c1419b65b798ba86b66))
    - Updated tests for tokenizer. ([`c790d0c`](https://github.com/o2sh/onefetch/commit/c790d0caae34e009c2fafa4d8885caa2a408bafb))
    - Merge branch 'master' into refactor-modules ([`b5682bf`](https://github.com/o2sh/onefetch/commit/b5682bffddcef43f7331f5d9e7bd2e42b6637f37))
    - Add bold flag to tests, add tests for disabled bold characters ([`9f14637`](https://github.com/o2sh/onefetch/commit/9f1463790dbac38360c3181bf709b8a44ba44cef))
    - Merge branch 'master' into bold-flag ([`37e62c5`](https://github.com/o2sh/onefetch/commit/37e62c5fc900db5c2c175f96fac50c0440f15cba))
    - Add bold parameter to AsciiArt, adjust boldness of logo based on flag ([`539b928`](https://github.com/o2sh/onefetch/commit/539b928fa0feb4bb98b06d2c8fbfbd9aa7233b4a))
    - Merge branch 'master' into bold-flag ([`924a4ed`](https://github.com/o2sh/onefetch/commit/924a4ede12c307af72b6ab417e21b402d900d42b))
    - fixing build by adapting test to take into account boldness #96 ([`e9d3111`](https://github.com/o2sh/onefetch/commit/e9d31114060ea42e6189d2078e5f708546cfad3e))
    - Refactor formatted label function to be a method of Info ([`ded5f48`](https://github.com/o2sh/onefetch/commit/ded5f48656284b9f546649ae28b57ee9302797b7))
    - Adjust boldness of logo based on bold flag ([`dccea3d`](https://github.com/o2sh/onefetch/commit/dccea3dcd2092dabc1f289b9e2e9d8698576f028))
    - Merge branch 'master' into refactor-modules ([`57c7f26`](https://github.com/o2sh/onefetch/commit/57c7f26a64b8ed070a233ffe72a99926f4f9b67c))
    - Removed useless tests ([`0efd3c5`](https://github.com/o2sh/onefetch/commit/0efd3c53166c60096317a48bea2541e752379165))
    - Split code into modules ([`8d1a3c1`](https://github.com/o2sh/onefetch/commit/8d1a3c17054bb3d69c33ee367901a861fd241bae))
    - add two line breaks when (none, none) ([`405937b`](https://github.com/o2sh/onefetch/commit/405937b4b0ca458d2d29bb18b5f6376935484fb9))
    - increase central pad and added missing colors in asccii art ([`cdf4f7a`](https://github.com/o2sh/onefetch/commit/cdf4f7a36f2ab4fb3fe2f143c6e13d444310f7e7))
    - Merge pull request #96 from andymac-2/fix-art-widths ([`ae5f8bf`](https://github.com/o2sh/onefetch/commit/ae5f8bfaada6c2a4baf7b45227e8ae21c4165f24))
    - embolden logo ([`843b122`](https://github.com/o2sh/onefetch/commit/843b1225e7d145112a2838f58331efbcfc0b0e90))
    - remove lines from the top and bottom of the logo and info ([`da28b62`](https://github.com/o2sh/onefetch/commit/da28b628716d819c586c1a6b6b6c42d0d18f7b89))
    - Renamed module ([`ff7814a`](https://github.com/o2sh/onefetch/commit/ff7814ad4bf5d9a0ee21426aa966a5fdaca4813a))
    - Merge branch 'master' into fix-art-widths ([`abb81a2`](https://github.com/o2sh/onefetch/commit/abb81a21a7370f3729a934a056a0716752ca197d))
    - Fixing ascii art widths. ([`f8dff54`](https://github.com/o2sh/onefetch/commit/f8dff546bda8562b05300d019fffac477334e448))
    - Change boldness of info labels based on command line flag ([`397343b`](https://github.com/o2sh/onefetch/commit/397343b56eefbd73262ef43731e9be4cc28c07e1))
    - bold logo by default ([`962dc42`](https://github.com/o2sh/onefetch/commit/962dc42af7b1874db70b2d44482398210e82a57b))
    - prolog ([`ffe4e20`](https://github.com/o2sh/onefetch/commit/ffe4e20af0e78aa8dd454e3c0b1d056f283b8652))
    - inverse colors for tex ([`58a15c9`](https://github.com/o2sh/onefetch/commit/58a15c92f3a332026c314c0a85e53e396e8e0948))
    - Merge pull request #92 from KaindlJulian/tex-support ([`55e12a5`](https://github.com/o2sh/onefetch/commit/55e12a5108db48ff655662d318d7e1f7bd5eb83b))
    - Small ascii file changes ([`8213e6b`](https://github.com/o2sh/onefetch/commit/8213e6b8933d0c19b7d6a01906df77ee2913b203))
    - Add TeX Support ([`d46df84`](https://github.com/o2sh/onefetch/commit/d46df84b60efeac4abe2faf099310c654b35faf0))
    - Merge pull request #88 from spenserblack/master ([`f23aa63`](https://github.com/o2sh/onefetch/commit/f23aa630679c540bd09a35f8553c78f9bef0f1c9))
    - Fix compilation error on Linux ([`0264b7c`](https://github.com/o2sh/onefetch/commit/0264b7c44f16b97399166195259889a3949ce565))
    - Build for multiple OSs ([`1ba0286`](https://github.com/o2sh/onefetch/commit/1ba028659c3492e06e25a21a7d4f54fae744a8af))
    - win10 color fix #86 ([`ad64f9c`](https://github.com/o2sh/onefetch/commit/ad64f9cde2f676b1378c10a0763f128cb06cae4b))
    - Handle multiple prefixes for license detection -- COPYING ([`0fd4d43`](https://github.com/o2sh/onefetch/commit/0fd4d43ad853471d9b09f8e9ddc212ce33e1d383))
    - Merge pull request #85 from amiralies/add-elixir-2 ([`b7a1761`](https://github.com/o2sh/onefetch/commit/b7a1761ddf05685d31c2aa67a22bf9b21e0ae654))
    - Add elixir ([`90c7a7d`](https://github.com/o2sh/onefetch/commit/90c7a7d7106a589e86a39164089cd07422b54803))
    - improved xml art ([`7cd6fe2`](https://github.com/o2sh/onefetch/commit/7cd6fe2d99168d7de23fda1b39b32c8fb025f5b2))
    - Merge pull request #84 from tacrazymage/master ([`0c905c8`](https://github.com/o2sh/onefetch/commit/0c905c829925c98be1ae82b23c951bfe6c92d385))
    - correct second color of xml ([`03cc3de`](https://github.com/o2sh/onefetch/commit/03cc3de179c88a4d5bcbc83b0552a66c0d5842c9))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`ef4148e`](https://github.com/o2sh/onefetch/commit/ef4148e4ddfb508addd6650dee45394b3793c348))
    - remove unused assets ([`a21fe8f`](https://github.com/o2sh/onefetch/commit/a21fe8fc3496ebe6ebe0e6679fea3026ad9b79ec))
    - Update README.md ([`4e11c15`](https://github.com/o2sh/onefetch/commit/4e11c15f4a281920541b5619e6dfb60632737536))
    - split color marker in two lines ([`339aa73`](https://github.com/o2sh/onefetch/commit/339aa7326ff56de2e95c1426f8e3e3a64049fa13))
    - added xml ascii art ([`e7e25ee`](https://github.com/o2sh/onefetch/commit/e7e25ee72eb9d123b9e6d0b8670e57d8ecce691b))
    - Merge pull request #82 from ktsuench/master ([`4d56dbb`](https://github.com/o2sh/onefetch/commit/4d56dbb538454a711981e51761e7d2fcffcf28b6))
    - updated --disable flag behaviour by removing use of vec.contains and using bool struct instead ([`149c3e4`](https://github.com/o2sh/onefetch/commit/149c3e4d6e58c555fde3cf358be572f9e453f8f7))
    - updated behaviour of --disable flag to not error out on unrecognized values ([`450a651`](https://github.com/o2sh/onefetch/commit/450a6511946097cb744418cc4597311e43c62e48))
    - Revert "changed behaviour of --disable flag" ([`1dc797c`](https://github.com/o2sh/onefetch/commit/1dc797ce4b524b775bf5f6e410f6f0c91b0356e7))
    - changed behaviour of --disable flag ([`006d7e8`](https://github.com/o2sh/onefetch/commit/006d7e8d4fa1391c5948d7cb5326841b057efcff))
    - added feature to disable info fields from showing in output ([`fb62e9d`](https://github.com/o2sh/onefetch/commit/fb62e9d306716810a22e856ec0da7b95b38e3f25))
    - Merge changes from fork parent ([`40ae2bd`](https://github.com/o2sh/onefetch/commit/40ae2bd7091c87223e08ab31df8eb2182b724300))
    - Merge pull request #79 from spenserblack/select-colors ([`67616ea`](https://github.com/o2sh/onefetch/commit/67616ea4e998c67f425b705cb62131a07881d560))
    - Add visual marker ([`bbda405`](https://github.com/o2sh/onefetch/commit/bbda405ffc3bc72add98c8b6df817bf870bfb082))
    - Merge pull request #78 from astynax/master ([`6fdcae3`](https://github.com/o2sh/onefetch/commit/6fdcae3fb6b28e2d8b4fcf35452266e11e4f67f4))
    - Allow custom colors via CLI ([`2c08598`](https://github.com/o2sh/onefetch/commit/2c085983d67c5dacf008b3830dfe706317480595))
    - Add logo for Elm language ([`3dd22b4`](https://github.com/o2sh/onefetch/commit/3dd22b475e40d0d0c39c717c4589a01774bfa920))
    - Merge pull request #75 from ktsuench/master ([`79164fd`](https://github.com/o2sh/onefetch/commit/79164fd580d6f2d5f5d85ec49bda4fcc19f3a09c))
    - added in --ascii_language option ([`bc7972e`](https://github.com/o2sh/onefetch/commit/bc7972e5bc6109bb6640a94aa2b316d4c0a793f8))
    - added in strum package ([`277147a`](https://github.com/o2sh/onefetch/commit/277147a6a547a4abf374e614dc012a923517f4be))
    - Merge pull request #72 from ktsuench/master ([`983424d`](https://github.com/o2sh/onefetch/commit/983424d80dc3d1e67aa49635b28502e35234fc81))
    - reworked coffeescript ascii art ([`af89b18`](https://github.com/o2sh/onefetch/commit/af89b18440ef4e395e2b060d56496fec3b91a28a))
    - Merge pull request #71 from estevam31/master ([`a2b1d43`](https://github.com/o2sh/onefetch/commit/a2b1d43152a9df8041ae09e6f93f578d32efd8f6))
    - fixing missing comma ([`f0285a0`](https://github.com/o2sh/onefetch/commit/f0285a06a1779fc1871345d3d1d3fad090ca004c))
    - Merge pull request #68 from MaxJohansen/html-css ([`7e4fd0f`](https://github.com/o2sh/onefetch/commit/7e4fd0f679cc176bfcd215e4969dc40e3658ab93))
    - Add HTML/CSS language support ([`22ed5a2`](https://github.com/o2sh/onefetch/commit/22ed5a28ad6c5b782e08f6836eeeb7bd7c88375f))
    - added in coffeescript ([`b90f95b`](https://github.com/o2sh/onefetch/commit/b90f95be21655b21d7eaf56c40d7e7d406233f7a))
    - Add Vue.js language support ([`f1538c1`](https://github.com/o2sh/onefetch/commit/f1538c105cfda50bbda995965c0f23614a665d49))
</details>

## v1.6.5 (2019-10-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 4 calendar days.
 - 5 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - move creation date to its own line ([`d9bfb73`](https://github.com/o2sh/onefetch/commit/d9bfb73787257f841c69213432264542898527bc))
    - Merge pull request #61 from bojan88/project_creation_date ([`7b31ed5`](https://github.com/o2sh/onefetch/commit/7b31ed5807f1dff66d63ea7fa30460154cb2dda0))
    - Merge pull request #67 from spenserblack/master ([`6a73ae1`](https://github.com/o2sh/onefetch/commit/6a73ae194df4d0e1cdadabf0fe86a2db28f01fbc))
    - Use raw images for screenshots ([`4ac01be`](https://github.com/o2sh/onefetch/commit/4ac01becab21b90f4a4efcd77f2b05acad2887f7))
    - Merge pull request #66 from GooseDB/purescript ([`205bf18`](https://github.com/o2sh/onefetch/commit/205bf18710a6e7ad33a1dd64e7c8d2416b2ac971))
    - tcl && purescript ([`379d0c0`](https://github.com/o2sh/onefetch/commit/379d0c05a573675a490458c62397c68808a9209b))
    - Merge pull request #65 from spenserblack/master ([`c36d5bf`](https://github.com/o2sh/onefetch/commit/c36d5bf4e3d2b46a79abe1bf1dfed23a9f81b5ab))
    - Exclude images from crate ([`70da25c`](https://github.com/o2sh/onefetch/commit/70da25c3cff693783ec7ba816e06a49a00650306))
    - Merge pull request #62 from SamTebbs33/master ([`2c7aa5b`](https://github.com/o2sh/onefetch/commit/2c7aa5ba367dd07525463d1998f22cfe53e2fc93))
    - Add Zig support ([`ef9af43`](https://github.com/o2sh/onefetch/commit/ef9af43b899e9baa272c64ea7d8b73a73b374a16))
    - Merge pull request #64 from jadijadi/erlang_logo ([`b4b0a98`](https://github.com/o2sh/onefetch/commit/b4b0a9870da683e3804407fe9ce1997b8f49e2c6))
    - erlang ascii art is added ([`9311548`](https://github.com/o2sh/onefetch/commit/93115483eadf59d02b28afe2be01cb83ef0aa1e7))
    - Added project creation date ([`8f22ff5`](https://github.com/o2sh/onefetch/commit/8f22ff5362c3a85ff842cd03e2e8f57858bda66c))
    - update image for README ([`0994a5c`](https://github.com/o2sh/onefetch/commit/0994a5cd69e3e5a6f4405fbf3ba307e3d741f603))
    - rollback last change ([`982a65e`](https://github.com/o2sh/onefetch/commit/982a65e79b35fce641ccb78ec0c96eda69346bcb))
    - Merge pull request #58 from Vipul-Bajaj/master ([`a1aff05`](https://github.com/o2sh/onefetch/commit/a1aff056bcdf0accf71ff3b365a2119bf2dd25f8))
    - Add contributors ([`7773f93`](https://github.com/o2sh/onefetch/commit/7773f9319a5add772f2899efc46e83c813ed5f85))
    - Merge pull request #57 from WillyChen123/authors-info ([`05d3788`](https://github.com/o2sh/onefetch/commit/05d3788736629ee0392ff3b25dca5fe62fc1be68))
    - Add number of files to Repository size ([`3ee77f5`](https://github.com/o2sh/onefetch/commit/3ee77f5b3435a28722e706df3e8fabe2f29791eb))
    - Additional info about authors ([`cca43a7`](https://github.com/o2sh/onefetch/commit/cca43a701e2ed513722b55d8e26ee36190c6b6b3))
</details>

## v1.6.0 (2019-10-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 3 calendar days.
 - 83 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - new version ([`868a95f`](https://github.com/o2sh/onefetch/commit/868a95f069e52695de4f7c4ee37114bb63a75350))
    - missing reference to Idris ([`6dd7da5`](https://github.com/o2sh/onefetch/commit/6dd7da54ed69f33398a71f61c3a45696cb4b123a))
    - Merge pull request #54 from Emanon42/master ([`197c56d`](https://github.com/o2sh/onefetch/commit/197c56d5ecd2727a12077bad1f051ce257514362))
    - Update main.rs ([`032d015`](https://github.com/o2sh/onefetch/commit/032d015eb53c19c7e55942064b99a42995ce91c8))
    - Create idris.ascii ([`6a9a056`](https://github.com/o2sh/onefetch/commit/6a9a056827f87a3bb1cd309b3246d84ae34ade65))
    - Merge pull request #53 from arvidboivie/feature/short-commit-hash ([`71f245a`](https://github.com/o2sh/onefetch/commit/71f245a44a27fa25f04ab840ec3cf924401858cf))
    - Shorten commit hash to 7 characters ([`c51b68d`](https://github.com/o2sh/onefetch/commit/c51b68d508a7e4d14dae6dbdb601c595635132e8))
    - Merge pull request #49 from hoop33/master ([`6474be0`](https://github.com/o2sh/onefetch/commit/6474be0f8135b4231dd3db7bae4da4d01ffc6e44))
    - Add support for Objective-C ([`6df25cd`](https://github.com/o2sh/onefetch/commit/6df25cd4e9ed2beec31a412de4d177f25e7dbb0f))
    - Merge pull request #48 from spenserblack/update/tokei/10.0 ([`39b3f05`](https://github.com/o2sh/onefetch/commit/39b3f054101d02cb6d02a2edf01885877c1b5b40))
    - Update tokei to v10.0 ([`66b282c`](https://github.com/o2sh/onefetch/commit/66b282cc532d695f6120014e9b29ce56924b3708))
    - Merge pull request #46 from spenserblack/editorconfig ([`45e86c5`](https://github.com/o2sh/onefetch/commit/45e86c5206bed258c30a3eceff953c592f5550f8))
    - Merge pull request #47 from spenserblack/lang/kotlin ([`59c81c2`](https://github.com/o2sh/onefetch/commit/59c81c21095da4dbf093f90f0a3456a92a44861e))
    - Add colors for Kotlin ([`aea155e`](https://github.com/o2sh/onefetch/commit/aea155e8dcd61df5b95c5d54d2add9c5e60e714b))
    - Add basic Kotlin support ([`4d91437`](https://github.com/o2sh/onefetch/commit/4d91437bcfd33170e4c1afc87d1cc41f890079f6))
    - Add editorconfig ([`400313d`](https://github.com/o2sh/onefetch/commit/400313dfd611d5d9837046625b774c6384493543))
    - Merge pull request #44 from nikofil/master ([`6fbad3b`](https://github.com/o2sh/onefetch/commit/6fbad3b76ac3a8578f326799d0c9ea86d4159b98))
    - Display repository size ([`004e687`](https://github.com/o2sh/onefetch/commit/004e6875a281718e61c0f46dd1bec0002c8916d4))
    - Display current commit and its references' names ([`bde2d86`](https://github.com/o2sh/onefetch/commit/bde2d866b6f57e8df443660dba97d7780877a9a4))
    - Merge pull request #42 from spenserblack/master ([`34bd253`](https://github.com/o2sh/onefetch/commit/34bd253a58056d8b95de96b6393a5e23520dfc1f))
    - Fix unused/deprecated warnings ([`cc2d95c`](https://github.com/o2sh/onefetch/commit/cc2d95c4ab44b215373b6007af6b1ef2af5fc3bd))
    - Use clap to handle command-line arguments ([`36088b7`](https://github.com/o2sh/onefetch/commit/36088b708d0b444a53d5f982ed0d3d269bd94e97))
</details>

## v1.5.5 (2019-07-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 96 calendar days.
 - 97 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare release 1.5.5 ([`64fc1d8`](https://github.com/o2sh/onefetch/commit/64fc1d8e34a32d4f7cefa7a60a83125a7d6f199a))
    - added support for Perl #39 ([`1dd1169`](https://github.com/o2sh/onefetch/commit/1dd11695f3d3d71393d5fa635ffa9bbf8c11dad3))
    - better symetry Nim #37 ([`26515eb`](https://github.com/o2sh/onefetch/commit/26515eb979c2198e20baa392fab365f652d9ad68))
    - added support for Nim #37 ([`20ddd8b`](https://github.com/o2sh/onefetch/commit/20ddd8b2af6c48ebf2233926c8b78fe3088cc2a4))
    - added support for Dart #38 ([`40a23f9`](https://github.com/o2sh/onefetch/commit/40a23f9b7158e586547256a84c977c8cfc96a11b))
    - Merge pull request #35 from jephthai/master ([`827c113`](https://github.com/o2sh/onefetch/commit/827c11334108e9b1bae277a4a4692e9b2bf1c195))
    - Add Forth language ([`57eb0d0`](https://github.com/o2sh/onefetch/commit/57eb0d09a4bfa0bfab33136397f6fd48e7250e38))
    - Merge pull request #32 from vinhnx/master ([`8194d16`](https://github.com/o2sh/onefetch/commit/8194d161e19ab4b04cb2ea6edf9133c8561b6432))
    - Add Swift ASCII art ([`827eadb`](https://github.com/o2sh/onefetch/commit/827eadbc1866218de615ebe054ca34c06d7e58e4))
    - Add Swift lanaguage detection support ([`948638b`](https://github.com/o2sh/onefetch/commit/948638b14098694605049a5f592b8b19584cea1e))
    - Merge pull request #30 from aeter/master ([`2e8d973`](https://github.com/o2sh/onefetch/commit/2e8d9734383ce2b76592f6af132757075385f452))
    - Add assembly detection and ascii image ([`580bead`](https://github.com/o2sh/onefetch/commit/580bead0f06c9ec24239e1d5a2c824606e14b590))
    - Merge pull request #29 from hoop33/master ([`cb75aff`](https://github.com/o2sh/onefetch/commit/cb75aff3a45011ead5dc7d4c8ca824df1326b32c))
    - Separate build and install targets ([`a90439c`](https://github.com/o2sh/onefetch/commit/a90439cd9144d4a629904d3b862274a6b8c449f4))
    - cargo release ([`7ee749d`](https://github.com/o2sh/onefetch/commit/7ee749d513c7666720e7ad3ad1d0695afac93b50))
</details>

## v1.5.4 (2019-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 29 calendar days.
 - 29 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`4f3aab6`](https://github.com/o2sh/onefetch/commit/4f3aab6a8ae1e7bbf359d6e0c9385e9e6d334e91))
    - add last change #28 ([`f0b718e`](https://github.com/o2sh/onefetch/commit/f0b718ec260ebaa1347b702fb78dcbfbef85598e))
    - Update README.md ([`365a484`](https://github.com/o2sh/onefetch/commit/365a4845b403919f0da18fde9ab1e8d3b1400f49))
    - Update README.md ([`a3d1e3f`](https://github.com/o2sh/onefetch/commit/a3d1e3f40c3ab872ebb0065696d300b48315ce34))
    - update cargo version ([`d6d3694`](https://github.com/o2sh/onefetch/commit/d6d36941b1492d654826199a82d2dbf332e4247e))
</details>

## v1.5.3 (2019-03-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`68c8439`](https://github.com/o2sh/onefetch/commit/68c8439e6dfd963dcc329fdc7eefab1764d6efb5))
    - forgot to update error messages #27 ([`14c96dc`](https://github.com/o2sh/onefetch/commit/14c96dcffb9182d20ed0190332d7fb8030a8b0b1))
    - Update README.md ([`ca577ab`](https://github.com/o2sh/onefetch/commit/ca577ab7b556760bc7f82a9f6581ba22e582c349))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`c8a9cc9`](https://github.com/o2sh/onefetch/commit/c8a9cc92e7e97e1ec1ab2493a0defa669cab6a10))
    - Specify path on the command line #27 ([`aac411c`](https://github.com/o2sh/onefetch/commit/aac411c9579b364092a4e2f0e20da33226266f0b))
    - Update README.md ([`9ab8134`](https://github.com/o2sh/onefetch/commit/9ab813461b15f7212aa18632e09a8d1b950ac295))
    - haskell and travis update ([`dff457c`](https://github.com/o2sh/onefetch/commit/dff457ce2123a33e8f3105df1ee1e42381cf4355))
    - R logo improved ([`d015587`](https://github.com/o2sh/onefetch/commit/d0155878546406d6139115f6774d64a69051c020))
    - cargo release ([`6d84aba`](https://github.com/o2sh/onefetch/commit/6d84abaebe2077628ca9dbbf74305d4d0c0ad2a4))
</details>

## v1.5.2 (2019-02-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update README.md ([`50df9c5`](https://github.com/o2sh/onefetch/commit/50df9c567de214defe55798accbc6dc281b23056))
    - Update README.md ([`24a6392`](https://github.com/o2sh/onefetch/commit/24a6392a7b8ab582219cf240964aeb0a37b645f6))
    - add cpp.png ([`02bc32b`](https://github.com/o2sh/onefetch/commit/02bc32bc80c5c23b341fee7ba450b569dfcd4c45))
    - add cpp.png ([`bac1952`](https://github.com/o2sh/onefetch/commit/bac1952c05e1a9de97e51e6bcf2cba443943b2b3))
    - clippy #25 ([`3826fd1`](https://github.com/o2sh/onefetch/commit/3826fd1047bf15338f97e82c058008d3a4f930dc))
    - multiple language overflow and max between info and logo in main for loop #25 ([`14282be`](https://github.com/o2sh/onefetch/commit/14282bef79ae0e77d98bb9f627925fc5ca8dfb2e))
    - cargo release ([`c7e7d58`](https://github.com/o2sh/onefetch/commit/c7e7d5854c59daf65fadb4317def1d9d6aa905ab))
</details>

## v1.5.1 (2019-02-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update logo: haskell ([`4272d01`](https://github.com/o2sh/onefetch/commit/4272d011744d116989a0b59b2728497f9ef31668))
    - improve shell logo ([`2eb13eb`](https://github.com/o2sh/onefetch/commit/2eb13eb1563950c575ddbad7c655b2eb6718d028))
    - update pics ([`cdf78c5`](https://github.com/o2sh/onefetch/commit/cdf78c54a33499cc3be3b788d9e08f3b5120babc))
    - clippy review ([`e297b36`](https://github.com/o2sh/onefetch/commit/e297b3690b070192c0152271fb7a2a18d69b6e66))
    - multiple language stats ([`511323a`](https://github.com/o2sh/onefetch/commit/511323a7874e48c4c14592c8334b2011849941fd))
    - version number snap ([`6e5ca58`](https://github.com/o2sh/onefetch/commit/6e5ca584b43927fd30c0889f90e674d739a960c0))
    - cargo release ([`19c3dc7`](https://github.com/o2sh/onefetch/commit/19c3dc7974df20516c196aaa729ff5e8ec01e09a))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`79e62e1`](https://github.com/o2sh/onefetch/commit/79e62e12a57c2c50df0c912abdf635edcabd7916))
    - .gitignore ([`76e9414`](https://github.com/o2sh/onefetch/commit/76e94146f8e79a5fade763bc5ec6a4ed89244c6a))
</details>

## v1.5.0 (2019-01-30)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 46 calendar days.
 - 46 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update README.md ([`77ee216`](https://github.com/o2sh/onefetch/commit/77ee216a4c08240f9e10dbc5ee97af775e7ffb85))
    - Update README.md ([`48da254`](https://github.com/o2sh/onefetch/commit/48da254c1e13be6713afad39da8453d34ae46fc4))
    - Update README.md ([`305f1ee`](https://github.com/o2sh/onefetch/commit/305f1ee77a88f326853387b3314bfb5abdab033a))
    - Update README.md ([`c427f96`](https://github.com/o2sh/onefetch/commit/c427f96758892a7665c1332d07944857a973c24b))
    - Update README.md ([`77acb07`](https://github.com/o2sh/onefetch/commit/77acb078ea9b3e92384280ee93719d10d178d621))
    - Update README.md ([`7159e79`](https://github.com/o2sh/onefetch/commit/7159e79f9c083882b5e1d51002155e0a98956fe5))
    - add logo ([`170a690`](https://github.com/o2sh/onefetch/commit/170a690e78131c206221681f78e0ddc41d58a6f5))
    - Delete onefetch.png ([`ac5fc38`](https://github.com/o2sh/onefetch/commit/ac5fc38652c931601ecbb3499dee9b9eb3dddef1))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`d3270de`](https://github.com/o2sh/onefetch/commit/d3270de0046633e0fa170632d47ed08a87e23cac))
    - add logo ([`2640c3a`](https://github.com/o2sh/onefetch/commit/2640c3ab8e6df0f53ee99c83352b23e20f9eaa6d))
    - Update README.md ([`3b3fa9d`](https://github.com/o2sh/onefetch/commit/3b3fa9d6963c8b9f7dd951b86d7ba0e4e264c0d9))
    - Update README.md ([`dddec31`](https://github.com/o2sh/onefetch/commit/dddec31f5a248a7d2ccc5500e8fc1b56c9f3855a))
    - Update README.md ([`6cd649c`](https://github.com/o2sh/onefetch/commit/6cd649c24f3753a4e1b55bef3d80b09a5f79e791))
    - add logo ([`b496ad9`](https://github.com/o2sh/onefetch/commit/b496ad9989b919e49291dcd6c482de9020c80383))
    - update assets ([`63d5ac1`](https://github.com/o2sh/onefetch/commit/63d5ac172e6ec601cf62a629364570178a7325af))
    - update assets ([`e205513`](https://github.com/o2sh/onefetch/commit/e205513a9d7e8a20ef936388c012a2b08ea7bc85))
    - detect number of commits #24 ([`1778b9d`](https://github.com/o2sh/onefetch/commit/1778b9d892600a9a2642ab79fb4c83a80a883638))
    - detect version #24 ([`6599801`](https://github.com/o2sh/onefetch/commit/65998017b5111cca0f5992ba48cb78b7c9d02f9b))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`7f4b71c`](https://github.com/o2sh/onefetch/commit/7f4b71c5aa5c32b35adc3d475419b3b6d2ade124))
    - multicolor ascii for haskell/python/Clojure and news ascii for Cpp/Csharp and added support for php ([`7541574`](https://github.com/o2sh/onefetch/commit/75415749137b9063ab44d6626b6370c37c4cf82f))
    - Merge pull request #23 from popey/add-snapcraft ([`24f3f54`](https://github.com/o2sh/onefetch/commit/24f3f54377905e8a4a1b61987dfe93051c425f01))
    - fix logo #22 ([`2019147`](https://github.com/o2sh/onefetch/commit/20191477bf0fa43431bd47903b344cca66abfa3e))
    - JavaScript logo fixed #22 ([`6af8789`](https://github.com/o2sh/onefetch/commit/6af8789f4ad69e3ba8f3b89dc684022eb318bd82))
    - JavaScript added #22 ([`1147100`](https://github.com/o2sh/onefetch/commit/1147100db57b017a9cfbf54e1b2cafa726014512))
    - Add support for building a snap ([`5a6f04d`](https://github.com/o2sh/onefetch/commit/5a6f04d5da208aa9bf198bb0f1462e8d870f054b))
    - Update README.md ([`e79e867`](https://github.com/o2sh/onefetch/commit/e79e867a7fc44e55736fb41dacadc9ad83ccf20a))
    - Update README.md ([`cd1b81a`](https://github.com/o2sh/onefetch/commit/cd1b81afe33161858d354eb0050ce7d3958ffb06))
    - cargo release ([`9885ba9`](https://github.com/o2sh/onefetch/commit/9885ba98479d4a2a618e85c0b15f6a70b36da549))
</details>

## v1.0.5 (2018-12-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 41 calendar days.
 - 42 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - alter rust logo ([`4792027`](https://github.com/o2sh/onefetch/commit/4792027bb7b54700769c1510d65ceb30e7734a95))
    - multicolor java logo ([`1dfb3e0`](https://github.com/o2sh/onefetch/commit/1dfb3e0a578a17155e5e5337e0f09bfb7f1bbd19))
    - add contributor + new screenshots ([`26ed764`](https://github.com/o2sh/onefetch/commit/26ed764e724afeb1d7ef640708706a2f58a37060))
    - Merge pull request #20 from xynxynxyn/master ([`1499618`](https://github.com/o2sh/onefetch/commit/1499618ae7ad1b0b83c0f7186129a70175b310c8))
    - multi color asciis ([`1fc91ba`](https://github.com/o2sh/onefetch/commit/1fc91ba0bec077a80bec3f61eb417283777a6188))
    - Merge pull request #19 from xynxynxyn/master ([`0b7297a`](https://github.com/o2sh/onefetch/commit/0b7297a4ebb288ed24b8a2bec8ee50c0d5430d93))
    - Fix clippy errors ([`71a1f70`](https://github.com/o2sh/onefetch/commit/71a1f7018397a1f08af632cd958125ccc3315813))
    - more adapted error message in case of wrong folder ([`d93fe8a`](https://github.com/o2sh/onefetch/commit/d93fe8afbcb79c823704a975c47c5b10341f0493))
    - Merge pull request #18 from xynxynxyn/master ([`17ea45d`](https://github.com/o2sh/onefetch/commit/17ea45dc7c334885739184e2c9d86628101e92c9))
    - Custom error type and proper error messages ([`809d900`](https://github.com/o2sh/onefetch/commit/809d90026ddb4cf6318ece0dd5d22e4205541bdb))
    - Return Errors instead of process:exit ([`26569e1`](https://github.com/o2sh/onefetch/commit/26569e1f69036ab607272737e08fd141872ada2e))
    - square brackets since unwrap is called anyways ([`9ec4df2`](https://github.com/o2sh/onefetch/commit/9ec4df2564331387df598ec6e8019c1f6e822a72))
    - sort_by_key to reduce clutter ([`2097896`](https://github.com/o2sh/onefetch/commit/209789627a127c60138dc9c8bd4eb7d687ff6542))
    - move color impl to Info instead of language ([`01594b5`](https://github.com/o2sh/onefetch/commit/01594b5bc78fc6495395c6fef451faa4b7a6d84c))
    - use language.color() implemented function instead of get_color(&language) ([`2b1454f`](https://github.com/o2sh/onefetch/commit/2b1454f081c163cdab73c37f679b310022d56cd6))
    - submitting to AUR #17 ([`ba243b8`](https://github.com/o2sh/onefetch/commit/ba243b864a082ab66a02c0468457982643e8a17b))
    - forgot license in metadata... #17 ([`336bbda`](https://github.com/o2sh/onefetch/commit/336bbdaff41a6b71271224c2432a80d8cefb8d0c))
    - preperation for release on crates.io #17 ([`d4ac3b8`](https://github.com/o2sh/onefetch/commit/d4ac3b86a7790d4fccf86f1584274634d7d2faaa))
    - exit(1) instead of return ([`fce7441`](https://github.com/o2sh/onefetch/commit/fce744140f69123f425b4d2ae5eb473ae563c9b8))
    - when not git repo --> exit(1) instead of panic! ([`92d15bf`](https://github.com/o2sh/onefetch/commit/92d15bf79b71d4ca5eec3188b4d274faa83bc024))
    - Update README.md ([`1bd15cb`](https://github.com/o2sh/onefetch/commit/1bd15cbf3b723ffe2829c93fb8872572ba24b1c9))
    - Update README.md ([`cb590c9`](https://github.com/o2sh/onefetch/commit/cb590c96aafe80724ba43af92aa9bb80162673ba))
    - rust screenshot ([`1f92a43`](https://github.com/o2sh/onefetch/commit/1f92a430120f300041af5d53df612f8a42b73387))
    - Update README.md ([`2020738`](https://github.com/o2sh/onefetch/commit/202073852a5bea787f021034bc17246f1362de88))
    - Merge branch 'master' of https://github.com/o2sh/onefetch ([`bf80717`](https://github.com/o2sh/onefetch/commit/bf80717964bb2a79713c994c2908243649824a10))
    - java screenshot ([`69c20ed`](https://github.com/o2sh/onefetch/commit/69c20ed6e378b3164084c144c5cce700b6f74ada))
    - Update README.md ([`fc837fd`](https://github.com/o2sh/onefetch/commit/fc837fdaf6facf72854c7f3e383c1304e771b724))
    - Update README.md ([`9fdcf2e`](https://github.com/o2sh/onefetch/commit/9fdcf2e36d26b46d080f4a1a06fb15d37e727e0e))
    - Update README.md ([`64bdb37`](https://github.com/o2sh/onefetch/commit/64bdb37c0300b4a6fc1451c6e1e80dcc2870776d))
    - Update README.md ([`4dd2fad`](https://github.com/o2sh/onefetch/commit/4dd2fad373f23b79f1a0726f433929e6e6ad7df3))
</details>

## v1.0.0 (2018-11-02)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 54 commits contributed to the release over the course of 48 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - code review ([`3e726c8`](https://github.com/o2sh/onefetch/commit/3e726c8d3d8542cebf1e2543013505940a47be78))
    - Update README.md ([`78617c7`](https://github.com/o2sh/onefetch/commit/78617c7ac106e1f90cbd37cb1dd73a9ff6b9f23f))
    - new assets ([`7f1d102`](https://github.com/o2sh/onefetch/commit/7f1d102b221eab65c35d6c3eb67d10104450f383))
    - Update README.md ([`26a049c`](https://github.com/o2sh/onefetch/commit/26a049cf4c6929d228a8b8606746839a4867cc46))
    - update travis ([`7d8dbd2`](https://github.com/o2sh/onefetch/commit/7d8dbd2e4fa0635466687cb44a5e4f69064a73d9))
    - update travis ([`bcb90dd`](https://github.com/o2sh/onefetch/commit/bcb90dd6ed9db5bc6a512a4ffd1bf49b8f36df7a))
    - Merge pull request #16 from kitlith/license-detection ([`8864385`](https://github.com/o2sh/onefetch/commit/8864385ce18b888f0653401fc6cb3eb0bccf9649))
    - Add basic license detection. ([`bf89e2f`](https://github.com/o2sh/onefetch/commit/bf89e2fd392db4282581c57e9027aa8e3e320e7e))
    - Merge pull request #15 from francesco-dipi/master ([`604b987`](https://github.com/o2sh/onefetch/commit/604b9874783bbe386daced566973da5911a3b0eb))
    - Add Lua ascii logo ([`f346b25`](https://github.com/o2sh/onefetch/commit/f346b2575881038d9499ff6c39240e173a972b28))
    - Merge pull request #13 from JoshBrudnak/master ([`61a223b`](https://github.com/o2sh/onefetch/commit/61a223b1777fba12166e7188dbbc6232ea2f8761))
    - Use last part of the repo url for the repo name ([`f851ece`](https://github.com/o2sh/onefetch/commit/f851eceb688e69ebe404e83218a93d52e415fcdc))
    - Use git2 without default features ([`c5333f2`](https://github.com/o2sh/onefetch/commit/c5333f28ec37252e8853d35092ff420b1d03a740))
    - Merge pull request #12 from zxey/info-detect ([`668c696`](https://github.com/o2sh/onefetch/commit/668c696cb99ba7e5aad3dbf85af6da9f8466c640))
    - Detect authors ([`6f92fc9`](https://github.com/o2sh/onefetch/commit/6f92fc97bb6f1d109052d05f9ce69e93a8b75700))
    - Detect repository name and url ([`6c0f43d`](https://github.com/o2sh/onefetch/commit/6c0f43dc754fdaa8451893d1e49c253c8f3aef3c))
    - Formatted the project using rustfmt ([`a0e2d0a`](https://github.com/o2sh/onefetch/commit/a0e2d0a40e2a5e621e3a43aef6a20eb2b61fe0a0))
    - added rtroxler as contributor ([`a406cc1`](https://github.com/o2sh/onefetch/commit/a406cc10dcf45d207bac0ef2a8b006d2b7876f7c))
    - Merge pull request #11 from rtroxler/master ([`f71b797`](https://github.com/o2sh/onefetch/commit/f71b7975de724b72e2b10a05cc332d5072062029))
    - Grab LOC from tokei and add it to Info ([`2944ee8`](https://github.com/o2sh/onefetch/commit/2944ee8a4284910ab474103d21096aabcaccf7cf))
    - rustfmt ([`1779fb3`](https://github.com/o2sh/onefetch/commit/1779fb30e98b2e2305a68ba12ef8f28877e1ccb0))
    - Merge pull request #10 from cnsumner/master ([`dc5c4fc`](https://github.com/o2sh/onefetch/commit/dc5c4fc793fb70ff5b00636f48f8467e55502392))
    - Add caching to the travis ci config to speed things up ([`7bb3399`](https://github.com/o2sh/onefetch/commit/7bb339952d773d7bda83ada6c275d1a94ccb1969))
    - Add support for typescript ([`314ec0a`](https://github.com/o2sh/onefetch/commit/314ec0a2ccb52c0f3ca933798027475b96c51426))
    - Sort languages in ascending order in various places ([`51b2b9d`](https://github.com/o2sh/onefetch/commit/51b2b9d78b794c369afd4d1a3602444f82867633))
    - add zxey as contributor ([`2f7e689`](https://github.com/o2sh/onefetch/commit/2f7e689d0359d08225fd4b679c24979627d8b3ff))
    - Merge pull request #6 from zxey/lang-detect ([`e145e6d`](https://github.com/o2sh/onefetch/commit/e145e6ddb4edb50110c2490fa1672cfb0c2b99d7))
    - Merge pull request #7 from francesco-dipi/master ([`310e0b0`](https://github.com/o2sh/onefetch/commit/310e0b02e29aaffef948509c525792ce5999e21e))
    - Add Clojure ascii logo ([`9076cb0`](https://github.com/o2sh/onefetch/commit/9076cb09575f68ad5d1e78e47c9e7084e547ae35))
    - Merge pull request #5 from zxey/move-ascii-art ([`440e4a9`](https://github.com/o2sh/onefetch/commit/440e4a9fd49f44a28af18979133366ecb3ebea38))
    - Detect dominant language type ([`b854fa9`](https://github.com/o2sh/onefetch/commit/b854fa9acdcbbf21b4735070f8727106d2d706e0))
    - Move all ascii art to separate files ([`15b766d`](https://github.com/o2sh/onefetch/commit/15b766de72044cf1be761c7ea2956499129cbfdb))
    - Merge pull request #3 from di-wu/r ([`3e512fb`](https://github.com/o2sh/onefetch/commit/3e512fbc6a4aeb3f37c7a98229a2c3946aecaaf4))
    - R ASCII! ([`00162aa`](https://github.com/o2sh/onefetch/commit/00162aa0acb18677c1dabbce6737729eb725b16d))
    - rename ([`8c992f5`](https://github.com/o2sh/onefetch/commit/8c992f548e0080b37fbb77b0b7794a81ac6998c9))
    - add contributors file ([`a6176c6`](https://github.com/o2sh/onefetch/commit/a6176c6def304aff93dee40f89190b81746529c1))
    - Update README.md ([`d25e048`](https://github.com/o2sh/onefetch/commit/d25e04883429752636d2625bae5a2546e5675dd7))
    - Merge branch 'master' onfnfnfnfnf https://github.com/o2sh/onefetch ([`908eb36`](https://github.com/o2sh/onefetch/commit/908eb364a87afce2806dc087f7fdcda20f358089))
    - add ci ([`ed82c88`](https://github.com/o2sh/onefetch/commit/ed82c88f59a0d666227f899fddf8b8ff50ad33d9))
    - Update README.md ([`5be5e6f`](https://github.com/o2sh/onefetch/commit/5be5e6f4c5e9b24721d5b5f1588181e7c2b2e409))
    - Create LICENSE ([`046bfea`](https://github.com/o2sh/onefetch/commit/046bfea9d0147ee51d817fc869f6dcef30c93e0e))
    - preview images ([`5eaae5b`](https://github.com/o2sh/onefetch/commit/5eaae5bb1fad24298833bb19bfa1f9f7ea24b17e))
    - switch colors ([`e028804`](https://github.com/o2sh/onefetch/commit/e0288045493dda817910dcae5c796c581b3dace2))
    - color ([`c7ec9f7`](https://github.com/o2sh/onefetch/commit/c7ec9f78205009000570692e665b3ed8c2af1b38))
    - empty line beginning ([`d1bb3dc`](https://github.com/o2sh/onefetch/commit/d1bb3dc6e7ae76b601da28257b3a83516aa0e286))
    - logo and info side by side ([`e19fb33`](https://github.com/o2sh/onefetch/commit/e19fb3300bb52ecd9b27129a0c3ad8f43ff1a3db))
    - ascii logos ([`8062d92`](https://github.com/o2sh/onefetch/commit/8062d926c7d0e1d871bd2f5204f3e175edb13949))
    - fields info ([`6eb2a8d`](https://github.com/o2sh/onefetch/commit/6eb2a8d2af313aa2b8b911ce53c02af0451143c9))
    - barely trying ([`ac35d3c`](https://github.com/o2sh/onefetch/commit/ac35d3cc0f9c9563a9978d405cd427c7a7e048ed))
    - color output ([`668a3d7`](https://github.com/o2sh/onefetch/commit/668a3d7f54f97b0270210d7e1c88f92a7a87e522))
    - enum andd struct ([`8135e95`](https://github.com/o2sh/onefetch/commit/8135e95cda56d0568160b1b4eb27cd1f59dee0df))
    - side by side ([`d90f86d`](https://github.com/o2sh/onefetch/commit/d90f86dab8d854349729eae9725d7ffeaa099b64))
    - init project ([`8556f9a`](https://github.com/o2sh/onefetch/commit/8556f9af5c5b61154502f3f92cdc4201de6a61ab))
    - Initial commit ([`cc46f1d`](https://github.com/o2sh/onefetch/commit/cc46f1d1d143e0ddac139d542f550c2cb99d3cd7))
</details>

