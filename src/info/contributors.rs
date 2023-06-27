use super::git::metrics::GitMetrics;
use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsInfo {
    pub total_number_of_authors: usize,
    #[serde(skip_serializing)]
    pub number_of_authors_to_display: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl ContributorsInfo {
    pub fn new(
        git_metrics: &GitMetrics,
        number_of_authors_to_display: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        Self {
            total_number_of_authors: git_metrics.total_number_of_authors,
            number_of_authors_to_display,
            number_separator,
        }
    }
}

#[typetag::serialize]
impl InfoField for ContributorsInfo {
    fn value(&self) -> String {
        if self.total_number_of_authors > self.number_of_authors_to_display {
            format_number(&self.total_number_of_authors, self.number_separator)
        } else {
            "".to_string()
        }
    }

    fn title(&self) -> String {
        "Contributors".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gix::date::Time;

    #[test]
    fn test_display_contributors_info() {
        let timestamp = Time::now_utc();
        let git_metrics = GitMetrics {
            authors_to_display: vec![],
            file_churns_to_display: vec![],
            total_number_of_authors: 12,
            total_number_of_commits: 2,
            churn_pool_size: 0,
            time_of_most_recent_commit: timestamp,
            time_of_first_commit: timestamp,
        };

        let contributors_info = ContributorsInfo::new(&git_metrics, 2, NumberSeparator::Plain);
        assert_eq!(contributors_info.value(), "12".to_string());
        assert_eq!(contributors_info.title(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo {
            total_number_of_authors: 1,
            number_of_authors_to_display: 3,
            number_separator: NumberSeparator::Plain,
        };

        assert!(contributors_info.value().is_empty());
    }
}
