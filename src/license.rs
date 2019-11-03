use askalono::{Store, TextData};

static CACHE_DATA: &[u8] = include_bytes!("../resources/license-cache.bin.gz");

pub fn from_text(text: &str) -> Option<String> {
    match Store::from_cache(CACHE_DATA) {
        Ok(store) => match store.analyze(&TextData::from(text)) {
            Ok(license) => Some(license.name),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
