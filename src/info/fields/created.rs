use crate::git::metrics::GitMetrics;
use crate::info::dates::serialize_time;
use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use gix::date::Time;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedInfo {
    #[serde(serialize_with = "serialize_time")]
    pub creation_date: Time,
}

impl CreatedInfo {
    pub fn new(git_metrics: &GitMetrics) -> Self {
        Self {
            creation_date: git_metrics.time_of_first_commit,
        }
    }
}

#[typetag::serialize]
impl InfoField for CreatedInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(options.time(self.creation_date))]
    }

    fn key(&self) -> String {
        "Created".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_created_info() {
        let created_info = CreatedInfo {
            creation_date: Time::new(946_771_200, 0),
        };
        let iso_time = DisplayOptions {
            iso_time: true,
            ..DisplayOptions::default()
        };

        assert_eq!(
            created_info.value(&iso_time),
            vec![Line::from("2000-01-02T00:00:00Z")]
        );
    }

    #[test]
    fn test_serialize_created_info() {
        let created_info = CreatedInfo {
            creation_date: Time::new(946_771_200, 0),
        };

        assert_eq!(
            serde_json::to_value(&created_info).unwrap()["creationDate"],
            "2000-01-02T00:00:00Z"
        );
    }
}
