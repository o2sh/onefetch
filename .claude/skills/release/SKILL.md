---
name: release
description: Prepare a new onefetch release locally — run the CI checks, update CHANGELOG.md, bump the version and MSRV in Cargo.toml, install the binary and regenerate the man page. Use when the user asks to cut, prepare or do a release / version bump. Requires the new version as argument (e.g. `/release 3.3.0`).
---

# Release onefetch

Prepare a release in the working tree. **Do not commit, tag or push** — the user reviews and commits the result themselves.

Argument (required): the new version, e.g. `3.3.0`. If it's missing or isn't a valid `X.Y.Z` version, stop before doing anything and ask the user for it — don't guess or propose one.

Stop and report at the first failing step; don't paper over failures.

## 1. CI checks

Mirror `.github/workflows/ci.yml` (`cargo test` builds the code, and `make install` does the release build later):

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## 2. Update CHANGELOG.md (don't commit)

- Last release: `git describe --abbrev=0 --tags`.
- Go through **every commit** since then, not just PRs: `git log <tag>..HEAD --no-merges --format='%H %s'`. Look at the diff (`git show --stat <sha>`) when the subject doesn't say enough.
  - Squash-merged PRs end in `(#NNNN)`: link the PR. Don't go by the title alone: read the PR description (`gh pr view NNNN` or the `body` from `curl -s https://api.github.com/repos/o2sh/onefetch/pulls/NNNN`) and its diff, and include every user-visible change, especially breaking ones (renamed or removed flags, changed defaults or behavior).
  - Commits pushed straight to main: link the full commit hash instead, e.g. `- Update the license cache in <full sha>`. Group related commits into one entry with several hashes.
- Add a new section at the top, below the header, matching the existing format, without authors:

  ```markdown
  ## X.Y.Z (YYYY-MM-DD)

  ### New Features

  - Short description in https://github.com/o2sh/onefetch/pull/NNNN
  - Short description in <full commit sha>
  ```

  Use today's date. Reuse the section headings already used in the file (`Breaking Changes`, `New Features`, `Bug Fixes`, `CI/CD`, `Chores`, …), include only those that have entries, and rewrite PR titles into readable sentences like past entries.
- Write for the people who use onefetch: one short line per change, saying what changed for them in plain words. For a breaking change, say what to use instead. Leave out changes users never see (CI, docs, refactors, repo tooling).
- Leave out dependency-bot updates (renovate/dependabot `chore(deps)` PRs) unless they're user-facing (e.g. an MSRV bump), and commits that change nothing meaningful (e.g. a comment-only `fix`).

## 3. Bump version

- Set `version = "X.Y.Z"` in `[package]` of `Cargo.toml`.
- Run `cargo update --workspace` so `Cargo.lock` picks up the new version (otherwise `make install` fails because of `--locked`).

## 4. Update MSRV

- Find the real minimum with `cargo msrv find --min <current rust-version>` if `cargo-msrv` is installed. If it isn't, ask the user whether to install it (`cargo install cargo-msrv --locked`) or keep the current value.
- Update `rust-version` in `Cargo.toml` only if it changed, and mention an MSRV bump in the changelog.

## 5. Install

```bash
make install          # cargo install --path . --locked
```

Check `onefetch --version` prints the new version (it must, for the man page below).

## 6. Regenerate the man page

```bash
help2man onefetch --no-info -n "Command-line Git information tool" >docs/onefetch.1
```

## Finish

Show `git status` / `git diff --stat` (expect `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `docs/onefetch.1`) and summarize: new version, MSRV (changed or not), and the changelog section. Remind the user nothing is committed.
