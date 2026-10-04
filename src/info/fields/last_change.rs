use crate::info::format::Format;
use crate::info::format::serialize_time;
use crate::info::git::metrics::GitMetrics;
use crate::info::{info_field::InfoField, text::Line};
use gix::date::Time;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChangeInfo {
    #[serde(serialize_with = "serialize_time")]
    pub last_change: Time,
}

impl LastChangeInfo {
    pub fn new(git_metrics: &GitMetrics) -> Self {
        Self {
            last_change: git_metrics.time_of_most_recent_commit,
        }
    }
}

#[typetag::serialize]
impl InfoField for LastChangeInfo {
    fn value(&self, format: &Format) -> Vec<Line> {
        vec![Line::from(format.time(self.last_change))]
    }

    fn key(&self) -> String {
        "Last change".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_last_change_info() {
        let last_change_info = LastChangeInfo {
            last_change: Time::new(946_771_200, 0),
        };
        let iso_time = Format {
            iso_time: true,
            ..Format::default()
        };

        assert_eq!(
            last_change_info.value(&iso_time),
            vec![Line::from("2000-01-02T00:00:00Z")]
        );
    }

    #[test]
    fn test_serialize_last_change_info() {
        let last_change_info = LastChangeInfo {
            last_change: Time::new(946_771_200, 0),
        };

        assert_eq!(
            serde_json::to_value(&last_change_info).unwrap()["lastChange"],
            "2000-01-02T00:00:00Z"
        );
    }
}
