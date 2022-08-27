use owo_colors::{DynColors, OwoColorize};

use super::get_style;

pub struct Title {
    pub git_username: String,
    pub git_version: String,
    pub title_color: DynColors,
    pub tilde_color: DynColors,
    pub underline_color: DynColors,
    pub is_bold: bool,
}

impl Title {
    fn get_git_info_field(&self) -> (String, usize) {
        let git_info_length = self.git_username.len() + self.git_version.len();
        let title_style = get_style(self.is_bold, self.title_color);

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
        }
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !&self.git_username.is_empty() || !&self.git_version.is_empty() {
            let (git_info_field_str, git_info_field_len) = self.get_git_info_field();
            writeln!(f, "{}", git_info_field_str)?;
            let separator = "-".repeat(git_info_field_len);
            writeln!(f, "{}", separator.color(self.underline_color))
        } else {
            write!(f, "")
        }
    }
}
