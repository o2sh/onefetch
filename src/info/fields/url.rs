use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use anyhow::Result;
use gix::Repository;
use regex::regex;
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

pub fn get_repo_url(repo: &Repository, hide_token: bool, http_url: bool) -> Result<String> {
    let remote = match repo.find_default_remote(gix::remote::Direction::Fetch) {
        Some(remote) => remote?,
        None => return Ok(String::new()),
    };

    Ok(remote
        .url(gix::remote::Direction::Fetch)
        .map(|url| format_url(&url.to_string(), hide_token, http_url))
        .unwrap_or_default())
}

fn format_url(url: &str, hide_token: bool, http_url: bool) -> String {
    let formatted_url = if hide_token {
        remove_token_from_url(url)
    } else {
        String::from(url)
    };

    if http_url && !formatted_url.starts_with("http") {
        create_http_url_from_ssh(&formatted_url)
    } else {
        formatted_url
    }
}

fn remove_token_from_url(url: &str) -> String {
    regex!(r"(https?://)([^@]+@)")
        .replace(url, "$1")
        .to_string()
}

fn create_http_url_from_ssh(url: &str) -> String {
    regex!(r"([^@]+)@([^:]+):(.*)")
        .replace(url, "https://${2}/${3}")
        .to_string()
}

#[typetag::serialize]
impl InfoField for UrlInfo {
    fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
        vec![Line::from(self.repo_url.to_string())]
    }

    fn key(&self) -> String {
        "URL".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "https://username:token@github.com/user/repo",
        true,
        false,
        "https://github.com/user/repo"
    )]
    #[case(
        "https://user:token@gitlab.com/user/repo",
        true,
        false,
        "https://gitlab.com/user/repo"
    )]
    #[case(
        "git@github.com:user/repo.git",
        false,
        true,
        "https://github.com/user/repo.git"
    )]
    #[case(
        "git@gitlab.com:user/repo",
        false,
        true,
        "https://gitlab.com/user/repo"
    )]
    #[case(
        "https://github.com/user/repo",
        true,
        true,
        "https://github.com/user/repo"
    )]
    #[case(
        "https://username:token@github.com/user/repo",
        false,
        false,
        "https://username:token@github.com/user/repo"
    )]
    fn test_format_url(
        #[case] url: &str,
        #[case] hide_token: bool,
        #[case] http_url: bool,
        #[case] expected: &str,
    ) {
        assert_eq!(format_url(url, hide_token, http_url), expected);
    }

    #[rstest]
    #[case("git@github.com:user/repo.git", "https://github.com/user/repo.git")]
    #[case("git@gitlab.com:user/repo", "https://gitlab.com/user/repo")]
    fn test_create_http_url_from_ssh(#[case] url: &str, #[case] expected: &str) {
        assert_eq!(create_http_url_from_ssh(url), expected);
    }
}
