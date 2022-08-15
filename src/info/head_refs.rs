use serde::ser::SerializeStruct;
use serde::Serialize;

pub struct HeadRefs {
    short_commit_id: String,
    refs: Vec<String>,
}

impl HeadRefs {
    pub fn new(short_commit_id: String, refs: Vec<String>) -> HeadRefs {
        HeadRefs {
            short_commit_id,
            refs,
        }
    }
}

impl std::fmt::Display for HeadRefs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.refs.is_empty() {
            let refs_str = self
                .refs
                .iter()
                .map(|ref_name| ref_name.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            write!(f, "{} ({})", self.short_commit_id, refs_str)
        } else {
            write!(f, "{}", self.short_commit_id)
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
        state.serialize_field("oid", &self.short_commit_id)?;
        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_head_refs() {
        let head = HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]);
        assert_eq!(format!("{}", head), "be561d5 (main, origin/main)")
    }

    #[test]
    fn test_head_refs_with_no_refs() {
        let head = HeadRefs::new("be561d5".into(), vec![]);
        assert_eq!(format!("{}", head), "be561d5")
    }
}
