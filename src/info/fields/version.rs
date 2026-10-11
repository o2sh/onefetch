use crate::info::manifest::Manifest;
use anyhow::Result;
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub version: String,
}

impl Version {
    pub fn new(repo: &Repository, manifest: Option<&Manifest>) -> Result<Self> {
        let version = get_version(repo, manifest)?;
        Ok(Self { version })
    }
}

fn get_version(repo: &Repository, manifest: Option<&Manifest>) -> Result<String> {
    let mut version = String::new();
    let mut most_recent = 0;

    for tag in repo.references()?.tags()?.peeled()?.filter_map(Result::ok) {
        if let Ok(commit) = tag.id().object()?.try_into_commit() {
            let current_time = commit.time()?.seconds;
            if current_time > most_recent {
                most_recent = current_time;
                version = tag.name().shorten().to_string();
            }
        }
    }

    if version.is_empty() {
        let version_from_manifest = manifest.and_then(|m| m.version.clone()).unwrap_or_default();
        Ok(version_from_manifest)
    } else {
        Ok(version)
    }
}
