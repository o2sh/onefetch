use super::git::metrics::GitMetrics;
use crate::{
    cli::NumberSeparator,
    info::utils::{format_number, info_field::InfoField},
};
use serde::Serialize;
use std::fmt::Write;

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
    pub fn new(git_metrics: &GitMetrics) -> Self {
        let authors = git_metrics.authors_to_display.clone();
        Self { authors }
    }

    fn top_contribution(&self) -> usize {
        if let Some(top_contributor) = self.authors.get(0) {
            return top_contributor.contribution;
        }
        0
    }
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
}
