use eyre::Result;
use hubcaps::{releases::ReleaseOptions, Credentials, Github};

const AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct GithubClient {
    client: Github,
}

impl GithubClient {
    pub fn new<T>(pat: T) -> Result<Self>
    where
        T: ToString,
    {
        let client = Github::new(AGENT, Credentials::Token(pat.to_string()))?;

        Ok(Self { client })
    }

    pub async fn create_new_release<T>(
        &self,
        project: T,
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
        let (owner, repo) = parse_project_string(project);

        client
            .repo(owner, repo)
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
