use clap::Clap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Example: git-releaser -r egilsster/test -v [major|minor|patch] -f package.json -t $GITHUB_TOKEN
#[derive(Clap)]
#[clap(version = VERSION)]
pub struct CliArgs {
    /// The Github project to create a release of
    #[clap(short, long = "repo", value_name = "REPO", required = true)]
    pub repo: String,
    /// Which type of release to create
    #[clap(short, long = "type", value_name = "TYPE", default_value = "minor")]
    pub version_type: String,
    /// Which branch of the repository to use
    #[clap(short = 'f', long = "file", value_name = "FILE", required = true)]
    pub version_file: String,
    /// The Github personal access token
    #[clap(short = 't', long = "token", value_name = "TOKEN")]
    pub personal_token: String,
    /// Specifies the main branch the project uses
    #[clap(
        short = 'b',
        long = "branch",
        value_name = "BRANCH",
        default_value = "main"
    )]
    pub main_branch: String,
}
