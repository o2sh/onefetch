use super::get_style;
use crate::cli;
use gix::Repository;
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub git_username: String,
    pub git_version: String,
    #[serde(skip_serializing)]
    pub title_color: DynColors,
    #[serde(skip_serializing)]
    pub tilde_color: DynColors,
    #[serde(skip_serializing)]
    pub underline_color: DynColors,
    #[serde(skip_serializing)]
    pub is_bold: bool,
}

impl Title {
    pub fn new(
        repo: &Repository,
        title_color: DynColors,
        tilde_color: DynColors,
        underline_color: DynColors,
        is_bold: bool,
    ) -> Self {
        let git_username = get_git_username(repo);
        let git_version = cli::get_git_version();
        Self {
            git_username,
            git_version,
            title_color,
            tilde_color,
            underline_color,
            is_bold,
        }
    }
}
pub fn get_git_username(repo: &Repository) -> String {
    repo.committer()
        .and_then(Result::ok)
        .map(|c| c.name.to_string())
        .unwrap_or_default()
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !&self.git_username.is_empty() || !&self.git_version.is_empty() {
            let git_info_length = self.git_username.len() + self.git_version.len();
            let title_style = get_style(self.is_bold, self.title_color);

            let (git_info_field_str, git_info_field_len) =
                if !&self.git_username.is_empty() && !&self.git_version.is_empty() {
                    let tilde_style = get_style(self.is_bold, self.tilde_color);
                    (
                        format!(
                            "{} {} {}",
                            &self.git_username.style(title_style),
                            "~".style(tilde_style),
                            &self.git_version.style(title_style)
                        ),
                        git_info_length + 3,
                    )
                } else {
                    (
                        format!(
                            "{}{}",
                            &self.git_username.style(title_style),
                            &self.git_version.style(title_style)
                        ),
                        git_info_length,
                    )
                };

            writeln!(f, "{git_info_field_str}")?;
            let separator = "-".repeat(git_info_field_len);
            writeln!(f, "{}", separator.color(self.underline_color))
        } else {
            Ok(())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use owo_colors::AnsiColors;

    #[test]
    fn test_title_format() -> Result<()> {
        let mut title = Title {
            git_username: "onefetch-committer-name".to_string(),
            git_version: "git version 2.37.2".to_string(),
            title_color: DynColors::Ansi(AnsiColors::Red),
            tilde_color: DynColors::Ansi(AnsiColors::White),
            underline_color: DynColors::Ansi(AnsiColors::Blue),
            is_bold: true,
        };

        title.git_version = "git version 2.37.2".to_string();
        assert!(title.to_string().contains("onefetch-committer-name"));
        assert!(title.to_string().contains('~'));
        assert!(title.to_string().contains("git version 2.37.2"));

        title.git_version = String::new();
        assert!(title.to_string().contains("onefetch-committer-name"));
        assert!(!title.to_string().contains('~'));
        assert!(!title.to_string().contains("git version 2.37.2"));

        title.git_username = String::new();
        let expected_title = String::new();
        assert_eq!(format!("{title}"), expected_title);

        Ok(())
    }
}
