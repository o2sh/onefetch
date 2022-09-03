use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoFieldValue, InfoType},
};

pub struct CreatedInfo {
    pub creation_date: String,
}

impl CreatedInfo {
    pub fn new(iso_time: bool, commits: &Commits) -> Self {
        let creation_date = commits.get_creation_date(iso_time);
        Self { creation_date }
    }
}

impl InfoField for CreatedInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Created,
            value: self.creation_date.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Created")
    }
}
