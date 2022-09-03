use crate::info::{
    git::get_pending_changes,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;

pub struct PendingInfo {
    pub pending_changes: String,
}

impl PendingInfo {
    pub fn new(repo: &git_repository::Repository) -> Result<Self> {
        let git_dir = repo.git_dir().to_owned();
        let repo = git2::Repository::open(git_dir)?;
        let pending_changes = get_pending_changes(&repo)?;
        Ok(Self { pending_changes })
    }
}

impl InfoField for PendingInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Pending,
            value: self.pending_changes.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Pending")
    }
}
