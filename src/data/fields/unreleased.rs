use crate::data::dates::serialize_time;
use anyhow::Result;
use gix::Repository;
use gix::commit::describe::SelectRef;
use gix::date::Time;
use serde::Serialize;

/// The commits since the latest release: the closest tag that HEAD descends
/// from, as `git describe --tags` finds it.
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
    /// `None` when HEAD doesn't descend from any tag.
    pub fn new(repo: &Repository) -> Result<Option<Self>> {
        let head = repo.head_commit()?;
        let Some(resolution) = head.describe().names(SelectRef::AllTags).try_resolve()? else {
            return Ok(None);
        };
        let outcome = resolution.outcome;
        let Some(tag) = outcome.name else {
            return Ok(None);
        };
        let Some(tagged_commit) = outcome
            .name_by_oid
            .iter()
            .find_map(|(id, name)| (*name == tag).then_some(*id))
        else {
            return Ok(None);
        };
        let tag_date = repo.find_commit(tagged_commit)?.time()?;

        Ok(Some(Self {
            tag: tag.to_string(),
            commits_since_tag: outcome.depth as usize,
            tag_date,
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
