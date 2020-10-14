# git-releaser

![Build status](https://github.com/egilsster/git-releaser/workflows/build/badge.svg?branch=main)
![Audit status](https://github.com/egilsster/git-releaser/workflows/audit/badge.svg?branch=main)
[![codecov](https://codecov.io/gh/egilsster/git-releaser/branch/main/graph/badge.svg?token=HDVQ70Y2KZ)](https://codecov.io/gh/egilsster/git-releaser)

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Supports TOML and JSON version files.

## Usage

**Requires a GitHub [personal access token](https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token)**

```sh
git-releaser \
  -r <org>/<repo> \
  -t [patch|minor|major] \
  -f [package.json|Cargo.toml] \
  -p $GITHUB_TOKEN \
  -b main
```

Example

```sh
git-releaser \
  -r egilsster/node-api \
  -t minor \
  -f package.json \
  -p $GITHUB_TOKEN \
  -b main
```

See `git-releaser --help` for more information on usage.

### Example

```txt
λ git-releaser -r egilsster/test -t minor -f package.json -b main -p $GITHUB_TOKEN
📝 Current version is v0.8.1-0
Do you want to release v0.9.0? yes
📎 Generating a changelog for v0.9.0
📡 Pushing updates
🧾 Creating a GitHub release
📖 Here are the changes for v0.9.0:
 - feat: added a new feature
 - fix: fixed pesky bugs
🚀 v0.9.0 has shipped!
```
