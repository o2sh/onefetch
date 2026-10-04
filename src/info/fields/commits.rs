use crate::info::git::metrics::GitMetrics;
use crate::info::info_field::InfoField;
use crate::info::text::{Line, Span};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitsInfo {
    pub number_of_commits: usize,
    is_shallow: bool,
}

impl CommitsInfo {
    pub fn new(git_metrics: &GitMetrics, is_shallow: bool) -> Self {
        Self {
            number_of_commits: git_metrics.total_number_of_commits,
            is_shallow,
        }
    }
}

#[typetag::serialize]
impl InfoField for CommitsInfo {
    fn value(&self) -> Vec<Line> {
        let mut spans = vec![Span::number(self.number_of_commits as u64)];
        if self.is_shallow {
            spans.push(Span::value(" (shallow)"));
        }
        vec![spans.into()]
    }

    fn key(&self) -> String {
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
        };

        assert_eq!(
            commits_info.value(),
            vec![Line::from(vec![Span::number(3)])]
        );
    }

    #[test]
    fn test_display_commits_info_shallow() {
        let commits_info = CommitsInfo {
            number_of_commits: 2,
            is_shallow: true,
        };

        assert_eq!(
            commits_info.value(),
            vec![Line::from(vec![Span::number(2), Span::value(" (shallow)")])]
        );
    }
}
