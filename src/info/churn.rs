use super::utils::info_field::InfoField;
use crate::{cli::NumberSeparator, info::utils::format_number};
use anyhow::Result;
use gix::bstr::BString;
use globset::{Glob, GlobSetBuilder};
use serde::Serialize;
use std::{collections::HashMap, fmt::Write};

#[derive(Serialize, Clone, Debug, PartialEq)]
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
}

impl ChurnInfo {
    pub fn new(
        number_of_commits_by_file_path: &HashMap<BString, usize>,
        churn_pool_size: usize,
        number_of_file_churns_to_display: usize,
        globs_to_exclude: &[String],
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        let file_churns = compute_file_churns(
            number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            globs_to_exclude,
            number_separator,
        )?;

        Ok(Self {
            file_churns,
            churn_pool_size,
        })
    }
}

fn compute_file_churns(
    number_of_commits_by_file_path: &HashMap<BString, usize>,
    number_of_file_churns_to_display: usize,
    globs_to_exclude: &[String],
    number_separator: NumberSeparator,
) -> Result<Vec<FileChurn>> {
    let mut builder = GlobSetBuilder::new();
    for glob in globs_to_exclude {
        builder.add(Glob::new(glob)?);
    }
    let glob_set = builder.build()?;
    let mut number_of_commits_by_file_path_sorted = Vec::from_iter(number_of_commits_by_file_path);

    number_of_commits_by_file_path_sorted
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    Ok(number_of_commits_by_file_path_sorted
        .into_iter()
        .filter_map(|(file_path, nbr_of_commits)| {
            if !glob_set.is_match(file_path.to_string()) {
                Some(FileChurn::new(
                    file_path.to_string(),
                    *nbr_of_commits,
                    number_separator,
                ))
            } else {
                None
            }
        })
        .take(number_of_file_churns_to_display)
        .collect())
}

impl std::fmt::Display for ChurnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut churn_info = String::new();

        let pad = self.title().len() + 2;

        for (i, file_churn) in self.file_churns.iter().enumerate() {
            if i == 0 {
                write!(churn_info, "{file_churn}")?;
            } else {
                write!(churn_info, "\n{:<width$}{}", "", file_churn, width = pad)?;
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
        };

        assert!(churn_info
            .value()
            .contains(&"\u{2026}/to/file.txt 50".to_string()));

        assert!(churn_info.value().contains(&"file_2.txt 30".to_string()));
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

    #[test]
    fn test_compute_file_churns() -> Result<()> {
        let mut number_of_commits_by_file_path = HashMap::new();
        number_of_commits_by_file_path.insert("path/to/file1.txt".into(), 2);
        number_of_commits_by_file_path.insert("path/to/file2.txt".into(), 5);
        number_of_commits_by_file_path.insert("path/to/file3.txt".into(), 3);
        number_of_commits_by_file_path.insert("path/to/file4.txt".into(), 7);
        number_of_commits_by_file_path.insert("foo/x/y/file.txt".into(), 70);
        number_of_commits_by_file_path.insert("foo/x/file.txt".into(), 10);

        let number_of_file_churns_to_display = 3;
        let number_separator = NumberSeparator::Comma;
        let globs_to_exclude = vec![
            "foo/**/file.txt".to_string(),
            "path/to/file2.txt".to_string(),
        ];
        let actual = compute_file_churns(
            &number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            &globs_to_exclude,
            number_separator,
        )?;
        let expected = vec![
            FileChurn::new(String::from("path/to/file4.txt"), 7, number_separator),
            FileChurn::new(String::from("path/to/file3.txt"), 3, number_separator),
            FileChurn::new(String::from("path/to/file1.txt"), 2, number_separator),
        ];
        assert_eq!(actual, expected);
        Ok(())
    }
}
