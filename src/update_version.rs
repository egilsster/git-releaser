use eyre::Result;
use semver::Version;

pub enum VersionType {
    Prerelease,
    Patch,
    Minor,
    Major,
}

pub fn map_version_type(version_type_str: &str) -> VersionType {
    match version_type_str.to_lowercase().as_ref() {
        "prerelease" => VersionType::Prerelease,
        "patch" => VersionType::Patch,
        "minor" => VersionType::Minor,
        "major" => VersionType::Major,
        _ => panic!("Invalid version type"),
    }
}

/// Updates the version value based on the version type.
///
/// ## Example
///
/// ```
/// let res = update_version(Version::parse("0.1.2").unwrap(), VersionType::Prerelease).unwrap();
/// assert_eq!(res.to_string(), "0.1.3-0");
///
/// let res = update_version(Version::parse("0.1.2").unwrap(), VersionType::Patch).unwrap();
/// assert_eq!(res.to_string(), "0.1.3");
///
/// let res = update_version(Version::parse("0.1.2").unwrap(), VersionType::Minor).unwrap();
/// assert_eq!(res.to_string(), "0.2.0");
///
/// let res = update_version(Version::parse("0.1.2").unwrap(), VersionType::Major).unwrap();
/// assert_eq!(res.to_string(), "1.0.0");
/// ```
pub fn update_version(mut version: Version, version_type: VersionType) -> Result<Version> {
    match version_type {
        VersionType::Prerelease => {
            debug!("Prerelease");
            version.patch += 1;
            version.pre = vec![semver::Identifier::Numeric(0)];
        }
        VersionType::Patch => {
            debug!("Patch");
            if !version.is_prerelease() {
                version.patch += 1;
            }
            version.pre = vec![];
        }
        VersionType::Minor => {
            debug!("Minor");
            version.minor += 1;
            version.patch = 0;
            version.pre = vec![];
        }
        VersionType::Major => {
            debug!("Major");
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.pre = vec![];
        }
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_version(version: &str) -> Version {
        Version::parse(version).unwrap()
    }

    #[test]
    fn test_update_version_prerelease() {
        let res = update_version(to_version("0.1.2"), VersionType::Prerelease).unwrap();
        assert_eq!(res.to_string(), "0.1.3-0");
    }

    #[test]
    fn test_update_version_patch() {
        let res = update_version(to_version("0.1.2"), VersionType::Patch).unwrap();
        assert_eq!(res.to_string(), "0.1.3");
        let res = update_version(to_version("0.1.2-0"), VersionType::Patch).unwrap();
        assert_eq!(res.to_string(), "0.1.2");
    }

    #[test]
    fn test_update_version_minor() {
        let res = update_version(to_version("0.1.2"), VersionType::Minor).unwrap();
        assert_eq!(res.to_string(), "0.2.0");
        let res = update_version(to_version("0.1.2-0"), VersionType::Minor).unwrap();
        assert_eq!(res.to_string(), "0.2.0");
        let res = update_version(to_version("0.2.1-0"), VersionType::Minor).unwrap();
        assert_eq!(res.to_string(), "0.3.0");
    }

    #[test]
    fn test_update_version_major() {
        let res = update_version(to_version("0.1.2"), VersionType::Major).unwrap();
        assert_eq!(res.to_string(), "1.0.0");
        let res = update_version(to_version("0.1.0-0"), VersionType::Major).unwrap();
        assert_eq!(res.to_string(), "1.0.0");
        let res = update_version(to_version("1.0.1-0"), VersionType::Major).unwrap();
        assert_eq!(res.to_string(), "2.0.0");
    }
}
