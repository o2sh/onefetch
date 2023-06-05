use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::git::CommitMetrics, utils::info_field::InfoField},
};
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;
use std::fmt::Write;

#[derive(Serialize, Clone)]
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
    #[serde(skip_serializing)]
    pub info_color: DynColors,
}

impl AuthorsInfo {
    pub fn new(info_color: DynColors, commit_metrics: &CommitMetrics) -> Self {
        let authors = commit_metrics.authors_to_display.clone();
        Self {
            authors,
            info_color,
        }
    }
}

impl std::fmt::Display for AuthorsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut authors_info = String::new();

        let pad = self.title().len() + 2;

        for (i, author) in self.authors.iter().enumerate() {
            let author_str = author.color(self.info_color);

            if i == 0 {
                write!(authors_info, "{author_str}")?;
            } else {
                write!(authors_info, "\n{:<width$}{}", "", author_str, width = pad)?;
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

    fn should_color(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
            info_color: DynColors::Rgb(255, 0, 0),
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
            info_color: DynColors::Rgb(255, 0, 0),
            authors: vec![author, author_2],
        };

        assert_eq!(authors_info.title(), "Authors");
    }

    #[test]
    fn test_author_info_value_with_one_author() {
        let author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
            NumberSeparator::Plain,
        );

        let authors_info = AuthorsInfo {
            info_color: DynColors::Rgb(255, 0, 0),
            authors: vec![author],
        };

        assert_eq!(
            authors_info.value(),
            "75% John Doe <john.doe@email.com> 1500"
                .color(DynColors::Rgb(255, 0, 0))
                .to_string()
        );
    }

    #[test]
    fn test_author_info_value_with_two_authors() {
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
            info_color: DynColors::Rgb(255, 0, 0),
            authors: vec![author, author_2],
        };

        assert!(authors_info.value().contains(
            &"75% John Doe <john.doe@email.com> 1500"
                .color(DynColors::Rgb(255, 0, 0))
                .to_string()
        ));

        assert!(authors_info.value().contains(
            &"80% Roberto Berto 240"
                .color(DynColors::Rgb(255, 0, 0))
                .to_string()
        ));
    }
}
