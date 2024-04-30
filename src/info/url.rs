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

pub fn get_repo_url(repo: &Repository, hide_token: bool, http_url: bool) -> String {
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
        Some(url) => format_url(&url, hide_token, http_url),
        None => String::default(),
    }
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
    let pattern = Regex::new(r"(https?://)([^@]+@)").unwrap();
    let replaced_url = pattern.replace(url, "$1").to_string();
    replaced_url
}

fn create_http_url_from_ssh(url: &str) -> String {
    let pattern = Regex::new(r"([^@]+)@([^:]+):(.*)").unwrap();
    let replaced_url = pattern.replace(url, "https://${2}/${3}").to_string();
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
    use rstest::rstest;

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

    #[test]
    fn test_remove_token_from_url() {
        assert_eq!(
            remove_token_from_url("https://username:token@github.com/user/repo"),
            "https://github.com/user/repo"
        );
    }

    #[rstest]
    #[case("git@github.com:user/repo.git", "https://github.com/user/repo.git")]
    #[case("git@gitlab.com:user/repo", "https://gitlab.com/user/repo")]
    fn test_create_http_url_from_ssh(#[case] url: &str, #[case] expected: &str) {
        assert_eq!(create_http_url_from_ssh(url), expected);
    }
}
