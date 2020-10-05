use crate::commit::Commit;
use crate::git::{commits_in_log, first_commit, last_tag};
use chrono::prelude::*;
use eyre::Result;
use std::fs;

pub struct ChangelogGenerator {}

impl ChangelogGenerator {
    // Generates a changelog between the current version and the new version
    pub async fn generate_changelog(
        &self,
        main_branch: &str,
        new_ver: &str,
    ) -> Result<Vec<Commit>> {
        println!("📎 Generating a changelog for {}", new_ver);

        // Find last tag
        let commits = match last_tag()? {
            Some(tag) => {
                println!("📝 Last version is {}", tag);
                // Get commits from last tag
                let range = format!("{}..{}", tag, main_branch);
                commits_in_log(&[range])?
            }
            None => {
                // No tag, get the first commit instead
                let first_commit = first_commit()?;
                let range = format!("{}..{}", first_commit, main_branch);
                commits_in_log(&[range])?
            }
        };

        self.update_changelog(&commits, new_ver)?;

        Ok(commits)
    }

    pub fn update_changelog(&self, commits: &[Commit], version: &str) -> Result<bool> {
        let local: DateTime<Local> = Local::now();
        let current_date = local.date().format("%Y-%m-%d").to_string(); // e.g. 2020-10-04
        let version_header = format!("## {} ({})", version, current_date);

        let change_list = if commits.is_empty() {
            "No commits since last version".to_string()
        } else {
            commits
                .iter()
                .map(|commit| format!("- {}\n", commit.compact()))
                .collect::<String>()
        };

        let entry = format!("{}\n\n{}", version_header, change_list);

        self.write_changelog(version, entry)
    }

    // TODO(egilsster): When the header is not exactly what I expect, nothing gets
    // added to the changelog, could probably just find a lvl1 header and replace it
    // with what I want. This also applies if the file is empty
    pub fn write_changelog(&self, version: &str, new_entry: String) -> Result<bool> {
        println!("Add {} to CHANGELOG.md", version);

        let header = "# CHANGELOG\n\n";

        let file_path = "CHANGELOG.md";
        let changelog_file_contents = fs::read_to_string(file_path);

        // File does probably not exist, create an empty file
        if changelog_file_contents.is_err() {
            fs::write(file_path, header).unwrap();
        }

        let changelog_file_contents = fs::read_to_string(file_path).unwrap();
        if changelog_file_contents.contains(&format!("## {}", version)) {
            return Err(eyre!("Version entry already in CHANGELOG.md"));
        }

        let entry_to_insert = format!("{}{}\n", header, new_entry);
        let new_contents = changelog_file_contents.replace(header, &entry_to_insert);

        let write_res = fs::write(file_path, new_contents).map_err(|e| eyre!(e.to_string()));

        Ok(write_res.is_ok())
    }

    /// Creates a compact output of commits for the CLI to print in the terminal
    pub fn compact_changelog(&self, commits: Vec<Commit>) -> String {
        // This fn can be extended to display stats and other things
        if commits.is_empty() {
            return "No commits since last version".to_string();
        }

        commits
            .into_iter()
            .map(|commit| format!(" - {}\n", commit.compact()))
            .collect::<String>()
    }
}
