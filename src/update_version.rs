use eyre::Result;
use semver::{Prerelease, Version};

#[derive(PartialEq, Debug)]
pub enum VersionType {
    Prerelease,
    Patch,
    Minor,
    Major,
}

pub fn map_version_type(version_type_str: &str) -> Result<VersionType> {
    match version_type_str.to_lowercase().as_ref() {
        "prerelease" => Ok(VersionType::Prerelease),
        "patch" => Ok(VersionType::Patch),
        "minor" => Ok(VersionType::Minor),
        "major" => Ok(VersionType::Major),
        _ => Err(eyre!("Invalid version type")),
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
            version.pre = Prerelease::new("0").unwrap();
        }
        VersionType::Patch => {
            debug!("Patch");
            if version.pre.is_empty() {
                version.patch += 1;
            }
            version.pre = Prerelease::EMPTY;
        }
        VersionType::Minor => {
            debug!("Minor");
            version.minor += 1;
            version.patch = 0;
            version.pre = Prerelease::EMPTY;
        }
        VersionType::Major => {
            debug!("Major");
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.pre = Prerelease::EMPTY;
        }
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_version_type_valid() {
        assert_eq!(
            map_version_type("Prerelease").unwrap(),
            VersionType::Prerelease
        );
        assert_eq!(map_version_type("patch").unwrap(), VersionType::Patch);
        assert_eq!(map_version_type("minoR").unwrap(), VersionType::Minor);
        assert_eq!(map_version_type("major").unwrap(), VersionType::Major);

        assert!(map_version_type("foo").is_err());
    }

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
