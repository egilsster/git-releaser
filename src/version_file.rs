use eyre::{Result, WrapErr};
use semver::Version;
use std::fs;

pub enum VersionFiletype {
    JSON,
}

impl VersionFiletype {
    pub fn from_str(filename: &str) -> Result<Self> {
        let filename_lower = filename.to_lowercase();
        let file_ext = filename_lower.split('.').last().unwrap();
        match file_ext {
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
}

impl VersionFile {
    pub fn new(filename: String) -> Result<Self> {
        if !is_version_file_supported(&filename) {
            return Err(eyre!("The specified version file is not supported"));
        }

        let version_filetype = VersionFiletype::from_str(&filename)?;
        let version_value = read_version_file(&version_filetype, &filename)?;

        Ok(VersionFile {
            filename,
            version_value,
            version_filetype,
        })
    }

    /// Updates the version file with the new version value
    pub fn update_version_file(&mut self, new_ver: &Version) -> Result<()> {
        let ver_file = fs::read_to_string(&self.filename)?;

        match self.version_filetype {
            VersionFiletype::JSON => {
                let mut v: serde_json::Value = serde_json::from_str(&ver_file)?;
                v["version"] = serde_json::to_value(&new_ver.to_string())?;

                let version_file_contents = format!("{}\n", serde_json::to_string_pretty(&v)?);
                fs::write(&self.filename, version_file_contents)?;
            }
        };

        debug!("📝 New version is {}", new_ver);

        self.version_value = new_ver.to_owned();

        Ok(())
    }

    pub fn get_version_value(&self) -> &Version {
        &self.version_value
    }
}

pub fn read_version_file(version_filetype: &VersionFiletype, file_path: &str) -> Result<Version> {
    match version_filetype {
        VersionFiletype::JSON => {
            let ver_file =
                fs::read_to_string(file_path).wrap_err("Could not read JSON version file")?;
            let v: serde_json::Value =
                serde_json::from_str(&ver_file).wrap_err("Could not parse JSON file")?;

            match v.get("version") {
                Some(ver) => ver.as_str().unwrap().to_version(),
                None => panic!("Version property is not valid"),
            }
        }
    }
}

/// Returns true if the version file is supported
/// and false otherwise.
fn is_version_file_supported(version_file: &str) -> bool {
    version_file.ends_with(".toml") || version_file.ends_with(".json")
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut v = VersionFile::new(test_file.to_owned()).unwrap();

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
    }
}
