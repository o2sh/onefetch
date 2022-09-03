use crate::info::{
    git::Repo,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
use anyhow::Result;

pub struct VersionInfo {
    pub version: String,
}

impl VersionInfo {
    pub fn new(repo: &Repo) -> Result<Self> {
        let version = repo.get_version()?;
        Ok(Self { version })
    }
}
impl InfoField for VersionInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Version,
            value: self.version.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Version")
    }
}
