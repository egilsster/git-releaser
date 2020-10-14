use crate::commit::Commit;
use crate::git::{commits_in_log, first_commit, last_tag};
use chrono::prelude::*;
use eyre::Result;
use semver::Version;
use std::fs;

static CHANGELOG_FILE_PATH: &str = "CHANGELOG.md";
static CHANGELOG_HEADER: &str = "# CHANGELOG\n\n";

pub struct ChangelogGenerator {}

impl ChangelogGenerator {
    pub fn new() -> Self {
        ChangelogGenerator {}
    }

    // Generates a changelog between the current version and the new version
    pub async fn generate_changelog(
        &self,
        main_branch: &str,
        new_ver: &Version,
    ) -> Result<Vec<Commit>> {
        info!("📎 Generating a changelog for v{}", new_ver);

        // Find last tag
        let commits = match last_tag()? {
            Some(tag) => {
                debug!("📝 Last version is v{}", tag);
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

    pub fn update_changelog(&self, commits: &[Commit], version: &Version) -> Result<bool> {
        let entry = self.markdown_changelog(commits, Some(version));
        self.write_changelog(version, entry)
    }

    /// Inserts the new changelog entry below the main header to
    /// maintain ascending order by date.
    pub fn insert_entry(
        &self,
        contents: String,
        version: &Version,
        new_entry: String,
    ) -> Result<String> {
        if contents.contains(&format!("## v{}", version)) {
            return Err(eyre!("Version entry already in CHANGELOG.md"));
        }
        if !contents.starts_with("# CHANGELOG") {
            return Err(eyre!("CHANGELOG.md must start with '# CHANGELOG'"));
        }
        let entry_to_insert = format!("{}{}\n", CHANGELOG_HEADER, new_entry);
        let new_contents = contents.replace(CHANGELOG_HEADER, &entry_to_insert);

        Ok(new_contents)
    }

    pub fn read_changelog_contents(&self) -> Result<String> {
        // File does probably not exist when it can not be read
        // so create a file with the header
        if fs::read_to_string(CHANGELOG_FILE_PATH).is_err() {
            fs::write(CHANGELOG_FILE_PATH, CHANGELOG_HEADER)?;
        }

        let contents = fs::read_to_string(CHANGELOG_FILE_PATH)?;
        if contents.is_empty() {
            fs::write(CHANGELOG_FILE_PATH, CHANGELOG_HEADER)?;
        }

        let contents = fs::read_to_string(CHANGELOG_FILE_PATH)?;
        Ok(contents)
    }

    /// Ensures the changelog is valid and injects the new changelog entry
    /// to the top of the file, below the header.
    pub fn write_changelog(&self, version: &Version, new_entry: String) -> Result<bool> {
        debug!("Add {} to CHANGELOG.md", version);

        let changelog_file_contents = self.read_changelog_contents()?;
        let updated_contents = self.insert_entry(changelog_file_contents, version, new_entry)?;
        let write_res =
            fs::write(CHANGELOG_FILE_PATH, updated_contents).map_err(|e| eyre!(e.to_string()));

        Ok(write_res.is_ok())
    }

    /// Creates a changelog in markdown format.
    pub fn markdown_changelog(&self, commits: &[Commit], version: Option<&Version>) -> String {
        let current_date = Local::now().date().format("%Y-%m-%d").to_string(); // e.g. 2020-10-04
        let version_header = match version {
            Some(version) => format!("## v{} ({})", version, current_date),
            None => format!("## {}", current_date),
        };

        let change_list = if commits.is_empty() {
            "No commits since last version\n".to_string()
        } else {
            commits
                .iter()
                .map(|commit| format!("- {}\n", commit.compact()))
                .collect::<String>()
        };

        format!("{}\n\n{}", version_header, change_list)
    }

    /// Creates a compact output of commits for the CLI to print in the terminal.
    pub fn compact_changelog(&self, commits: &[Commit]) -> String {
        // This fn can be extended to display stats and other things
        if commits.is_empty() {
            return "No commits since last version".to_string();
        }

        commits
            .iter()
            .map(|commit| format!(" - {}\n", commit.compact()))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commit;

    #[test]
    fn test_update_changelog() {
        //
    }

    #[test]
    fn test_insert_entry_empty() {
        let change_gen = ChangelogGenerator {};

        let log = "".to_owned();
        let version = &Version::parse("0.1.2").unwrap();
        let new_entry = "## v0.1.2 (2020-10-05)\n\n- change 1\nchange 2".to_string();

        let res = change_gen.insert_entry(log, version, new_entry.to_string());
        assert!(res.is_err());
    }

    #[test]
    fn test_insert_entry_already_exists() {
        let change_gen = ChangelogGenerator {};

        let log = "# CHANGELOG\n\n## v0.1.2 (2020-10-04)".to_owned();
        let version = &Version::parse("0.1.2").unwrap();
        let new_entry = "## v0.1.2 (2020-10-05)\n\n- change 1\nchange 2".to_string();

        let res = change_gen.insert_entry(log, version, new_entry.to_string());
        assert!(res.is_err());
    }

    #[test]
    fn test_insert_entry_incorrect_header() {
        let change_gen = ChangelogGenerator {};

        let log = "# RELEASES".to_owned();
        let version = &Version::parse("0.1.2").unwrap();
        let new_entry = "## v0.1.2 (2020-10-05)\n\n- change 1\nchange 2".to_string();

        let res = change_gen.insert_entry(log, version, new_entry.to_string());
        assert!(res.is_err());
    }

    #[test]
    fn test_insert_entry_just_header() {
        let change_gen = ChangelogGenerator {};

        let log = CHANGELOG_HEADER.to_owned();
        let version = &Version::parse("0.1.2").unwrap();
        let new_entry = "## v0.1.2 (2020-10-05)\n\n- change 1\nchange 2".to_string();

        let res = change_gen
            .insert_entry(log, version, new_entry.to_string())
            .unwrap();
        let expected = format!("{}{}\n", CHANGELOG_HEADER, new_entry);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_markdown_changelog() {
        let change_gen = ChangelogGenerator {};
        let ver = &Version::parse("1.2.3").unwrap();

        let user = commit::User {
            name: "name".to_string(),
            email: "email".to_string(),
            date: "date".to_string(),
        };

        let non_empty: Vec<Commit> = vec![
            commit::Commit {
                commit: "commit2".to_string(),
                abbreviated_commit: "abb_commit2".to_string(),
                refs: "refs2".to_string(),
                commit_notes: "commit_notes".to_string(),
                subject: "second".to_string(),
                sanitized_subject_line: "sanitized-subject-line".to_string(),
                author: user.to_owned(),
                committer: user.to_owned(),
            },
            commit::Commit {
                commit: "commit1".to_string(),
                abbreviated_commit: "abb_commit1".to_string(),
                refs: "refs1".to_string(),
                commit_notes: "commit_notes".to_string(),
                subject: "first".to_string(),
                sanitized_subject_line: "sanitized-subject-line".to_string(),
                author: user.to_owned(),
                committer: user.to_owned(),
            },
        ];
        let non_empty_expected = "## v1.2.3 (2020-10-14)\n\n- second\n- first\n".to_string();
        assert_eq!(
            change_gen.markdown_changelog(&non_empty, Some(ver)),
            non_empty_expected
        );

        let empty: Vec<Commit> = vec![];
        let empty_expected = "## 2020-10-14\n\nNo commits since last version\n".to_string();
        assert_eq!(change_gen.markdown_changelog(&empty, None), empty_expected);
    }

    #[test]
    fn test_compact_changelog() {
        let change_gen = ChangelogGenerator {};

        let user = commit::User {
            name: "name".to_string(),
            email: "email".to_string(),
            date: "date".to_string(),
        };

        let non_empty: Vec<Commit> = vec![
            commit::Commit {
                commit: "commit2".to_string(),
                abbreviated_commit: "abb_commit2".to_string(),
                refs: "refs2".to_string(),
                commit_notes: "commit_notes".to_string(),
                subject: "second".to_string(),
                sanitized_subject_line: "sanitized-subject-line".to_string(),
                author: user.to_owned(),
                committer: user.to_owned(),
            },
            commit::Commit {
                commit: "commit1".to_string(),
                abbreviated_commit: "abb_commit1".to_string(),
                refs: "refs1".to_string(),
                commit_notes: "commit_notes".to_string(),
                subject: "first".to_string(),
                sanitized_subject_line: "sanitized-subject-line".to_string(),
                author: user.to_owned(),
                committer: user.to_owned(),
            },
        ];
        let non_empty_expected = " - second\n - first\n".to_string();
        assert_eq!(change_gen.compact_changelog(&non_empty), non_empty_expected);

        let empty: Vec<Commit> = vec![];
        let empty_expected = "No commits since last version".to_string();
        assert_eq!(change_gen.compact_changelog(&empty), empty_expected);
    }
}
