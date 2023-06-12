use super::git::metrics::GitMetrics;
use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitsInfo {
    pub number_of_commits: usize,
    is_shallow: bool,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl CommitsInfo {
    pub fn new(
        git_metrics: &GitMetrics,
        is_shallow: bool,
        number_separator: NumberSeparator,
    ) -> Self {
        Self {
            number_of_commits: git_metrics.total_number_of_commits,
            is_shallow,
            number_separator,
        }
    }
}

#[typetag::serialize]
impl InfoField for CommitsInfo {
    fn value(&self) -> String {
        format!(
            "{}{}",
            format_number(&self.number_of_commits, self.number_separator),
            self.is_shallow.then_some(" (shallow)").unwrap_or_default()
        )
    }

    fn title(&self) -> String {
        "Commits".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_commits_info() {
        let commits_info = CommitsInfo {
            number_of_commits: 3,
            is_shallow: false,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(commits_info.value(), "3".to_string());
    }

    #[test]
    fn test_display_commits_info_shallow() {
        let commits_info = CommitsInfo {
            number_of_commits: 2,
            is_shallow: true,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(commits_info.value(), "2 (shallow)".to_string());
    }
}
