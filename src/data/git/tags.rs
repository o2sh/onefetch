use anyhow::Result;
use gix::{ObjectId, Repository};

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
