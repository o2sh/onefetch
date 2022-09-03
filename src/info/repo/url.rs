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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_url_info() {
        let url_info = UrlInfo {
            repo_url: "git@github.com:o2sh/onefetch.git".to_string(),
        };

        assert_eq!(
            url_info.value().value,
            "git@github.com:o2sh/onefetch.git".to_string()
        );
    }
}
