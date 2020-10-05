#[macro_use]
extern crate eyre;
#[macro_use]
extern crate log;

extern crate semver;

mod changelog_gen;
mod commit;
mod git;
mod update_version;

use crate::git::in_git_repository;
use crate::update_version::{map_version_type, VersionType};
use env_logger::Env;
use eyre::Result;
use serde_json::Value;
use std::fs;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    version_type: String,
    // version_file: String, // package.json, Cargo.toml
}

// REF https://github.com/github-changelog-generator/github-changelog-generator
// REF https://docs.rs/git2/0.13.8/git2/struct.Repository.html

// Could probably have a config for main branch and stuff

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::from_args();
    let CliArgs {
        version_type,
        // version_file,
    } = args;
    let log_env = Env::default().default_filter_or("info");
    env_logger::from_env(log_env)
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();

    in_git_repository()?;

    let main_branch = "main";
    let version_type = map_version_type(&version_type);

    let change_gen = changelog_gen::ChangelogGenerator {};

    // git::ensure_no_changes();

    // info!("⚠️  Disabling status checks on the main branch");

    let file_path = "package.json";

    // TODO(egilsster): Support reading Cargo.toml

    // 1. Get current version value
    let ver_file = fs::read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&ver_file).unwrap();
    let current_ver = v["version"].as_str().unwrap();
    info!("📝 Current version is {}", current_ver);
    let new_ver = update_version::update_version(current_ver, version_type);

    // 2. Get the new version value
    let tag_ver = &update_version::update_version_file(file_path, &new_ver);
    // 3. Commit version file change and push that plus the new tag
    git::add_files(&["package.json"])?;
    git::commit(&format!("chore: releasing {}", tag_ver))?;

    // 4. Generate a changelog, stage the CHANGELOG.md, commit that and push
    let changelog = change_gen.generate_changelog(main_branch, tag_ver).await?;
    git::tag(tag_ver)?; // tagged commit, new version is name and version
    git::add_files(&["CHANGELOG.md"])?;
    git::commit("docs: updating changelog [ci skip]")?;

    // 5. Bump the working release number to prerelease
    let ver_file = fs::read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&ver_file).unwrap();
    let current_ver = v["version"].as_str().unwrap();
    let new_pre_ver = update_version::update_version(current_ver, VersionType::Prerelease);
    let pre_ver = update_version::update_version_file(file_path, &new_pre_ver);

    // 6. Commit and push updated package.json file
    git::add_files(&["package.json"])?;
    git::commit(&format!(
        "chore: beginning development on {} [ci skip]",
        pre_ver
    ))?;
    git::push(main_branch)?;
    git::push_tag(tag_ver)?;

    // info!("✅ Enabling status checks on the main branch");

    info!(
        "📖 Here are the changes for {}:\n{}",
        new_ver,
        change_gen.compact_changelog(changelog)
    );

    info!("🚀 {} has shipped!", tag_ver);

    Ok(())
}
