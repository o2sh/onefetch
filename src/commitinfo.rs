extern crate git2;

use git2::Oid;
use std::fmt;

pub struct CommitInfo {
    pub commit: Oid,
    pub refs: Vec<String>,
}

impl CommitInfo {
    pub fn new(commit: Oid, refs: Vec<String>) -> CommitInfo {
        CommitInfo { commit, refs }
    }
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_commit = self.commit.to_string().chars().take(7).collect::<String>();
        if self.refs.len() > 0 {
            let refs_str = self
                .refs
                .iter()
                .map(|ref_name| ref_name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            write!(f, "{} ({})", short_commit, refs_str)
        } else {
            write!(f, "{}", short_commit)
        }
    }
}
