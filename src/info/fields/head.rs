use anyhow::{Context, Result};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadRefs {
    pub short_commit_id: String,
    pub refs: Vec<String>,
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
pub struct Head {
    pub head_refs: HeadRefs,
}

impl Head {
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
