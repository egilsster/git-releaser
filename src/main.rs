#[macro_use]
extern crate eyre;
#[macro_use]
extern crate log;

extern crate semver;

mod changelog_gen;
mod commit;
mod git;
mod github;
mod update_version;
mod version_file;

use crate::changelog_gen::ChangelogGenerator;
use crate::git::in_git_repository;
use crate::github::GithubClient;
use crate::update_version::{map_version_type, update_version, VersionType};
use crate::version_file::VersionFile;
use dialoguer::Confirm;
use env_logger::Env;
use eyre::Result;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "\ngit-releaser -r egilsster/test -t [major|minor|patch] -f package.json -b main -p $GITHUB_TOKEN"
)]
struct CliArgs {
    #[structopt(short = "r", long = "repo")]
    repo: String,

    #[structopt(short = "t", long = "type")]
    version_type: String,

    #[structopt(short = "f", long = "file")]
    version_file: String,

    #[structopt(short = "p", long = "personal-token")]
    personal_token: String,

    #[structopt(short = "b", long)]
    main_branch: String,
}

// REF https://github.com/github-changelog-generator/github-changelog-generator

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::from_args();
    let CliArgs {
        repo,
        version_type,
        version_file,
        personal_token,
        main_branch,
    } = args;
    let log_env = Env::default().default_filter_or("info");
    env_logger::from_env(log_env)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    in_git_repository()?;

    let gh_client = GithubClient::new(personal_token)?;

    let version_type = map_version_type(&version_type)?;
    let change_gen = ChangelogGenerator::new();

    let mut version_file = VersionFile::new(&version_file)?;

    // 1. Get current version value
    let current_ver = version_file.get_version_value();
    info!("📝 Current version is v{}", current_ver);

    // 2. Get the new version value
    let new_ver = &update_version(current_ver.to_owned(), version_type)?;
    debug!("📝 New version is v{}", new_ver);

    let prompt_text = format!("Do you want to release v{}?", new_ver);
    if !Confirm::new().with_prompt(prompt_text).interact()? {
        std::process::exit(0);
    }

    version_file.update_version_file(new_ver)?;

    // 3. Commit version file change and push that plus the new tag
    git::add_files(version_file.get_tracked_files())?;
    git::commit(&format!("chore: releasing {}", new_ver))?;

    // 4. Generate a changelog, stage the CHANGELOG.md, commit that and push
    let changelog = change_gen.generate_changelog(&main_branch, new_ver).await?;
    let new_git_tag = &format!("v{}", new_ver.to_string());
    git::tag(new_git_tag)?; // tagged commit, new version is name and version
    git::add_files(vec!["CHANGELOG.md".to_owned()])?;
    git::commit("docs: updating changelog [ci skip]")?;

    // 5. Bump the working release number to prerelease
    let current_ver = version_file.get_version_value();
    let pre_ver = &update_version(current_ver.to_owned(), VersionType::Prerelease)?;
    version_file.update_version_file(pre_ver)?;

    // 6. Commit and push updated package.json file
    git::add_files(version_file.get_tracked_files())?;
    git::commit(&format!(
        "chore: beginning development on {} [ci skip]",
        pre_ver
    ))?;
    info!("📡 Pushing updates");
    git::push(&main_branch)?;
    git::push_tag(new_git_tag)?;

    info!("🧾 Creating a GitHub release");
    let tag_commit = git::get_commit_for_tag(new_git_tag)?;
    gh_client
        .create_new_release(
            &repo,
            new_git_tag,
            &tag_commit,
            &change_gen.markdown_changelog(&changelog, None),
        )
        .await?;

    info!(
        "📖 Here are the changes for v{}:\n{}",
        new_ver,
        change_gen.compact_changelog(&changelog)
    );

    info!("🚀 v{} has shipped!", new_ver);

    Ok(())
}
