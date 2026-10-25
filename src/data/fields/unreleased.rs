use super::version::LatestTag;
use crate::data::dates::serialize_time;
use anyhow::Result;
use gix::Repository;
use gix::date::Time;
use serde::Serialize;

/// The commits since the latest release: the tag the Version field shows.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unreleased {
    pub tag: String,
    pub commits_since_tag: usize,
    /// The commit time of the tagged commit.
    #[serde(serialize_with = "serialize_time")]
    pub tag_date: Time,
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
            tag_date: latest_tag.commit_time,
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
            tag_date: Time::new(946_771_200, 0),
        };

        assert_eq!(
            serde_json::to_value(&unreleased).unwrap(),
            serde_json::json!({
                "tag": "v1.0.0",
                "commitsSinceTag": 3,
                "tagDate": "2000-01-02T00:00:00Z",
            })
        );
    }
}
