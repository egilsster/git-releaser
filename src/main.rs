extern crate semver;

mod changelog_gen;
mod git;
mod update_version;

use crate::update_version::{map_version_type, VersionType};
use serde_json::Value;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    repo: String,
    version_type: String,
    github_api_token: String,
}

// REF https://github.com/github-changelog-generator/github-changelog-generator
// REF https://docs.rs/git2/0.13.8/git2/struct.Repository.html

// I need to have 3 commits because the tag uses the version number which has proper sem ver form
// then I need to update to a pre-release version again so I am not adding commits with the released version
//
// or do I? I can probably perform work with the same version number, that means when running you are
// essentially on the latest released plus any new changes.. hmm never mind, makes more sense having
// a pre-release number since it is the thing that WILL be a new release
fn main() {
    let args = CliArgs::from_args();
    let CliArgs {
        repo,
        version_type,
        github_api_token,
    } = args;
    let version_type = map_version_type(&version_type);

    git::ensure_no_changes();

    println!("⚠️  Disabling status checks on the main branch");

    let file_path = "package.json";

    // TODO: Validation and error handling
    // Support reading version.txt
    let ver_file = fs::read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&ver_file).unwrap();
    let current_ver = v["version"].as_str().unwrap();
    println!("📝 Current version is {}", current_ver);
    let new_ver = update_version::update_version(current_ver, version_type);

    let tag_ver = &update_version::update_version_file(file_path, &new_ver);
    git::commit(&format!("chore: releasing {}", tag_ver));
    git::tag(tag_ver); // tagged commit, new version is name and version
    git::push();
    git::push_tag(tag_ver);

    let ver_file = fs::read_to_string(file_path).unwrap();
    let v: Value = serde_json::from_str(&ver_file).unwrap();
    let current_ver = v["version"].as_str().unwrap();
    println!("📝 Current version is {}", current_ver);
    let new_ver = update_version::update_version(current_ver, VersionType::Prerelease);

    let pre_ver = update_version::update_version_file(file_path, &new_ver);
    git::commit(&format!(
        "chore: beginning development on {} [ci skip]",
        pre_ver
    ));
    git::push();

    changelog_gen::generate_changelog(current_ver, tag_ver);
    git::commit("docs: updating changelog [ci skip]");
    git::push();

    println!("✅ Enabling status checks on the main branch");

    println!("🚀 {} has shipped!", tag_ver);
    // println!("Print a link to the commit or some useful statistics, maybe even the changelog in a compact form")
}
