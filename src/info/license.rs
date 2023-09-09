use crate::info::utils::info_field::InfoField;
use anyhow::{bail, Result};
use askalono::{Store, TextData};
use onefetch_manifest::Manifest;
use serde::Serialize;
use std::path::Path;
use std::{ffi::OsStr, fs};

const LICENSE_FILES: [&str; 3] = ["LICENSE", "LICENCE", "COPYING"];

static CACHE_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/license.cache.zstd"
));
const MIN_THRESHOLD: f32 = 0.8;

pub struct Detector {
    store: Store,
}

impl Detector {
    pub fn new() -> Result<Self> {
        match Store::from_cache(CACHE_DATA) {
            Ok(store) => Ok(Self { store }),
            Err(_) => {
                bail!("Could not initialize the license detector")
            }
        }
    }

    fn get_license(&self, dir: &Path, manifest: Option<&Manifest>) -> Result<String> {
        let license_from_manifest = manifest.and_then(|m| m.license.clone()).unwrap_or_default();
        if license_from_manifest.is_empty() {
            let mut output = fs::read_dir(dir)?
                .filter_map(std::result::Result::ok)
                .map(|entry| entry.path())
                .filter(|entry| {
                    entry.is_file()
                        && entry
                            .file_name()
                            .map(OsStr::to_string_lossy)
                            .map(is_license_file)
                            .unwrap_or_default()
                })
                .filter_map(|entry| {
                    let contents = fs::read_to_string(entry).unwrap_or_default();
                    self.analyze(&contents)
                })
                .collect::<Vec<_>>();

            output.sort();
            output.dedup();
            let license = output.join(", ");
            Ok(license)
        } else {
            Ok(license_from_manifest)
        }
    }

    fn analyze(&self, text: &str) -> Option<String> {
        let matched = self.store.analyze(&TextData::from(text));

        if matched.score >= MIN_THRESHOLD {
            Some(matched.name.into())
        } else {
            None
        }
    }
}

fn is_license_file<S: AsRef<str>>(file_name: S) -> bool {
    LICENSE_FILES
        .iter()
        .any(|&name| file_name.as_ref().starts_with(name))
}

#[derive(Serialize)]
pub struct LicenseInfo {
    pub license: String,
}

impl LicenseInfo {
    pub fn new(repo_path: &Path, manifest: Option<&Manifest>) -> Result<Self> {
        let license = Detector::new()?.get_license(repo_path, manifest)?;
        Ok(Self { license })
    }
}

#[typetag::serialize]
impl InfoField for LicenseInfo {
    fn value(&self) -> String {
        self.license.to_string()
    }

    fn title(&self) -> String {
        "License".into()
    }
}

#[cfg(test)]
mod test {
    use onefetch_manifest::ManifestType;

    use super::*;

    #[test]
    fn test_get_license() -> Result<()> {
        let detector = Detector::new()?;
        let license = detector.get_license(Path::new("."), None)?;
        assert_eq!(license, "MIT");
        Ok(())
    }

    #[test]
    fn test_is_license_file() {
        for file_name in &LICENSE_FILES {
            assert!(is_license_file(file_name));
        }
        assert!(!is_license_file("NOT_LICENSE"));
    }

    #[test]
    fn test_analyze() -> Result<()> {
        let detector = Detector::new()?;
        let license_text = fs::read_to_string(Path::new("LICENSE.md"))?;
        let license = detector.analyze(&license_text);
        assert_eq!(license, Some("MIT".into()));
        Ok(())
    }

    #[test]
    fn should_read_from_manifest_first() -> Result<()> {
        let license_info = LicenseInfo::new(
            Path::new("."),
            Some(&Manifest {
                manifest_type: ManifestType::Cargo,
                name: String::new(),
                description: None,
                number_of_dependencies: 0,
                version: String::new(),
                license: Some("LICENSE".into()),
            }),
        )?;
        assert_eq!(license_info.value(), "LICENSE");
        Ok(())
    }
}
