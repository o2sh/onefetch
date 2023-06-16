use super::sig::Sig;
use crate::cli::{CliOptions, NumberSeparator};
use crate::info::author::Author;
use crate::info::churn::FileChurn;
use anyhow::Result;
use gix::bstr::BString;
use gix::date::Time;
use std::collections::HashMap;

pub struct GitMetrics {
    pub authors_to_display: Vec<Author>,
    pub file_churns_to_display: Vec<FileChurn>,
    pub total_number_of_authors: usize,
    pub total_number_of_commits: usize,
    pub churn_pool_size: usize,
    pub time_of_most_recent_commit: gix::actor::Time,
    pub time_of_first_commit: gix::actor::Time,
}

impl GitMetrics {
    pub fn new(
        number_of_commits_by_signature: HashMap<Sig, usize>,
        number_of_commits_by_file_path: HashMap<BString, usize>,
        churn_pool_size: usize,
        time_of_first_commit: Option<Time>,
        time_of_most_recent_commit: Option<Time>,
        options: &CliOptions,
    ) -> Result<Self> {
        let total_number_of_commits = number_of_commits_by_signature.values().sum();
        let (authors_to_display, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            total_number_of_commits,
            options.info.number_of_authors,
            options.info.email,
            options.text_formatting.number_separator,
        );

        let file_churns_to_display = compute_file_churns(
            number_of_commits_by_file_path,
            options.info.number_of_file_churns,
            options.text_formatting.number_separator,
        );

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        Ok(Self {
            authors_to_display,
            file_churns_to_display,
            total_number_of_authors,
            total_number_of_commits,
            churn_pool_size,
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }
}

fn compute_file_churns(
    number_of_commits_by_file_path: HashMap<BString, usize>,
    number_of_file_churns_to_display: usize,
    number_separator: NumberSeparator,
) -> Vec<FileChurn> {
    let mut number_of_commits_by_file_path_sorted = Vec::from_iter(number_of_commits_by_file_path);

    number_of_commits_by_file_path_sorted
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    number_of_commits_by_file_path_sorted
        .into_iter()
        .map(|(file_path, nbr_of_commits)| {
            FileChurn::new(file_path.to_string(), nbr_of_commits, number_separator)
        })
        .take(number_of_file_churns_to_display)
        .collect()
}

fn compute_authors(
    number_of_commits_by_signature: HashMap<Sig, usize>,
    total_number_of_commits: usize,
    number_of_authors_to_display: usize,
    show_email: bool,
    number_separator: NumberSeparator,
) -> (Vec<Author>, usize) {
    let total_number_of_authors = number_of_commits_by_signature.len();
    let mut signature_with_number_of_commits_sorted: Vec<(Sig, usize)> =
        number_of_commits_by_signature.into_iter().collect();

    signature_with_number_of_commits_sorted.sort_by(|(sa, a_count), (sb, b_count)| {
        b_count.cmp(a_count).then_with(|| sa.name.cmp(&sb.name))
    });

    let authors: Vec<Author> = signature_with_number_of_commits_sorted
        .into_iter()
        .map(|(author, author_nbr_of_commits)| {
            Author::new(
                author.name.to_string(),
                if show_email {
                    Some(author.email.to_string())
                } else {
                    None
                },
                author_nbr_of_commits,
                total_number_of_commits,
                number_separator,
            )
        })
        .take(number_of_authors_to_display)
        .collect();
    (authors, total_number_of_authors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_authors() {
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        number_of_commits_by_signature.insert(
            Sig {
                name: "John Doe".into(),
                email: "johndoe@example.com".into(),
            },
            10,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Jane Doe".into(),
                email: "janedoe@example.com".into(),
            },
            5,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Ellen Smith".into(),
                email: "ellensmith@example.com".into(),
            },
            50,
        );
        let total_number_of_commits = 15;
        let number_of_authors_to_display = 2;
        let show_email = false;
        let number_separator = NumberSeparator::Comma;

        let (authors, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
            number_separator,
        );

        assert_eq!(total_number_of_authors, 3);
        assert_eq!(authors.len(), 2);
        assert_eq!(authors.get(0).unwrap().name, "Ellen Smith".to_string());
    }

    #[test]
    fn test_compute_file_churns() {
        let mut number_of_commits_by_file_path = HashMap::new();
        number_of_commits_by_file_path.insert("path/to/file1.txt".into(), 2);
        number_of_commits_by_file_path.insert("path/to/file2.txt".into(), 5);
        number_of_commits_by_file_path.insert("path/to/file3.txt".into(), 3);
        number_of_commits_by_file_path.insert("path/to/file4.txt".into(), 7);

        let number_of_file_churns_to_display = 3;
        let number_separator = NumberSeparator::Comma;
        let file_churns = compute_file_churns(
            number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            number_separator,
        );

        assert_eq!(file_churns.len(), 3);
        assert_eq!(
            file_churns.get(0).unwrap().file_path,
            "path/to/file4.txt".to_string()
        );
        assert_eq!(file_churns.get(0).unwrap().nbr_of_commits, 7);
    }
}
