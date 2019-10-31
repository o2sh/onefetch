use git2::Oid;

pub struct CommitInfo {
    commit: Oid,
    refs: Vec<String>,
}

impl CommitInfo {
    pub fn new(commit: Oid, refs: Vec<String>) -> CommitInfo {
        CommitInfo { commit, refs }
    }
}

impl std::fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let short_commit = self.commit.to_string().chars().take(7).collect::<String>();
        if !self.refs.is_empty() {
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
