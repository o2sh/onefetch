use askalono::{Store, TextData};

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

static CACHE_DATA: &[u8] = include_bytes!("../resources/licenses/cache.bin.zstd");
const MIN_THRESHOLD: f32 = 0.8;

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
        let matched = self.store.analyze(&TextData::from(text));

        if matched.score >= MIN_THRESHOLD {
            Some(matched.name.into())
        } else {
            None
        }
    }
}
