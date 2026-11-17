use super::identity::Identity;
use gix::bstr::BString;
use gix::date::Time;
use std::collections::HashMap;

pub struct GitMetrics {
    pub number_of_commits_by_identity: HashMap<Identity, usize>,
    pub number_of_commits_by_file_path: HashMap<BString, usize>,
    pub total_number_of_authors: usize,
    pub total_number_of_commits: usize,
    pub churn_pool_size: usize,
    /// The commit time of each counted commit (bots left out by `--no-bots`
    /// aren't), newest first, in seconds since the Unix epoch.
    pub commit_times: Vec<gix::date::SecondsSinceUnixEpoch>,
    pub time_of_most_recent_commit: gix::date::Time,
    pub time_of_first_commit: gix::date::Time,
}

impl GitMetrics {
    pub fn new(
        number_of_commits_by_identity: HashMap<Identity, usize>,
        total_number_of_commits: usize,
        number_of_commits_by_file_path: HashMap<BString, usize>,
        churn_pool_size: usize,
        commit_times: Vec<gix::date::SecondsSinceUnixEpoch>,
        time_of_first_commit: Option<Time>,
        time_of_most_recent_commit: Option<Time>,
    ) -> Self {
        let total_number_of_authors = number_of_commits_by_identity.len();

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .zip(time_of_most_recent_commit)
            .unwrap_or_default();

        Self {
            number_of_commits_by_identity,
            number_of_commits_by_file_path,
            total_number_of_authors,
            total_number_of_commits,
            churn_pool_size,
            commit_times,
            time_of_most_recent_commit,
            time_of_first_commit,
        }
    }
}
