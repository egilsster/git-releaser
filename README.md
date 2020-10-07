# git-releaser

![Build status](https://github.com/egilsster/git-releaser/workflows/build/badge.svg?branch=main)
![Audit status](https://github.com/egilsster/git-releaser/workflows/audit/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/egilsster/git-releaser/branch/main/graph/badge.svg?token=HDVQ70Y2KZ)](https://codecov.io/gh/egilsster/git-releaser)

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Supports TOML and JSON version files.

## Usage

```sh
[RUST_LOG="debug"] cargo run {patch, minor, major} [package.json|Cargo.toml]
```

Example

```txt
λ git-releaser minor Cargo.toml
[INFO] 📝 Current version is 0.8.1-0
Do you want to release 0.9.0? yes
[INFO] 📎 Generating a changelog for 0.9.0
[INFO] 📡 Pushing updates
[INFO] 📖 Here are the changes for 0.9.0:
 - feat: added a new feature
 - fix: fixed pesky bugs
[INFO] 🚀 0.9.0 has shipped!
```
