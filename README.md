# git-releaser

![Build status](https://github.com/egilsster/git-releaser/workflows/build/badge.svg?branch=main)
![Audit status](https://github.com/egilsster/git-releaser/workflows/audit/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/egilsster/git-releaser/branch/main/graph/badge.svg?token=HDVQ70Y2KZ)](https://codecov.io/gh/egilsster/git-releaser)

*In development*

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Currently supports only versions from `package.json`.

## Usage

```sh
[RUST_LOG="debug"] cargo run {patch, minor, major}
```

Example

```txt
λ git-releaser minor
[INFO] 📝 Current version is 0.8.1-0
[INFO] 📎 Generating a changelog for v0.9.0
[INFO] 📖 Here are the changes for 0.9.0:
 - feat: added a new feature
 - fix: fixed pesky bugs
[INFO] 🚀 v0.9.0 has shipped!
```

## TODO

- Support Cargo.toml
- Create a Github release with the new changelog
- Signed commits
- Improve error handling
- Unit test all the things
- Read PR information for the repo instead of just git commits
- Prompt to stash local changes so the working dir is clean during the release process
- Add confirmation on the release that will be created
