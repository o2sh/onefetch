use crate::git::identity::Identity;
use crate::info::display_options::DisplayOptions;
use crate::info::fields::InfoField;
use crate::info::text::Line;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    email: Option<String>,
    nbr_of_commits: usize,
    contribution: usize,
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
pub struct AuthorsInfo {
    pub authors: Vec<Author>,
}

impl AuthorsInfo {
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

    fn top_contribution(&self) -> usize {
        if let Some(top_contributor) = self.authors.first() {
            return top_contributor.contribution;
        }
        0
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

fn digit_difference(num1: usize, num2: usize) -> usize {
    let count_digits = |num: usize| (num.checked_ilog10().unwrap_or(0) + 1) as usize;
    count_digits(num1).abs_diff(count_digits(num2))
}

#[typetag::serialize]
impl InfoField for AuthorsInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        self.authors
            .iter()
            .map(|author| {
                let pad = digit_difference(self.top_contribution(), author.contribution);
                let contribution = format!("{:pad$}{}%", "", author.contribution);
                let commits = options.number(&author.nbr_of_commits);
                let line = match &author.email {
                    Some(email) => format!("{contribution} {} <{email}> {commits}", author.name),
                    None => format!("{contribution} {} {commits}", author.name),
                };
                Line::from(line)
            })
            .collect()
    }

    fn key(&self) -> String {
        let mut title: String = "Author".into();
        if self.authors.len() > 1 {
            title.push('s');
        }
        title
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_authors_info_title_with_one_author() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
        );

        let authors_info = AuthorsInfo {
            authors: vec![author],
        };

        assert_eq!(authors_info.key(), "Author");
    }

    #[test]
    fn test_authors_info_title_with_two_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
        );

        let author_2 = Author::new("Roberto Berto".into(), None, 240, 300);

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2],
        };

        assert_eq!(authors_info.key(), "Authors");
    }

    #[test]
    fn test_author_info_with_two_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
        );

        let author_2 = Author::new("Roberto Berto".into(), None, 240, 300);

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2],
        };
        assert_eq!(
            authors_info.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
            ]
        );
    }
    #[test]
    fn test_author_info_alignment_with_three_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
        );

        let author_2 = Author::new("Roberto Berto".into(), None, 240, 300);

        let author_3 = Author::new("Jane Doe".into(), None, 1, 100);

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2, author_3],
        };
        assert_eq!(
            authors_info.value(&DisplayOptions::default()),
            vec![
                Line::from("75% John Doe <john.doe@email.com> 1500"),
                Line::from("80% Roberto Berto 240"),
                Line::from(" 1% Jane Doe 1"),
            ]
        );
    }

    #[rstest]
    #[case(456, 123, 0)]
    #[case(456789, 123, 3)]
    #[case(1, 12, 1)]
    fn test_digit_difference(#[case] num1: usize, #[case] num2: usize, #[case] expected: usize) {
        let result = digit_difference(num1, num2);
        assert_eq!(result, expected);
    }

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
