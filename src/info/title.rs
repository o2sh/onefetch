use crate::cli;
use crate::info::utils::text::{Line, Span, Style};
use gix::Repository;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub git_username: String,
    pub git_version: String,
}

impl Title {
    pub fn new(repo: &Repository) -> Self {
        Self {
            git_username: get_git_username(repo),
            git_version: cli::get_git_version(),
        }
    }

    pub fn line(&self) -> Line {
        let mut spans = Vec::new();
        for part in [&self.git_username, &self.git_version] {
            if part.is_empty() {
                continue;
            }
            if !spans.is_empty() {
                spans.push(Span::plain(" "));
                spans.push(Span::new("~", Style::Tilde));
                spans.push(Span::plain(" "));
            }
            spans.push(Span::new(part, Style::Title));
        }
        Line::from(spans)
    }
}

pub fn get_git_username(repo: &Repository) -> String {
    repo.committer()
        .and_then(Result::ok)
        .map(|c| c.name.to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn title(git_username: &str, git_version: &str) -> Title {
        Title {
            git_username: git_username.into(),
            git_version: git_version.into(),
        }
    }

    #[test]
    fn test_title_line() {
        let title = title("onefetch-committer-name", "git version 2.37.2");
        assert_eq!(
            title.line(),
            Line::from(vec![
                Span::new("onefetch-committer-name", Style::Title),
                Span::plain(" "),
                Span::new("~", Style::Tilde),
                Span::plain(" "),
                Span::new("git version 2.37.2", Style::Title),
            ])
        );
    }

    #[test]
    fn test_title_line_without_git_version() {
        let title = title("onefetch-committer-name", "");
        assert_eq!(
            title.line(),
            Line::from(vec![Span::new("onefetch-committer-name", Style::Title)])
        );
    }

    #[test]
    fn test_title_line_unknown() {
        assert!(title("", "").line().is_empty());
    }
}
