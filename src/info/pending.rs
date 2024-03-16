use crate::info::utils::info_field::InfoField;
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

impl std::fmt::Display for PendingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        if self.modified > 0 {
            result = format!("{}+-", self.modified);
        }

        if self.added > 0 {
            result = format!("{result} {}+", self.added);
        }

        if self.deleted > 0 {
            result = format!("{result} {}-", self.deleted);
        }

        write!(f, "{}", result.trim())
    }
}

#[typetag::serialize]
impl InfoField for PendingInfo {
    fn value(&self) -> String {
        self.to_string()
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
            added: 0,
            deleted: 0,
            modified: 4,
        };

        assert_eq!(pending_info.value(), "4+-".to_string());
    }
}
