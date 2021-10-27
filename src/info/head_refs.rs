use git2::Oid;
use serde::ser::SerializeStruct;
use serde::Serialize;

pub struct HeadRefs {
    commit: Oid,
    refs: Vec<String>,
}

impl HeadRefs {
    pub fn new(commit: Oid, refs: Vec<String>) -> HeadRefs {
        HeadRefs { commit, refs }
    }
}

impl std::fmt::Display for HeadRefs {
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

impl Serialize for HeadRefs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("HeadRefs", 2)?;
        state.serialize_field("refs", &self.refs)?;
        state.serialize_field(
            "oid",
            &self.commit.to_string().chars().take(7).collect::<String>(),
        )?;
        state.end()
    }
}
