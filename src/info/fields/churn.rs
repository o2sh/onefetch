use anyhow::Result;
use gix::bstr::BString;
use globset::{Glob, GlobSetBuilder};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileChurn {
    pub file_path: String,
    pub nbr_of_commits: usize,
}

impl FileChurn {
    pub fn new(file_path: String, nbr_of_commits: usize) -> Self {
        Self {
            file_path,
            nbr_of_commits,
        }
    }
}

#[derive(Serialize)]
pub struct Churn {
    pub file_churns: Vec<FileChurn>,
    pub churn_pool_size: usize,
}

impl Churn {
    pub fn new(
        number_of_commits_by_file_path: &HashMap<BString, usize>,
        churn_pool_size: usize,
        number_of_file_churns_to_display: usize,
        globs_to_exclude: &[String],
    ) -> Result<Self> {
        let file_churns = compute_file_churns(
            number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            globs_to_exclude,
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
            if glob_set.is_match(file_path.to_string()) {
                None
            } else {
                Some(FileChurn::new(file_path.to_string(), *nbr_of_commits))
            }
        })
        .take(number_of_file_churns_to_display)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let globs_to_exclude = vec![
            "foo/**/file.txt".to_string(),
            "path/to/file2.txt".to_string(),
        ];
        let actual = compute_file_churns(
            &number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            &globs_to_exclude,
        )?;
        let expected = vec![
            FileChurn::new(String::from("path/to/file4.txt"), 7),
            FileChurn::new(String::from("path/to/file3.txt"), 3),
            FileChurn::new(String::from("path/to/file1.txt"), 2),
        ];
        assert_eq!(actual, expected);
        Ok(())
    }
}
