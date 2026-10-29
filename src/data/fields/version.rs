use crate::data::manifest::Manifest;
use anyhow::Result;
use gix::{ObjectId, Repository};
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

/// The tag on the most recent commit, taken as the latest release.
pub struct LatestTag {
    pub name: String,
    pub commit_id: ObjectId,
}

pub fn get_latest_tag(repo: &Repository) -> Result<Option<LatestTag>> {
    let mut latest: Option<LatestTag> = None;
    let mut most_recent = 0;

    for tag in repo.references()?.tags()?.peeled()?.filter_map(Result::ok) {
        if let Ok(commit) = tag.id().object()?.try_into_commit() {
            let current_time = commit.time()?.seconds;
            if current_time > most_recent {
                most_recent = current_time;
                latest = Some(LatestTag {
                    name: tag.name().shorten().to_string(),
                    commit_id: commit.id,
                });
            }
        }
    }

    Ok(latest)
}
