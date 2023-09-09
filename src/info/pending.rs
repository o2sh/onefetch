use crate::info::utils::info_field::InfoField;
use anyhow::Result;
use git2::{Status, StatusOptions, StatusShow};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingInfo {
    pub pending_changes: String,
}

impl PendingInfo {
    pub fn new(repo: &Repository) -> Result<Self> {
        let git_dir = repo.git_dir().to_owned();
        let repo = git2::Repository::open(git_dir)?;
        let pending_changes = get_pending_changes(&repo)?;
        Ok(Self { pending_changes })
    }
}

fn get_pending_changes(repo: &git2::Repository) -> Result<String> {
    let statuses = repo.statuses(Some(
        StatusOptions::default()
            .show(StatusShow::Workdir)
            .update_index(true)
            .include_untracked(true)
            .renames_head_to_index(true)
            .recurse_untracked_dirs(true),
    ))?;

    let (added, deleted, modified) =
        statuses
            .iter()
            .fold((0, 0, 0), |(added, deleted, modified), e| {
                let s: Status = e.status();
                if s.is_index_new() || s.is_wt_new() {
                    (added + 1, deleted, modified)
                } else if s.is_index_deleted() || s.is_wt_deleted() {
                    (added, deleted + 1, modified)
                } else {
                    (added, deleted, modified + 1)
                }
            });

    let mut result = String::new();
    if modified > 0 {
        result = format!("{modified}+-");
    }

    if added > 0 {
        result = format!("{result} {added}+");
    }

    if deleted > 0 {
        result = format!("{result} {deleted}-");
    }

    Ok(result.trim().into())
}

#[typetag::serialize]
impl InfoField for PendingInfo {
    fn value(&self) -> String {
        self.pending_changes.to_string()
    }

    fn title(&self) -> String {
        "Pending".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_pending_info() {
        let pending_info = PendingInfo {
            pending_changes: "4+-".to_string(),
        };

        assert_eq!(pending_info.value(), "4+-".to_string());
    }
}
