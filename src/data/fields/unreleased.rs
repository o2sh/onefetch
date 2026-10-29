use super::version::LatestTag;
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
    /// `None` when HEAD isn't built on the latest tag.
    pub fn new(repo: &Repository, latest_tag: &LatestTag) -> Result<Option<Self>> {
        let head_id = repo.head_id()?.detach();
        let merge_base = repo.merge_base(head_id, latest_tag.commit_id)?;
        if merge_base.map(|id| id.detach()) != Some(latest_tag.commit_id) {
            return Ok(None);
        }

        // Same as `git rev-list --count <tag>..HEAD`.
        let commits_since_tag = repo
            .rev_walk([head_id])
            .with_hidden([latest_tag.commit_id])
            .all()?
            .try_fold(0, |count, info| info.map(|_| count + 1))?;

        Ok(Some(Self {
            tag: latest_tag.name.clone(),
            commits_since_tag,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_unreleased() {
        let unreleased = Unreleased {
            tag: "v1.0.0".into(),
            commits_since_tag: 3,
        };

        assert_eq!(
            serde_json::to_value(&unreleased).unwrap(),
            serde_json::json!({
                "tag": "v1.0.0",
                "commitsSinceTag": 3,
            })
        );
    }
}
