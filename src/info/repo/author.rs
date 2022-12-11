use crate::{
    cli::Format,
    info::{
        format_number,
        git::Commits,
        info_field::{InfoField, InfoType},
    },
};
use git_repository as git;
use owo_colors::{DynColors, OwoColorize};
use serde::Serialize;
use std::fmt::Write;

#[derive(Serialize, Clone)]
pub struct Author {
    name: String,
    email: String,
    nbr_of_commits: String,
    contribution: usize,
    #[serde(skip_serializing)]
    show_email: bool,
}

impl Author {
    pub fn new(
        name: git::bstr::BString,
        email: git::bstr::BString,
        nbr_of_commits: usize,
        total_nbr_of_commits: usize,
        show_email: bool,
        format: Option<&Format>,
    ) -> Self {
        let contribution =
            (nbr_of_commits as f32 * 100. / total_nbr_of_commits as f32).round() as usize;
        Self {
            name: name.to_string(),
            email: email.to_string(),
            nbr_of_commits: format_number(nbr_of_commits, format),
            contribution,
            show_email,
        }
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.show_email {
            write!(
                f,
                "{}% {} <{}> {}",
                self.contribution, self.name, self.email, self.nbr_of_commits
            )
        } else {
            write!(
                f,
                "{}% {} {}",
                self.contribution, self.name, self.nbr_of_commits
            )
        }
    }
}

pub struct AuthorsInfo {
    pub authors: Vec<Author>,
    pub info_color: DynColors,
}

impl AuthorsInfo {
    pub fn new(info_color: DynColors, commits: &mut Commits) -> Self {
        let authors = commits.authors.clone();
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
                let _ = write!(authors_info, "{}", author_str);
            } else {
                let _ = write!(authors_info, "\n{:<width$}{}", "", author_str, width = pad);
            }
        }

        write!(f, "{}", authors_info)
    }
}

impl InfoField for AuthorsInfo {
    const TYPE: InfoType = InfoType::Authors;

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

    #[test]
    fn test_display_author() {
        let author = Author::new(
            "John Doe".into(),
            "john.doe@email.com".into(),
            1500,
            2000,
            true,
            None,
        );

        assert_eq!(author.to_string(), "75% John Doe <john.doe@email.com> 1500");
    }

    #[test]
    fn test_display_author_with_no_email() {
        let author = Author::new(
            "John Doe".into(),
            "john.doe@email.com".into(),
            1500,
            2000,
            false,
            None,
        );

        assert_eq!(author.to_string(), "75% John Doe 1500");
    }

    #[test]
    fn test_authors_info_title_with_one_author() {
        let author = Author::new(
            "John Doe".into(),
            "john.doe@email.com".into(),
            1500,
            2000,
            true,
            None,
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
            "john.doe@email.com".into(),
            1500,
            2000,
            true,
            None,
        );

        let author_2 = Author::new(
            "Roberto Berto".into(),
            "bertolone2000@email.com".into(),
            240,
            300,
            false,
            None,
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
            "john.doe@email.com".into(),
            1500,
            2000,
            true,
            None,
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
            "john.doe@email.com".into(),
            1500,
            2000,
            true,
            None,
        );

        let author_2 = Author::new(
            "Roberto Berto".into(),
            "bertolone2000@email.com".into(),
            240,
            300,
            false,
            None,
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
