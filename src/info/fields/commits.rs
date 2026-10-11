use crate::git::metrics::GitMetrics;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Commits {
    pub number_of_commits: usize,
    pub is_shallow: bool,
}

impl Commits {
    pub fn new(git_metrics: &GitMetrics, is_shallow: bool) -> Self {
        Self {
            number_of_commits: git_metrics.total_number_of_commits,
            is_shallow,
        }
    }
}
