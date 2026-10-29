use crate::data::git::LatestTag;
use anyhow::Result;
use gix::Repository;
use serde::Serialize;

/// The commits since the latest release: the tag the Version field shows.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unreleased {
    pub tag: String,
    pub commits_since_tag: usize,
}

impl Unreleased {
    pub fn new(repo: &Repository, latest_tag: &LatestTag) -> Result<Self> {
        let head_id = repo.head_id()?.detach();

        // Same as `git rev-list --count <tag>..HEAD`.
        let commits_since_tag = repo
            .rev_walk([head_id])
            .with_hidden([latest_tag.commit_id])
            .all()?
            .try_fold(0, |count, info| info.map(|_| count + 1))?;

        Ok(Self {
            tag: latest_tag.name.clone(),
            commits_since_tag,
        })
    }
}
