use crate::commit::Commit;
use eyre::Result;
use std::iter::FromIterator;
use std::process::{Command, Output};

/// Check if we're in an git repository.
pub fn in_git_repository() -> Result<bool> {
    git(&["rev-parse", "--is-inside-work-tree"]).map(|o| o.status.success())
}

// Abort if there are changes in the directory, maybe ask to abort or stash and un-stash after
// pub fn ensure_no_changes() {
//     debug!("ensuring no changes")
// }

/// Get the first commit
pub fn first_commit() -> Result<String> {
    let log_args = vec!["log", "--reverse", "--format=format:%H"];
    let commits: Vec<String> = git(&log_args).map(|o| read_lines(&o))?;

    match commits.first() {
        Some(first_commit) => Ok(first_commit.to_owned()),
        None => Err(eyre!("No commits have been made")),
    }
}

/// Get the last tag
pub fn last_tag() -> Result<Option<String>> {
    last_tags(1).map(|mut v| v.pop())
}

/// Returns a list of commits for the specified project
///
/// ## Example
///
/// ```rust
/// let range = "<commit a>..<commit b>".to_string();
/// let commits: Vec<String> = commits_in_log(&[range])?;
/// ```
pub fn commits_in_log(args: &[String]) -> Result<Vec<Commit>> {
    // I hope this can be improved a bit, its a bit messy and hard to understand
    // but it is just taking selected info using the git log formatter and creating
    // a json object that gets parsed into a Commit struct.
    let mut log_args = vec!["log", "--pretty=format:{ \"commit\": \"%H\", \"abbreviated_commit\": \"%h\", \"refs\": \"%D\", \"subject\": \"%s\", \"sanitized_subject_line\": \"%f\", \"commit_notes\": \"\", \"author\": { \"name\": \"%aN\", \"email\": \"%aE\", \"date\": \"%aD\" }, \"committer\": { \"name\": \"%cN\", \"email\": \"%cE\", \"date\": \"%cD\" }}END"];
    log_args.extend(args.iter().map(String::as_str));
    git(&log_args).map(|o| {
        let all_lines: String = read_lines(&o);
        // Split by END which is my end of commit indicator
        all_lines
            .split("END")
            .filter(|e| e.len() > 2)
            .map(Commit::from)
            .filter(|commit| {
                // filter out the following:
                // - `chore: releasing vX.Y.Z`
                // - `chore: beginning development on vX.Y.Z`
                // - `docs: updating changelog`

                let subj = commit.subject.to_string();
                if subj.starts_with("chore: releasing")
                    || subj.starts_with("chore: beginning development on")
                    || subj.starts_with("docs: updating changelog")
                {
                    return false;
                }
                true
            })
            .collect::<Vec<Commit>>()
    })
}

// https://stackoverflow.com/questions/18659959/git-tag-sorted-in-chronological-order-of-the-date-of-the-commit-pointed-to/57901182#comment75323431_36636526
/// Get the last n tags
fn last_tags(n: i32) -> Result<Vec<String>> {
    git(&[
        "for-each-ref",
        &format!("--count={}", n),
        // "--sort=-taggerdate",
        "--sort=-committerdate",
        "--format=%(refname:short)",
        "refs/tags/*",
    ])
    .map(|o| read_lines(&o))
}

/// Stages the specified files.
pub fn add_files(files: Vec<String>) -> Result<Output> {
    let mut add_args = vec!["add"];
    files.iter().for_each(|file| add_args.push(file.as_ref()));
    git(&add_args)
}

/// Commits staged files with the given message.
/// Errors if there are no staged files.
pub fn commit(commit_message: &str) -> Result<bool> {
    let args = vec!["commit", "-m", commit_message];

    if git(&args).is_err() {
        Err(eyre!("\nNo staged changes detected. Exiting."))
    } else {
        Ok(true)
    }
}

pub fn tag(new_tag: &str) -> Result<Output> {
    let args = vec!["tag", new_tag];
    git(&args)
}

/// Pushes everything committed to the main upstream
/// i.e. just `git push`
pub fn push(main_branch: &str) -> Result<Output> {
    let push_args = vec!["push", "origin", main_branch];
    git(&push_args)
}

/// Pushes the new tag to the main upstream
/// i.e. `git push origin TAG`
pub fn push_tag(tag_ver: &str) -> Result<Output> {
    let tag_args = vec!["push", "origin", tag_ver];
    git(&tag_args)
}

/// Returns the corresponding commit for a tag
pub fn get_commit_for_tag(tag_ver: &str) -> Result<String> {
    let args = vec!["rev-list", "-n 1", tag_ver];
    git(&args).map(|o| read_lines(&o))
}

/// Run a git command with arguments.
fn git(args: &[&str]) -> Result<Output> {
    debug!("git {}", args.join(" "));
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        // TODO(egilsster): Clean this logic up, maybe display output instead of erroring
        let stderr = format!("{}", String::from_utf8_lossy(&output.stderr));
        if !stderr.is_empty() {
            return Err(eyre!(stderr));
        }
        let stdout = format!("{}", String::from_utf8_lossy(&output.stdout));
        Err(eyre!(stdout))
    }
}

/// Parses the git output to a collection
fn read_lines<T: FromIterator<String>>(o: &Output) -> T {
    String::from_utf8_lossy(&o.stdout)
        .lines()
        .map(String::from)
        .collect::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_git_repository() {
        assert!(in_git_repository().unwrap());
    }

    #[test]
    fn test_first_commit() {
        assert!(first_commit().is_ok());
    }

    #[test]
    fn test_last_tag() {
        assert!(last_tag().is_ok());
    }

    #[test]
    #[ignore = "this test needs some work to pass on ci"]
    fn test_commits() {
        let range =
            "aa58f3dd441b7511fc0e7b9566732fc7e964f96c..6ebd873bfc3907ee0e40d5cb5c66bf17cc5c83fa"
                .to_string();
        let res = commits_in_log(&[range]).unwrap();

        assert_eq!(res[0].abbreviated_commit, "6ebd873");
        assert_eq!(res[0].author.name, "Egill Sveinbjörnsson");
    }

    #[test]
    fn test_add_files() {
        let files: Vec<String> = vec!["non-existing-file.tmp".to_owned()];
        assert!(add_files(files).is_err());

        let files: Vec<String> = vec!["README.md".to_owned()];
        assert!(add_files(files).is_ok());
    }

    #[test]
    #[ignore = "No tags yet / no mocking yet"]
    fn test_get_commit_for_tag() {
        let tag = "0.1.0";
        let res = get_commit_for_tag(tag).unwrap();

        assert_eq!(res, "139810fdw80".to_string());
    }
}
