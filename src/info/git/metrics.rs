use super::sig::Sig;
use anyhow::Result;
use gix::bstr::BString;
use gix::date::Time;
use std::collections::HashMap;

pub struct GitMetrics {
    pub number_of_commits_by_signature: HashMap<Sig, usize>,
    pub number_of_commits_by_file_path: HashMap<BString, usize>,
    pub total_number_of_authors: usize,
    pub total_number_of_commits: usize,
    pub churn_pool_size: usize,
    pub time_of_most_recent_commit: gix::date::Time,
    pub time_of_first_commit: gix::date::Time,
}

impl GitMetrics {
    pub fn new(
        number_of_commits_by_signature: HashMap<Sig, usize>,
        number_of_commits_by_file_path: HashMap<BString, usize>,
        churn_pool_size: usize,
        time_of_first_commit: Option<Time>,
        time_of_most_recent_commit: Option<Time>,
    ) -> Result<Self> {
        let total_number_of_commits = number_of_commits_by_signature.values().sum();
        let total_number_of_authors = number_of_commits_by_signature.len();

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        Ok(Self {
            number_of_commits_by_signature,
            number_of_commits_by_file_path,
            total_number_of_authors,
            total_number_of_commits,
            churn_pool_size,
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }
}
