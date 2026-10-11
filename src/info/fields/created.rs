use crate::info::dates::serialize_time;
use crate::info::git::metrics::GitMetrics;
use gix::date::Time;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Created {
    #[serde(serialize_with = "serialize_time")]
    pub creation_date: Time,
}

impl Created {
    pub fn new(git_metrics: &GitMetrics) -> Self {
        Self {
            creation_date: git_metrics.time_of_first_commit,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_created() {
        let created = Created {
            creation_date: Time::new(946_771_200, 0),
        };

        assert_eq!(
            serde_json::to_value(&created).unwrap()["creationDate"],
            "2000-01-02T00:00:00Z"
        );
    }
}
