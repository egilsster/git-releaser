use eyre::Result;
use hubcaps::{releases::ReleaseOptions, Credentials, Github};

const AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct GithubClient {
    client: Github,
    owner: String,
    repo: String,
}

impl GithubClient {
    pub fn new<T>(project: T, pat: T) -> Result<Self>
    where
        T: ToString,
    {
        let client = Github::new(AGENT, Credentials::Token(pat.to_string()))?;
        let (owner, repo) = parse_project_string(project);

        Ok(Self {
            client,
            owner,
            repo,
        })
    }

    pub async fn create_new_release<T>(
        &self,
        tag: T,
        tagged_commitish: T,
        changelog: T,
    ) -> Result<bool>
    where
        T: ToString,
    {
        let client = self.client.to_owned();
        let release_opts = ReleaseOptions {
            tag_name: tag.to_string(),
            target_commitish: Some(tagged_commitish.to_string()),
            name: Some(tag.to_string()),
            body: Some(changelog.to_string()),
            draft: Some(false),
            prerelease: Some(false),
        };

        client
            .repo(&self.owner, &self.repo)
            .releases()
            .create(&release_opts)
            .await?;

        Ok(true)
    }
}

/// Creates a tuple from the '<org>/<repo>' value
fn parse_project_string<T>(project: T) -> (String, String)
where
    T: ToString,
{
    let p = project.to_string();
    let split = p.split('/').collect::<Vec<_>>();

    (split[0].to_string(), split[1].to_string())
}
