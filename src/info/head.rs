use crate::info::utils::info_field::InfoField;
use anyhow::{Context, Result};
use gix::{reference::Category, Reference, Repository};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadInfo {
    pub head_refs: HeadRefs,
}

impl HeadInfo {
    pub fn new(repo: &Repository) -> Result<Self> {
        let head_refs = get_head_refs(repo)?;
        Ok(Self { head_refs })
    }
}

fn get_head_refs(repo: &Repository) -> Result<HeadRefs> {
    let head_oid = repo.head_id().context("Could not read HEAD")?;
    let refs_info = repo
        .references()?
        .all()?
        .filter_map(Result::ok)
        .filter_map(|reference: Reference<'_>| {
            (reference.target().try_id() == Some(&head_oid)
                && reference.name().category() != Some(Category::Tag))
            .then(|| reference.name().shorten().to_string())
        })
        .collect();
    Ok(HeadRefs::new(head_oid.shorten()?.to_string(), refs_info))
}

#[typetag::serialize]
impl InfoField for HeadInfo {
    fn value(&self) -> String {
        self.head_refs.to_string()
    }

    fn title(&self) -> String {
        "HEAD".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_head_refs() {
        let head = HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]);
        assert_eq!(head.to_string(), "be561d5 (main, origin/main)");
    }

    #[test]
    fn test_display_head_refs_with_no_refs() {
        let head = HeadRefs::new("be561d5".into(), vec![]);
        assert_eq!(head.to_string(), "be561d5");
    }
}
