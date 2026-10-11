use anyhow::Result;
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending {
    pub added: usize,
    pub deleted: usize,
    pub modified: usize,
}

impl Pending {
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
