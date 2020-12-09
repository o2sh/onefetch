use git2::Oid;
use serde::ser::SerializeStruct;
use serde::Serialize;

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

impl Serialize for CommitInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CommitInfo", 2)?;
        state.serialize_field("refs", &self.refs)?;
        state
            .serialize_field("oid", &self.commit.to_string().chars().take(7).collect::<String>())?;
        state.end()
    }
}
