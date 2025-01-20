use crate::info::utils::info_field::InfoField;
use anyhow::{Context, Result};
use gix::Repository;
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
    let head_id = repo.head_id().context("Failed to retrieve HEAD ID")?;

    let mut ref_names = Vec::new();

    if let Some(head_ref) = repo.head_ref()? {
        let head_ref_name = head_ref.name().shorten().to_string();
        ref_names.push(head_ref_name);

        if let Some(Ok(remote_tracking_ref)) =
            repo.branch_remote_tracking_ref_name(head_ref.name(), gix::remote::Direction::Push)
        {
            let remote_tracking_ref_name = remote_tracking_ref.shorten().to_string();
            ref_names.push(remote_tracking_ref_name);
        }
    }

    Ok(HeadRefs::new(head_id.shorten()?.to_string(), ref_names))
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
