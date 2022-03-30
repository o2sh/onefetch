use git_repository as git;
use serde::ser::SerializeStruct;
use serde::Serialize;

pub struct Author {
    name: git::bstr::BString,
    email: Option<git::bstr::BString>,
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
            name,
            email,
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
