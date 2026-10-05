use crate::info::display_options::DisplayOptions;
use crate::info::{info_field::InfoField, text::Line};
use anyhow::Result;
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingInfo {
    added: usize,
    deleted: usize,
    modified: usize,
}

impl PendingInfo {
    pub fn new(repo: &Repository) -> Result<Self> {
        let statuses = repo
            .status(gix::progress::Discard)?
            .dirwalk_options(|options| {
                options.emit_untracked(gix::dir::walk::EmissionMode::Matching)
            })
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
        Ok(Self {
            added,
            deleted,
            modified,
        })
    }
}

#[typetag::serialize]
impl InfoField for PendingInfo {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        let mut pending = String::new();
        if self.modified > 0 {
            pending = format!("{}+-", self.modified);
        }

        if self.added > 0 {
            pending = format!("{pending} {}+", self.added);
        }

        if self.deleted > 0 {
            pending = format!("{pending} {}-", self.deleted);
        }

        vec![Line::from(pending.trim())]
    }

    fn key(&self) -> String {
        "Pending".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_pending_info() {
        let pending_info = PendingInfo {
            added: 0,
            deleted: 0,
            modified: 4,
        };

        assert_eq!(
            pending_info.value(&DisplayOptions::default()),
            vec![Line::from("4+-")]
        );
    }
}
