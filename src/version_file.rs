use eyre::{Result, WrapErr};
use semver::Version;
use std::fs;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum VersionFiletype {
    TOML,
    JSON,
}

impl VersionFiletype {
    pub fn from_str(filename: &str) -> Result<Self> {
        let filename_lower = filename.to_lowercase();
        let file_ext = filename_lower.split('.').last().unwrap();
        match file_ext {
            "toml" => Ok(VersionFiletype::TOML),
            "json" => Ok(VersionFiletype::JSON),
            _ => Err(eyre!("Extension not supported")),
        }
    }
}

trait ToVersion {
    fn to_version(&self) -> Result<Version>;
}

impl ToVersion for str {
    fn to_version(&self) -> Result<Version> {
        Version::parse(self).wrap_err("Invalid version")
    }
}

pub struct VersionFile {
    pub filename: String,
    pub version_value: Version,
    pub version_filetype: VersionFiletype,
    pub lockfile: Option<String>,
}

impl VersionFile {
    pub fn new(filename: &str) -> Result<Self> {
        if !is_version_file_supported(&filename) {
            return Err(eyre!("The specified version file is not supported"));
        }

        let version_filetype = VersionFiletype::from_str(filename)?;
        let version_value = read_version_file(&version_filetype, filename)?;
        let lockfile = get_lockfile(&version_filetype);

        Ok(VersionFile {
            filename: filename.to_owned(),
            version_value,
            version_filetype,
            lockfile,
        })
    }

    /// Updates the version file with the new version value
    pub fn update_version_file(&mut self, new_ver: &Version) -> Result<()> {
        let ver_file = fs::read_to_string(&self.filename)?;

        match self.version_filetype {
            VersionFiletype::TOML => {
                let mut v: toml::Value = toml::from_str(&ver_file)?;
                v["package"]["version"] = toml::Value::String(new_ver.to_string());

                let version_file_contents = toml::to_string(&v)?;
                fs::write(&self.filename, version_file_contents)?;
                sync_cargo_lockfile()?;
            }
            VersionFiletype::JSON => {
                let mut v: serde_json::Value = serde_json::from_str(&ver_file)?;
                v["version"] = serde_json::to_value(&new_ver.to_string())?;

                let version_file_contents = format!("{}\n", serde_json::to_string_pretty(&v)?);
                fs::write(&self.filename, version_file_contents)?;
            }
        };

        self.version_value = new_ver.to_owned();

        Ok(())
    }

    pub fn get_version_value(&self) -> &Version {
        &self.version_value
    }

    pub fn get_tracked_files(&self) -> Vec<String> {
        let filename = self.filename.to_owned();
        match self.lockfile.to_owned() {
            Some(lockfile) => vec![filename, lockfile],
            None => vec![filename],
        }
    }
}

pub fn read_version_file(version_filetype: &VersionFiletype, file_path: &str) -> Result<Version> {
    match version_filetype {
        VersionFiletype::TOML => {
            let ver_file = fs::read_to_string(file_path)?;
            let v: toml::Value = toml::from_str(&ver_file)?;
            let package_info = v.get("package");

            if package_info.is_none() {
                return Err(eyre!("No version property found in TOML file"));
            }

            match package_info.unwrap().get("version") {
                Some(ver) => ver.as_str().unwrap().to_version(),
                None => Err(eyre!("Version property is not valid")),
            }
        }
        VersionFiletype::JSON => {
            let ver_file = fs::read_to_string(file_path)?;
            let v: serde_json::Value = serde_json::from_str(&ver_file)?;

            match v.get("version") {
                Some(ver) => ver.as_str().unwrap().to_version(),
                None => Err(eyre!("Version property is not valid")),
            }
        }
    }
}

/// Returns true if the version file is supported
/// and false otherwise.
fn is_version_file_supported(version_file: &str) -> bool {
    version_file.ends_with(".toml") || version_file.ends_with(".json")
}

pub fn get_lockfile(version_filetype: &VersionFiletype) -> Option<String> {
    match version_filetype {
        VersionFiletype::TOML => Some("Cargo.lock".to_string()),
        VersionFiletype::JSON => None,
    }
}

