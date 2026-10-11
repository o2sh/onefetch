use crate::info::git::identity::Identity;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub nbr_of_commits: usize,
    pub contribution: usize,
}

impl Author {
    pub fn new(
        name: String,
        email: Option<String>,
        nbr_of_commits: usize,
        total_nbr_of_commits: usize,
    ) -> Self {
        let contribution =
            (nbr_of_commits as f32 * 100. / total_nbr_of_commits as f32).round() as usize;
        Self {
            name,
            email,
            nbr_of_commits,
            contribution,
        }
    }
}

#[derive(Serialize)]
pub struct Authors {
    pub authors: Vec<Author>,
}

impl Authors {
    pub fn new(
        number_of_commits_by_identity: &HashMap<Identity, usize>,
        total_number_of_commits: usize,
        number_of_authors_to_display: usize,
        show_email: bool,
    ) -> Self {
        let authors = compute_authors(
            number_of_commits_by_identity,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
        );
        Self { authors }
    }
}

fn compute_authors(
    number_of_commits_by_identity: &HashMap<Identity, usize>,
    total_number_of_commits: usize,
    number_of_authors_to_display: usize,
    show_email: bool,
) -> Vec<Author> {
    let mut identities_sorted_by_number_of_commits: Vec<(&Identity, &usize)> =
        Vec::from_iter(number_of_commits_by_identity);

    identities_sorted_by_number_of_commits.sort_by(|(a, a_count), (b, b_count)| {
        b_count.cmp(a_count).then_with(|| a.name.cmp(&b.name))
    });

    let authors: Vec<Author> = identities_sorted_by_number_of_commits
        .into_iter()
        .map(|(author, author_nbr_of_commits)| {
            Author::new(
                author.name.to_string(),
                if show_email {
                    Some(author.email.to_string())
                } else {
                    None
                },
                *author_nbr_of_commits,
                total_number_of_commits,
            )
        })
        .take(number_of_authors_to_display)
        .collect();
    authors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_authors() {
        let mut number_of_commits_by_identity: HashMap<Identity, usize> = HashMap::new();
        number_of_commits_by_identity.insert(
            Identity {
                name: "John Doe".into(),
                email: "johndoe@example.com".into(),
            },
            30,
        );
        number_of_commits_by_identity.insert(
            Identity {
                name: "Jane Doe".into(),
                email: "janedoe@example.com".into(),
            },
            20,
        );
        number_of_commits_by_identity.insert(
            Identity {
                name: "Ellen Smith".into(),
                email: "ellensmith@example.com".into(),
            },
            50,
        );
        let total_number_of_commits = 100;
        let number_of_authors_to_display = 2;
        let show_email = false;

        let actual = compute_authors(
            &number_of_commits_by_identity,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
        );

        let expected = vec![
            Author::new(String::from("Ellen Smith"), None, 50, 100),
            Author::new(String::from("John Doe"), None, 30, 100),
        ];
        assert_eq!(actual, expected);
    }
}
