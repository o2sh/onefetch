use super::info_field::{InfoField, InfoFieldType, InfoFieldValue};
use git_repository as git;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::fmt::Write;

pub struct Author {
    name: String,
    email: Option<String>,
    nbr_of_commits: usize,
    contribution: usize,
}

impl Author {
    pub fn new(
        name: git::bstr::BString,
        email: Option<git::bstr::BString>,
        nbr_of_commits: usize,
        total_nbr_of_commits: usize,
    ) -> Self {
        let contribution =
            (nbr_of_commits as f32 * 100. / total_nbr_of_commits as f32).round() as usize;
        Self {
            name: name.to_string(),
            email: email.map(|e| e.to_string()),
            nbr_of_commits,
            contribution,
        }
    }

    pub fn clear_email(&mut self) {
        self.email = None;
    }
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(email) = &self.email {
            write!(
                f,
                "{}% {} <{}> {}",
                self.contribution, self.name, email, self.nbr_of_commits
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

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Author", 1)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

pub struct AuthorsInfoField {
    pub authors: Vec<Author>,
}

impl std::fmt::Display for AuthorsInfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut author_field = String::from("");

        let pad = "Authors".len() + 2;

        for (i, author) in self.authors.iter().enumerate() {
            if i == 0 {
                let _ = write!(author_field, "{}", author);
            } else {
                let _ = write!(author_field, "\n{:<width$}{}", "", author, width = pad);
            }
        }

        write!(f, "{}", author_field)
    }
}

impl InfoField for AuthorsInfoField {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoFieldType::Authors,
            value: format!("{}", &self),
        }
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
        );

        assert_eq!(
            format!("{}", author),
            "75% John Doe <john.doe@email.com> 1500"
        );
    }

    #[test]
    fn test_display_author_with_no_email() {
        let author = Author::new("John Doe".into(), None, 1500, 2000);

        assert_eq!(format!("{}", author), "75% John Doe 1500");
    }

    #[test]
    fn test_clear_email() {
        let mut author = Author::new(
            "John Doe".into(),
            Some("john.doe@email.com".into()),
            1500,
            2000,
        );

        author.clear_email();

        assert!(author.email.is_none());
    }
}
