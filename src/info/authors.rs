use super::git::sig::Sig;
use crate::{
    cli::NumberSeparator,
    info::utils::{format_number, info_field::InfoField},
};
use serde::Serialize;
use std::{collections::HashMap, fmt::Write};

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    email: Option<String>,
    nbr_of_commits: usize,
    contribution: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl Author {
    pub fn new(
        name: String,
        email: Option<String>,
        nbr_of_commits: usize,
        total_nbr_of_commits: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        let contribution =
            (nbr_of_commits as f32 * 100. / total_nbr_of_commits as f32).round() as usize;
        Self {
            name,
            email,
            nbr_of_commits,
            contribution,
            number_separator,
        }
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(email) = &self.email {
            write!(
                f,
                "{}% {} <{}> {}",
                self.contribution,
                self.name,
                email,
                format_number(&self.nbr_of_commits, self.number_separator)
            )
        } else {
            write!(
                f,
                "{}% {} {}",
                self.contribution,
                self.name,
                format_number(&self.nbr_of_commits, self.number_separator)
            )
        }
    }
}

#[derive(Serialize)]
pub struct AuthorsInfo {
    pub authors: Vec<Author>,
}

impl AuthorsInfo {
    pub fn new(
        number_of_commits_by_signature: &HashMap<Sig, usize>,
        total_number_of_commits: usize,
        number_of_authors_to_display: usize,
        show_email: bool,
        number_separator: NumberSeparator,
    ) -> Self {
        let authors = compute_authors(
            number_of_commits_by_signature,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
            number_separator,
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
    number_of_commits_by_signature: &HashMap<Sig, usize>,
    total_number_of_commits: usize,
    number_of_authors_to_display: usize,
    show_email: bool,
    number_separator: NumberSeparator,
) -> Vec<Author> {
    let mut signature_with_number_of_commits_sorted: Vec<(&Sig, &usize)> =
        Vec::from_iter(number_of_commits_by_signature);

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
                *author_nbr_of_commits,
                total_number_of_commits,
                number_separator,
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

impl std::fmt::Display for AuthorsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut authors_info = String::new();

        let pad = self.title().len() + 2;
        for (i, author) in self.authors.iter().enumerate() {
            if i == 0 {
                write!(authors_info, "{author}")?;
            } else {
                write!(
                    authors_info,
                    "\n{:<width$}{}",
                    "",
                    author,
                    width = pad + digit_difference(self.top_contribution(), author.contribution)
                )?;
            }
        }

        write!(f, "{authors_info}")
    }
}

#[typetag::serialize]
impl InfoField for AuthorsInfo {
    fn value(&self) -> String {
        self.to_string()
    }

    fn title(&self) -> String {
        let mut title: String = "Author".into();
        if self.authors.len() > 1 {
            title.push('s')
        }
        title
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ui::text_colors::TextColors;
    use insta::assert_snapshot;
    use owo_colors::DynColors;
    use rstest::rstest;

    #[test]
    fn test_display_author() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        assert_eq!(author.to_string(), "75% John Doe <john.doe@email.com> 1500");
    }

    #[test]
    fn test_display_author_with_no_email() {
        let author = Author::new("John Doe".into(), None, 1500, 2000, NumberSeparator::Plain);

        assert_eq!(author.to_string(), "75% John Doe 1500");
    }

    #[test]
    fn test_authors_info_title_with_one_author() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let authors_info = AuthorsInfo {
            authors: vec![author],
        };

        assert_eq!(authors_info.title(), "Author");
    }

    #[test]
    fn test_authors_info_title_with_two_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let author_2 = Author::new(
            "Roberto Berto".into(),
            None,
            240,
            300,
            NumberSeparator::Plain,
        );

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2],
        };

        assert_eq!(authors_info.title(), "Authors");
    }

    #[test]
    fn test_author_info_with_one_author() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let authors_info = AuthorsInfo {
            authors: vec![author],
        };
        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let mut buffer = String::new();
        authors_info
            .write_styled(&mut buffer, false, &colors)
            .unwrap();

        assert_snapshot!(buffer);
    }

    #[test]
    fn test_author_info_with_two_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let author_2 = Author::new(
            "Roberto Berto".into(),
            None,
            240,
            300,
            NumberSeparator::Plain,
        );

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2],
        };

        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let mut buffer = String::new();
        authors_info
            .write_styled(&mut buffer, false, &colors)
            .unwrap();

        assert_snapshot!(buffer);
    }
    #[test]
    fn test_author_info_alignment_with_three_authors() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let author_2 = Author::new(
            "Roberto Berto".into(),
            None,
            240,
            300,
            NumberSeparator::Plain,
        );

        let author_3 = Author::new("Jane Doe".into(), None, 1, 100, NumberSeparator::Plain);

        let authors_info = AuthorsInfo {
            authors: vec![author, author_2, author_3],
        };

        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let mut buffer = String::new();
        authors_info
            .write_styled(&mut buffer, false, &colors)
            .unwrap();

        assert_snapshot!(buffer);
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
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        number_of_commits_by_signature.insert(
            Sig {
                name: "John Doe".into(),
                email: "johndoe@example.com".into(),
            },
            30,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Jane Doe".into(),
                email: "janedoe@example.com".into(),
            },
            20,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Ellen Smith".into(),
                email: "ellensmith@example.com".into(),
            },
            50,
        );
        let total_number_of_commits = 100;
        let number_of_authors_to_display = 2;
        let show_email = false;
        let number_separator = NumberSeparator::Comma;

        let actual = compute_authors(
            &number_of_commits_by_signature,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
            number_separator,
        );

        let expected = vec![
            Author::new(String::from("Ellen Smith"), None, 50, 100, number_separator),
            Author::new(String::from("John Doe"), None, 30, 100, number_separator),
        ];
        assert_eq!(actual, expected);
    }
}
