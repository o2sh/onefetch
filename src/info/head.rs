use crate::info::utils::{module::Module, text::Line};
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
impl Module for HeadInfo {
    fn value(&self) -> Vec<Line> {
        let HeadRefs {
            short_commit_id,
            refs,
        } = &self.head_refs;
        let head = if refs.is_empty() {
            short_commit_id.clone()
        } else {
            format!("{short_commit_id} ({})", refs.join(", "))
        };
        vec![Line::from(head)]
    }

    fn key(&self) -> String {
        "HEAD".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_head_refs() {
        let head_info = HeadInfo {
            head_refs: HeadRefs::new("be561d5".into(), vec!["main".into(), "origin/main".into()]),
        };
        assert_eq!(
            head_info.value(),
            vec![Line::from("be561d5 (main, origin/main)")]
        );
    }

    #[test]
    fn test_display_head_refs_with_no_refs() {
        let head_info = HeadInfo {
            head_refs: HeadRefs::new("be561d5".into(), vec![]),
        };
        assert_eq!(head_info.value(), vec![Line::from("be561d5")]);
    }
}