// I hate doing 'cargo check' here as its super slow when it has to fetch packages.
// There is some discussions going on here on this:
// https://internals.rust-lang.org/t/pre-rfc-cargo-command-to-just-sync-lockfile/13119
pub fn sync_cargo_lockfile() -> Result<bool> {
    debug!("Sync Cargo.lock");
    let output = Command::new("cargo").args(&["check"]).output()?;
    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_filetype_from_str() {
        assert_eq!(
            VersionFiletype::from_str("package.json").unwrap(),
            VersionFiletype::JSON
        );
        assert_eq!(
            VersionFiletype::from_str("version.json").unwrap(),
            VersionFiletype::JSON
        );
        assert_eq!(
            VersionFiletype::from_str("Cargo.toml").unwrap(),
            VersionFiletype::TOML
        );
        assert_eq!(
            VersionFiletype::from_str("version.toml").unwrap(),
            VersionFiletype::TOML
        );
        assert!(VersionFiletype::from_str("version.txt").is_err(),);
    }

    #[test]
    fn test_new_version_file_invalid() {
        assert!(VersionFile::new("version.txt").is_err());
    }

    #[test]
    #[ignore = "Syncing the cargo lock isn't working on ci"]
    fn test_update_version_file_cargo_toml() {
        let test_file = "Cargo_test.toml";
        let contents = r#"[package]
name = "git-releaser"
version = "0.1.0"
authors = ["Author <author@earth.com>"]
edition = "2018"
"#
        .to_owned();

        fs::write(test_file, contents).unwrap();

        let mut v = VersionFile::new(test_file).unwrap();

        v.update_version_file(&Version::parse("1.0.0").unwrap())
            .unwrap();

        let updated_contents = fs::read_to_string(&test_file).unwrap();
        fs::remove_file(&test_file).unwrap();

        assert_eq!(v.get_version_value().to_string(), "1.0.0");
        assert_eq!(
            updated_contents,
            r#"[package]
name = "git-releaser"
version = "1.0.0"
authors = ["Author <author@earth.com>"]
edition = "2018"
"#
        );
    }

    #[test]
    fn test_update_version_file_package_json() {
        let test_file = "test.json";
        let contents = r#"
            {
                "name": "testing",
                "version": "0.2.5",
                "author": "me"
            }
        "#
        .to_owned();

        fs::write(test_file, contents).unwrap();

        let mut v = VersionFile::new(test_file).unwrap();

        v.update_version_file(&Version::parse("0.2.6").unwrap())
            .unwrap();

        let updated_contents = fs::read_to_string(&test_file).unwrap();
        fs::remove_file(&test_file).unwrap();

        assert_eq!(v.get_version_value().to_string(), "0.2.6");
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

    #[test]
    fn test_read_version_file_package_json_invalid() {
        let test_file = "invalid.json";
        let contents = r#"
            {
                "name": "testing",
                "author": "me"
            }
        "#
        .to_owned();

        fs::write(test_file, contents).unwrap();

        let v = read_version_file(&VersionFiletype::JSON, test_file);
        assert!(v.is_err());
        fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_is_version_file_supported() {
        assert!(is_version_file_supported("Cargo.toml") == true);
        assert!(is_version_file_supported("Cargo_test.toml") == true);
        assert!(is_version_file_supported("package.json") == true);
        assert!(is_version_file_supported("version.json") == true);
        assert!(is_version_file_supported("version.txt") == false);
        assert!(is_version_file_supported("foo") == false);
    }

    #[test]
    fn test_str_to_version() {
        let ver = "0.1.2";
        let res = ver.to_version().unwrap();
        assert_eq!(res.to_string(), ver);

        assert!("foo".to_version().is_err());
        assert!("1.0.0.0".to_version().is_err());
    }

    #[test]
    #[ignore = "Need to mock the cargo check command"]
    fn test_get_lockfile_toml() {
        let ver_file = VersionFile {
            filename: "Cargo.toml".to_string(),
            version_value: "0.1.2".to_version().unwrap(),
            version_filetype: VersionFiletype::TOML,
            lockfile: Some("Cargo.lock".to_string()),
        };

        assert_eq!(
            ver_file.get_tracked_files(),
            vec!["Cargo.toml", "Cargo.lock"]
        );
    }

    #[test]
    fn test_get_lockfile_json() {
        let ver_file = VersionFile {
            filename: "package.json".to_string(),
            version_value: "0.1.2".to_version().unwrap(),
            version_filetype: VersionFiletype::TOML,
            lockfile: None,
        };

        assert_eq!(ver_file.get_tracked_files(), vec!["package.json"]);
    }
}
