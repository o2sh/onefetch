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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_pending_info() {
        let pending_info = PendingInfo {
            pending_changes: "4+-".to_string(),
        };

        assert_eq!(pending_info.value().value, "4+-".to_string());
    }
}
