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

#[cfg(test)]
mod test {
    use super::*;
    use gix::{ThreadSafeRepository, open};

    fn fixture_repo(name: &str) -> Repository {
        let path = gix_testtools::scripted_fixture_read_only("make_tags_repo.sh")
            .unwrap()
            .join(name);
        ThreadSafeRepository::open_opts(path, open::Options::isolated())
            .unwrap()
            .to_thread_local()
    }

    #[test]
    fn test_no_tags() {
        let repo = fixture_repo("no_tags");
        assert!(get_latest_tag(&repo).unwrap().is_none());
    }

    #[test]
    fn test_tag_on_tree_is_skipped() {
        let repo = fixture_repo("tree_tag");
        assert!(get_latest_tag(&repo).unwrap().is_none());
    }

    #[test]
    fn test_tag_on_newest_commit_wins() {
        let repo = fixture_repo("tags");
        let latest = get_latest_tag(&repo).unwrap().unwrap();

        assert_eq!(latest.name, "v2.0.0");
        // The commit, not the annotated tag object.
        assert_eq!(latest.commit_id, repo.head_id().unwrap().detach());
    }
}
