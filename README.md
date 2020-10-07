# git-releaser

![Build status](https://github.com/egilsster/git-releaser/workflows/build/badge.svg?branch=main)
![Audit status](https://github.com/egilsster/git-releaser/workflows/audit/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/egilsster/git-releaser/branch/main/graph/badge.svg?token=HDVQ70Y2KZ)](https://codecov.io/gh/egilsster/git-releaser)

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Supports TOML and JSON version files.

## Usage

```sh
git-releaser -t [patch|minor|major] -f [package.json|Cargo.toml] --main-branch main [--log-level debug]
```

Example

```sh
git-releaser -t minor -f package.json -b main
```

See `git-releaser --help` for more information on usage.

### Example

```txt
λ git-releaser minor Cargo.toml
📝 Current version is v0.8.1-0
Do you want to release v0.9.0? yes
📎 Generating a changelog for v0.9.0
📡 Pushing updates
📖 Here are the changes for v0.9.0:
 - feat: added a new feature
 - fix: fixed pesky bugs
🚀 v0.9.0 has shipped!
```
