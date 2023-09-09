use crate::info::utils::info_field::InfoField;
use gix::Repository;
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlInfo {
    pub repo_url: String,
}
impl UrlInfo {
    pub fn new(repo_url: &str) -> Self {
        Self {
            repo_url: repo_url.into(),
        }
    }
}

pub fn get_repo_url(repo: &Repository) -> String {
    let config = repo.config_snapshot();
    let remotes = match config.plumbing().sections_by_name("remote") {
        Some(sections) => sections,
        None => return String::default(),
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

    match remote_url {
        Some(url) => remove_token_from_url(&url),
        None => String::default(),
    }
}

fn remove_token_from_url(url: &str) -> String {
    let pattern = Regex::new(r"(https?://)([^@]+@)").unwrap();
    let replaced_url = pattern.replace(url, "$1").to_string();
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
            "https://1234567890abcdefghijklmnopqrstuvwxyz@github.com/jim4067/onefetch.git";
        let res_url = remove_token_from_url(remote_url);
        assert_eq!("https://github.com/jim4067/onefetch.git", res_url);
    }

    #[test]
    fn test_token_removal_gitlab() {
        let remote_url = "https://john:abc123personaltoken@gitlab.com/jim4067/myproject.git";
        let res_url = remove_token_from_url(remote_url);
        assert_eq!("https://gitlab.com/jim4067/myproject.git", res_url);
    }
}
