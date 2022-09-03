use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoFieldValue, InfoType},
};

pub struct CommitsInfo {
    pub number_of_commits: String,
}

impl CommitsInfo {
    pub fn new(commits: &Commits) -> Self {
        let number_of_commits = commits.count();
        Self { number_of_commits }
    }
}
impl InfoField for CommitsInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Commits,
            value: self.number_of_commits.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Commits")
    }
}
