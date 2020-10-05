use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Commit {
    pub commit: String,
    pub abbreviated_commit: String,
    pub refs: String,
    pub subject: String,
    pub sanitized_subject_line: String,
    pub commit_notes: String,
    pub author: User,
    pub committer: User,
}

impl Commit {
    pub fn compact(&self) -> String {
        // format!("{} [{}]", self.subject, self.author.name)
        self.subject.to_string()
    }
}

impl<T: AsRef<str>> From<T> for Commit {
    /// Construct a commit from a json structure
    fn from(input: T) -> Self {
        match serde_json::from_str(input.as_ref()) {
            Ok(commit) => commit,
            Err(err) => {
                println!("{:?}", err.to_string());
                Commit::default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let data = r#"
        {
            "commit": "d41902f4ac0efbfbabcf25742d959c320e685cf2",
            "abbreviated_commit": "d41902f",
            "refs": "HEAD -> main, origin/main, origin/HEAD",
            "subject": "ci: add unit test, format, clippy, audit actions (#2)",
            "sanitized_subject_line": "ci-add-unit-test-format-clippy-audit-actions-2",
            "commit_notes": "",
            "author": {
                "name": "Egill Sveinbjörnsson",
                "email": "egilsster@users.noreply.github.com",
                "date": "Thu, 1 Oct 2020 12:53:15 +0200"
            },
            "committer": {
                "name": "GitHub",
                "email": "noreply@github.com",
                "date": "Thu, 1 Oct 2020 12:53:15 +0200"
            }
        }"#;

        let commit = Commit::from(data);

        assert_eq!(commit.author.name, "Egill Sveinbjörnsson");
    }

    #[test]
    fn test_compact_form() {
        let data = r#"
        {
            "commit": "d41902f4ac0efbfbabcf25742d959c320e685cf2",
            "abbreviated_commit": "d41902f",
            "refs": "HEAD -> main, origin/main, origin/HEAD",
            "subject": "ci: add unit test, format, clippy, audit actions (#2)",
            "sanitized_subject_line": "ci-add-unit-test-format-clippy-audit-actions-2",
            "commit_notes": "",
            "author": {
                "name": "Egill Sveinbjörnsson",
                "email": "egilsster@users.noreply.github.com",
                "date": "Thu, 1 Oct 2020 12:53:15 +0200"
            },
            "committer": {
                "name": "GitHub",
                "email": "noreply@github.com",
                "date": "Thu, 1 Oct 2020 12:53:15 +0200"
            }
        }"#;

        let commit = Commit::from(data);

        assert_eq!(
            commit.compact(),
            "ci: add unit test, format, clippy, audit actions (#2)"
        );
    }
}
