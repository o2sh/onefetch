use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;

pub struct UrlInfo {
    pub repo_url: String,
}
impl UrlInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let repo_url = repo.get_url()?;
        Ok(Self { repo_url })
    }
}

impl InfoField for UrlInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Repo,
            value: self.repo_url.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Repo")
    }
}
