use semver::Version;
use serde_json::Value;
use std::fs;

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
/// let res = update_version("0.1.2", VersionType::Prerelease);
/// assert_eq!(res, "0.1.3-0");
///
/// let res = update_version("0.1.2", VersionType::Patch);
/// assert_eq!(res, "0.1.3");
///
/// let res = update_version("0.1.2", VersionType::Minor);
/// assert_eq!(res, "0.2.0");
///
/// let res = update_version("0.1.2", VersionType::Major);
/// assert_eq!(res, "1.0.0");
/// ```
pub fn update_version(version: &str, version_type: VersionType) -> String {
    let mut parsed = Version::parse(version).unwrap();

    match version_type {
        VersionType::Prerelease => {
            println!("Prerelease");
            parsed.patch += 1;
            parsed.pre = vec![semver::Identifier::Numeric(0)];
        }
        VersionType::Patch => {
            println!("Patch");
            parsed.patch += 1;
            parsed.pre = vec![];
        }
        VersionType::Minor => {
            println!("Minor");
            parsed.minor += 1;
            parsed.patch = 0;
            parsed.pre = vec![];
        }
        VersionType::Major => {
            println!("Major");
            parsed.major += 1;
            parsed.minor = 0;
            parsed.patch = 0;
            parsed.pre = vec![];
        }
    }
    return parsed.to_string();
}

/// Updates the version file with the new version value
pub fn update_version_file(file_path: &str, new_ver: &str) -> String {
    // TODO: Validation and error handling
    let ver_file = fs::read_to_string(file_path).unwrap();
    let mut v: Value = serde_json::from_str(&ver_file).unwrap();

    v["version"] = serde_json::to_value(&new_ver).unwrap();

    println!("📝 New version is {}", new_ver);
    let _res = fs::write(
        file_path,
        format!("{}\n", serde_json::to_string_pretty(&v).unwrap()),
    );

    return format!("v{}", new_ver);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_version_prerelease() {
        let res = update_version("0.1.2", VersionType::Prerelease);
        assert_eq!(res, "0.1.3-0");
    }

    #[test]
    fn test_update_version_patch() {
        let res = update_version("0.1.2", VersionType::Patch);
        assert_eq!(res, "0.1.3");
    }

    #[test]
    fn test_update_version_minor() {
        let res = update_version("0.1.2", VersionType::Minor);
        assert_eq!(res, "0.2.0");
    }

    #[test]
    fn test_update_version_major() {
        let res = update_version("0.1.2", VersionType::Major);
        assert_eq!(res, "1.0.0");
    }

    #[test]
    fn test_prerelease_version_and_preserve_structure() {
        let test_file = "test.json";
        let contents = r#"
            {
                "name": "testing",
                "version": "0.2.5",
                "author": "me"
            }
        "#
        .to_owned();
        let _ = fs::write(test_file, contents).unwrap();

        let res = update_version_file(test_file, "0.2.6");
        assert_eq!(res, "v0.2.6");

        let updated_contents = fs::read_to_string(test_file).unwrap();
        fs::remove_file(test_file).unwrap();

        assert_eq!(
            updated_contents,
            r#"{
  "name": "testing",
  "version": "0.2.6",
  "author": "me"
}
"#
        );
    }
}