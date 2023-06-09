use super::utils::{git::CommitMetrics, info_field::InfoField};
use crate::{cli::NumberSeparator, info::format_number};
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;
use std::fmt::Write;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileChurn {
    pub file_path: String,
    pub nbr_of_commits: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl FileChurn {
    pub fn new(
        file_path: String,
        nbr_of_commits: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        Self {
            file_path,
            nbr_of_commits,
            number_separator,
        }
    }
}

impl std::fmt::Display for FileChurn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            shorten_file_path(&self.file_path, 2),
            format_number(&self.nbr_of_commits, self.number_separator)
        )
    }
}

#[derive(Serialize)]
pub struct ChurnInfo {
    pub file_churns: Vec<FileChurn>,
    pub churn_pool_size: usize,
    #[serde(skip_serializing)]
    pub info_color: DynColors,
}
impl ChurnInfo {
    pub fn new(info_color: DynColors, commit_metrics: &CommitMetrics) -> Self {
        let file_churns = commit_metrics.file_churns_to_display.clone();
        Self {
            file_churns,
            churn_pool_size: commit_metrics.churn_pool_size,
            info_color,
        }
    }
}
impl std::fmt::Display for ChurnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut churn_info = String::new();

        let pad = self.title().len() + 2;

        for (i, churn) in self.file_churns.iter().enumerate() {
            let churn_str = churn.color(self.info_color);

            if i == 0 {
                write!(churn_info, "{churn_str}")?;
            } else {
                write!(churn_info, "\n{:<width$}{}", "", churn_str, width = pad)?;
            }
        }

        write!(f, "{churn_info}")
    }
}

#[typetag::serialize]
impl InfoField for ChurnInfo {
    fn value(&self) -> String {
        self.to_string()
    }

    fn title(&self) -> String {
        format!("Churn ({})", self.churn_pool_size)
    }

    fn should_color(&self) -> bool {
        false
    }
}

fn shorten_file_path(file_path: &str, depth: usize) -> String {
    let components: Vec<&str> = file_path.split('/').collect();

    if depth == 0 || components.len() <= depth {
        return file_path.to_string();
    }

    let truncated_components: Vec<&str> = components
        .iter()
        .skip(components.len() - depth)
        .copied()
        .collect();

    format!("\u{2026}/{}", truncated_components.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_file_churn() {
        let file_churn = FileChurn::new("path/to/file.txt".into(), 50, NumberSeparator::Plain);

        assert_eq!(file_churn.to_string(), "\u{2026}/to/file.txt 50");
    }

    #[test]
    fn test_churn_info_value_with_two_file_churns() {
        let file_churn_1 = FileChurn::new("path/to/file.txt".into(), 50, NumberSeparator::Plain);
        let file_churn_2 = FileChurn::new("file_2.txt".into(), 30, NumberSeparator::Plain);

        let churn_info = ChurnInfo {
            file_churns: vec![file_churn_1, file_churn_2],
            churn_pool_size: 5,
            info_color: DynColors::Rgb(255, 0, 0),
        };

        assert!(churn_info.value().contains(
            &"\u{2026}/to/file.txt 50"
                .color(DynColors::Rgb(255, 0, 0))
                .to_string()
        ));

        assert!(churn_info
            .value()
            .contains(&"file_2.txt 30".color(DynColors::Rgb(255, 0, 0)).to_string()));
    }

    #[test]
    fn test_truncate_file_path() {
        assert_eq!(shorten_file_path("path/to/file.txt", 3), "path/to/file.txt");
        assert_eq!(shorten_file_path("another/file.txt", 2), "another/file.txt");
        assert_eq!(shorten_file_path("file.txt", 1), "file.txt");
        assert_eq!(
            shorten_file_path("path/to/file.txt", 2),
            "\u{2026}/to/file.txt"
        );
        assert_eq!(
            shorten_file_path("another/file.txt", 1),
            "\u{2026}/file.txt"
        );
        assert_eq!(shorten_file_path("file.txt", 0), "file.txt");
    }
}
