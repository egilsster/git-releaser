# git-releaser

![Build status](https://github.com/egilsster/git-releaser/workflows/build/badge.svg?branch=main)

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Supports TOML and JSON version files.

## Installing

Homebrew

```sh
brew tap egilsster/git-releaser
brew install git-releaser
```

`wget`

```sh
λ TAG=v0.1.0 && wget https://github.com/egilsster/git-releaser/releases/download/$TAG/git-releaser-x86_64-apple-darwin.tar.gz
λ tar xf git-releaser-x86_64-apple-darwin.tar.gz -C /usr/local/bin
λ git-releaser --version
git-releaser 0.1.0
```

## Usage

**Requires a GitHub [personal access token](https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/creating-a-personal-access-token)**

```sh
git-releaser \
  -r <org>/<repo> \
  -v [patch|minor|major] \
  -f [package.json|Cargo.toml] \
  -t $GITHUB_TOKEN \
  -b main
```

Example

```sh
git-releaser \
  -r egilsster/node-api \
  -v minor \
  -f package.json \
  -t $GITHUB_TOKEN \
  -b main
```

See `git-releaser --help` for more information on usage.

### Example

```txt
λ git-releaser -r egilsster/test -v minor -f package.json -b main -t $GITHUB_TOKEN
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
