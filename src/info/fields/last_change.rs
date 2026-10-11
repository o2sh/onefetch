use crate::git::metrics::GitMetrics;
use crate::info::dates::serialize_time;
use gix::date::Time;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChange {
    #[serde(serialize_with = "serialize_time")]
    pub last_change: Time,
}

impl LastChange {
    pub fn new(git_metrics: &GitMetrics) -> Self {
        Self {
            last_change: git_metrics.time_of_most_recent_commit,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_last_change_info() {
        let last_change = LastChange {
            last_change: Time::new(946_771_200, 0),
        };

        assert_eq!(
            serde_json::to_value(&last_change).unwrap()["lastChange"],
            "2000-01-02T00:00:00Z"
        );
    }
}
