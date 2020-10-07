#[macro_use]
extern crate eyre;
#[macro_use]
extern crate log;

extern crate semver;

mod changelog_gen;
mod commit;
mod git;
mod update_version;
mod version_file;

use crate::changelog_gen::ChangelogGenerator;
use crate::git::in_git_repository;
use crate::update_version::{map_version_type, update_version, VersionType};
use crate::version_file::VersionFile;
use env_logger::Env;
use eyre::Result;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    version_type: String,
    version_file: String, // list supported version files in help
}

// REF https://github.com/github-changelog-generator/github-changelog-generator

// Could probably have a config for main branch and stuff
static MAIN_BRANCH: &str = "main";

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::from_args();
    let CliArgs {
        version_type,
        version_file,
    } = args;
    let log_env = Env::default().default_filter_or("info");
    env_logger::from_env(log_env)
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();

    in_git_repository()?;

    let version_type = map_version_type(&version_type);
    let change_gen = ChangelogGenerator::new();

    // git::ensure_no_changes();

    let mut version_file = VersionFile::new(version_file)?;

    // 1. Get current version value
    let current_ver = version_file.get_version_value();
    info!("📝 Current version is {}", current_ver);

    // 2. Get the new version value
    let new_ver = &update_version(current_ver.to_owned(), version_type)?;

    version_file.update_version_file(new_ver)?;

    // 3. Commit version file change and push that plus the new tag
    git::add_files(version_file.get_tracked_files())?;
    git::commit(&format!("chore: releasing {}", new_ver))?;

    // 4. Generate a changelog, stage the CHANGELOG.md, commit that and push
    let changelog = change_gen.generate_changelog(MAIN_BRANCH, new_ver).await?;
    git::tag(&new_ver.to_string())?; // tagged commit, new version is name and version
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
    git::push(MAIN_BRANCH)?;
    git::push_tag(&new_ver.to_string())?;

    info!(
        "📖 Here are the changes for {}:\n{}",
        new_ver,
        change_gen.compact_changelog(changelog)
    );

    info!("🚀 {} has shipped!", new_ver);

    Ok(())
}
