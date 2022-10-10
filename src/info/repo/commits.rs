use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoType},
};

pub struct CommitsInfo {
    pub number_of_commits: String,
}

impl CommitsInfo {
    pub fn new(commits: &Commits) -> Self {
        let number_of_commits = count(commits);
        Self { number_of_commits }
    }
}

pub fn count(commits: &Commits) -> String {
    format!(
        "{}{}",
        commits.num_commits,
        commits.is_shallow.then(|| " (shallow)").unwrap_or_default()
    )
}
impl InfoField for CommitsInfo {
    const TYPE: InfoType = InfoType::Commits;

    fn value(&self) -> String {
        self.number_of_commits.to_string()
    }

    fn title(&self) -> String {
        String::from("Commits")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_commits_info() {
        let commits_info = CommitsInfo {
            number_of_commits: "3".to_string(),
        };

        assert_eq!(commits_info.value(), "3".to_string());
    }
}
