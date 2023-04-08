use crate::info::utils::info_field::{InfoField, InfoType};
use anyhow::Result;
use gix::Repository;
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlInfo {
    pub repo_url: String,
}
impl UrlInfo {
    pub fn new(repo: &Repository) -> Result<Self> {
        let repo_url = get_url(repo)?;
        Ok(Self { repo_url })
    }
}

fn get_url(repo: &Repository) -> Result<String> {
    let config = repo.config_snapshot();
    let remotes = match config.plumbing().sections_by_name("remote") {
        Some(sections) => sections,
        None => return Ok(Default::default()),
    };

    let mut remote_url: Option<String> = None;
    for (name, url) in remotes.filter_map(|section| {
        let remote_name = section.header().subsection_name()?;
        let url = section.value("url")?;
        (remote_name, url).into()
    }) {
        remote_url = url.to_string().into();
        if name == "origin" {
            break;
        }
    }

    let remote_url = match remote_url {
        Some(url) => remove_token_from_url(url),
        None => return Ok(Default::default()),
    };

    Ok(remote_url)
}

fn remove_token_from_url(url: String) -> String {
    let pattern = Regex::new(r"(https?://)([^@]+@)").unwrap();
    let replaced_url = pattern.replace(&url, "$1").to_string();
    replaced_url
}

#[typetag::serialize]
impl InfoField for UrlInfo {
    fn value(&self) -> String {
        self.repo_url.to_string()
    }

    fn title(&self) -> String {
        "URL".into()
    }

    fn r#type(&self) -> InfoType {
        InfoType::URL
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
            url_info.value(),
            "git@github.com:o2sh/onefetch.git".to_string()
        );
    }

    #[test]
    fn test_token_removal_github() {
        let remote_url =
            "https://1234567890abcdefghijklmnopqrstuvwxyz@github.com/jim4067/onefetch.git"
                .to_string();
        let res_url = remove_token_from_url(remote_url);
        assert_eq!("https://github.com/jim4067/onefetch.git", res_url);
    }

    #[test]
    fn test_token_removal_gitlab() {
        let remote_url =
            "https://john:abc123personaltoken@gitlab.com/jim4067/myproject.git".to_string();
        let res_url = remove_token_from_url(remote_url);
        assert_eq!("https://gitlab.com/jim4067/myproject.git", res_url);
    }
}
