use super::{git::metrics::GitMetrics, utils::format_time};
use crate::info::utils::info_field::InfoField;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedInfo {
    pub creation_date: String,
}

impl CreatedInfo {
    pub fn new(iso_time: bool, git_metrics: &GitMetrics) -> Self {
        let creation_date = get_creation_date(git_metrics, iso_time);
        Self { creation_date }
    }
}

fn get_creation_date(git_metrics: &GitMetrics, iso_time: bool) -> String {
    format_time(git_metrics.time_of_first_commit, iso_time)
}

#[typetag::serialize]
impl InfoField for CreatedInfo {
    fn value(&self) -> String {
        self.creation_date.to_string()
    }

    fn title(&self) -> String {
        "Created".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_created_info() {
        let created_info = CreatedInfo {
            creation_date: "2 years ago".to_string(),
        };

        assert_eq!(created_info.value(), "2 years ago".to_string());
    }
}
