use crate::info::utils::info_field::InfoField;
use anyhow::Result;
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingInfo {
    pub pending_changes: String,
}

impl PendingInfo {
    pub fn new(repo: &Repository) -> Result<Self> {
        let pending_changes = get_pending_changes(repo)?;
        Ok(Self { pending_changes })
    }
}

fn get_pending_changes(repo: &Repository) -> Result<String> {
    let statuses = repo
        .status(gix::progress::Discard)?
        .dirwalk_options(|options| options.emit_untracked(gix::dir::walk::EmissionMode::Matching))
        .into_index_worktree_iter(Vec::new())?;

    let (added, deleted, modified) = statuses
        .take_while(Result::is_ok)
        .filter_map(Result::ok)
        .filter_map(|item| item.summary())
        .fold((0, 0, 0), |(added, deleted, modified), status| {
            use gix::status::index_worktree::iter::Summary;
            match status {
                Summary::Removed => (added, deleted + 1, modified),
                Summary::Added | Summary::Copied => (added + 1, deleted, modified),
                Summary::Modified | Summary::TypeChange => (added, deleted, modified + 1),
                Summary::Renamed => (added + 1, deleted + 1, modified),
                Summary::IntentToAdd | Summary::Conflict => (added, deleted, modified),
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
