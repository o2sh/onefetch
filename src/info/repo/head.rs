use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
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

pub struct HeadInfo {
    pub head_refs: HeadRefs,
}

impl HeadInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let head_refs = repo.get_head_refs()?;
        Ok(Self { head_refs })
    }
}

impl InfoField for HeadInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Head,
            value: self.head_refs.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("HEAD")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_head_refs() {
        let head = HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]);
        assert_eq!(format!("{}", head), "be561d5 (main, origin/main)")
    }

    #[test]
    fn test_display_head_refs_with_no_refs() {
        let head = HeadRefs::new("be561d5".into(), vec![]);
        assert_eq!(format!("{}", head), "be561d5")
    }
}
