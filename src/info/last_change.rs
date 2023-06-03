use super::utils::format_time;
use crate::info::{utils::git::CommitMetrics, utils::info_field::InfoField};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastChangeInfo {
    pub last_change: String,
}

impl LastChangeInfo {
    pub fn new(iso_time: bool, commit_metrics: &CommitMetrics) -> Self {
        let last_change = get_date_of_last_commit(commit_metrics, iso_time);

        Self { last_change }
    }
}

fn get_date_of_last_commit(commit_metrics: &CommitMetrics, iso_time: bool) -> String {
    format_time(commit_metrics.time_of_most_recent_commit, iso_time)
}

#[typetag::serialize]
impl InfoField for LastChangeInfo {
    fn value(&self) -> String {
        self.last_change.to_string()
    }

    fn title(&self) -> String {
        "Last change".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_last_change_info() {
        let last_change_info = LastChangeInfo {
            last_change: "34 minutes ago".to_string(),
        };

        assert_eq!(last_change_info.value(), "34 minutes ago".to_string());
    }
}
