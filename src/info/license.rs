use anyhow::{bail, Result};
use askalono::{Store, TextData};
use std::{ffi::OsStr, fs};

const LICENSE_FILES: [&str; 3] = ["LICENSE", "LICENCE", "COPYING"];

static CACHE_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/licenses/cache.bin.zstd"));
const MIN_THRESHOLD: f32 = 0.8;

pub struct Detector {
    store: Store,
}

impl Detector {
    pub fn new() -> Result<Self> {
        match Store::from_cache(CACHE_DATA) {
            Ok(store) => Ok(Self { store }),
            Err(_) => bail!("Could not initialize the license detector"),
        }
    }

    pub fn get_license(&self, dir: &str) -> Result<String> {
        fn is_license_file<S: AsRef<str>>(file_name: S) -> bool {
            LICENSE_FILES.iter().any(|&name| file_name.as_ref().starts_with(name))
        }

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
        Ok(output.join(", "))
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
