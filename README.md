# git-releaser

*In development*

A CLI tool to that creates a git tag, a changelog and a git release, all in one command.

Currently supports only versions from `package.json`.

## Usage

```sh
cargo run ORG/REPO {patch, minor, major} GITHUB_TOKEN
```

## TODO

- [ ] Make CLI actually do something
- [ ] Generate changelog from git
- [ ] Support signed commits
- [ ] Nice error handling
- [ ] Add more TODO items
