use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoFieldValue, InfoType},
};

pub struct LastChangeInfo {
    pub last_change: String,
}

impl LastChangeInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let last_change = commits.get_date_of_last_commit(iso_time);

        Self { last_change }
    }
}

impl InfoField for LastChangeInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::LastChange,
            value: self.last_change.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Last change")
    }
}
