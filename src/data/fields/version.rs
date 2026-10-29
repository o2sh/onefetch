use crate::data::git::LatestTag;
use crate::data::manifest::Manifest;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub version: String,
}

impl Version {
    pub fn new(latest_tag: Option<&LatestTag>, manifest: Option<&Manifest>) -> Self {
        let version = match latest_tag {
            Some(tag) => tag.name.clone(),
            None => manifest.and_then(|m| m.version.clone()).unwrap_or_default(),
        };
        Self { version }
    }
}
