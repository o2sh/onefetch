use askalono::{Store, TextData};

use crate::onefetch::error::*;

pub const LICENSE_FILES: [&str; 3] = ["LICENSE", "LICENCE", "COPYING"];

static CACHE_DATA: &[u8] = include_bytes!("../../resources/licenses/cache.bin.zstd");
const MIN_THRESHOLD: f32 = 0.8;

pub struct Detector {
    store: Store,
}

impl Detector {
    pub fn new() -> Result<Self> {
        Store::from_cache(CACHE_DATA)
            .map(|store| Self { store })
            .map_err(|_| "Could not initialize the license detector".into())
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
