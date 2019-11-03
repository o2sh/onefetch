use askalono::{Store, TextData};

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

static CACHE_DATA: &[u8] = include_bytes!("../resources/license-cache.bin.gz");

pub struct Detector {
    store: Store,
}

impl Detector {
    pub fn new() -> Result<Self> {
        Store::from_cache(CACHE_DATA)
            .map(|store| Self { store })
            .map_err(|_| Error::LicenseDetectorError)
    }

    pub fn analyze(&self, text: &str) -> Option<String> {
        self.store
            .analyze(&TextData::from(text))
            .ok()
            .map(|license| license.name)
    }
}
