use gix::bstr::BString;
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
    ) -> Self {
        Self {
            file_churns: compute_file_churns(
                number_of_commits_by_file_path,
                number_of_file_churns_to_display,
            ),
            churn_pool_size,
        }
    }
}

fn compute_file_churns(
    number_of_commits_by_file_path: &HashMap<BString, usize>,
    number_of_file_churns_to_display: usize,
) -> Vec<FileChurn> {
    let mut number_of_commits_by_file_path_sorted = Vec::from_iter(number_of_commits_by_file_path);

    number_of_commits_by_file_path_sorted
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    number_of_commits_by_file_path_sorted
        .into_iter()
        .map(|(file_path, nbr_of_commits)| FileChurn::new(file_path.to_string(), *nbr_of_commits))
        .take(number_of_file_churns_to_display)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_file_churns() {
        let mut number_of_commits_by_file_path = HashMap::new();
        number_of_commits_by_file_path.insert("path/to/file1.txt".into(), 2);
        number_of_commits_by_file_path.insert("path/to/file2.txt".into(), 5);
        number_of_commits_by_file_path.insert("path/to/file3.txt".into(), 3);
        number_of_commits_by_file_path.insert("path/to/file4.txt".into(), 7);
        number_of_commits_by_file_path.insert("foo/x/y/file.txt".into(), 70);
        number_of_commits_by_file_path.insert("foo/x/file.txt".into(), 10);

        let number_of_file_churns_to_display = 3;
        let actual = compute_file_churns(
            &number_of_commits_by_file_path,
            number_of_file_churns_to_display,
        );
        let expected = vec![
            FileChurn::new(String::from("foo/x/y/file.txt"), 70),
            FileChurn::new(String::from("foo/x/file.txt"), 10),
            FileChurn::new(String::from("path/to/file4.txt"), 7),
        ];
        assert_eq!(actual, expected);
    }
}
