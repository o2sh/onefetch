use crate::data::manifest::Manifest;
use anyhow::{Result, bail};
use askalono::{Store, TextData};
use serde::Serialize;
use std::path::Path;
use std::{ffi::OsStr, fs};

const LICENSE_FILES: [&str; 3] = ["LICENSE", "LICENCE", "COPYING"];

static CACHE_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/resources/license.cache.zstd"
));
const MIN_THRESHOLD: f32 = 0.8;

struct Detector {
    store: Store,
}

impl Detector {
    fn new() -> Result<Self> {
        match Store::from_cache(CACHE_DATA) {
            Ok(store) => Ok(Self { store }),
            Err(e) => {
                bail!("Could not initialize the license detector: {}", e)
            }
        }
    }

    fn detect(&self, dir: &Path, manifest: Option<&Manifest>) -> Result<String> {
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
                            .is_some_and(is_license_file)
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
pub struct License {
    pub license: String,
}

impl License {
    pub fn new(repo_path: &Path, manifest: Option<&Manifest>) -> Result<Self> {
        let license = Detector::new()?.detect(repo_path, manifest)?;
        Ok(Self { license })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::manifest::ManifestType;

    #[test]
    fn test_detect_license() -> Result<()> {
        let detector = Detector::new()?;
        let license = detector.detect(Path::new("."), None)?;
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
    fn should_read_from_manifest_first() -> Result<()> {
        let license = License::new(
            Path::new("."),
            Some(&Manifest {
                manifest_type: ManifestType::Cargo,
                name: None,
                description: None,
                number_of_dependencies: 0,
                version: None,
                license: Some("LICENSE".into()),
            }),
        )?;
        assert_eq!(license.license, "LICENSE");
        Ok(())
    }
}
