use crate::{
    cli::ThousandsSeparator,
    info::{
        format_number,
        git::Commits,
        info_field::{InfoField, InfoType},
    },
};

pub struct CommitsInfo {
    pub number_of_commits: String,
}

impl CommitsInfo {
    pub fn new(commits: &Commits, thousands_separator: Option<&ThousandsSeparator>) -> Self {
        let number_of_commits = number_of_commits(commits, thousands_separator);
        Self { number_of_commits }
    }
}

fn number_of_commits(
    commits: &Commits,
    thousands_separator: Option<&ThousandsSeparator>,
) -> String {
    format!(
        "{}{}",
        format_number(commits.num_commits, thousands_separator),
        commits.is_shallow.then(|| " (shallow)").unwrap_or_default()
    )
}
impl InfoField for CommitsInfo {
    const TYPE: InfoType = InfoType::Commits;

    fn value(&self) -> String {
        self.number_of_commits.to_string()
    }

    fn title(&self) -> String {
        "Commits".into()
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

    #[test]
    fn test_display_commits_info_shallow() {
        use crate::info::git::Commits;
        use git_repository::actor::Time;

        let timestamp = Time::now_utc();
        let commits = Commits {
            authors: vec![],
            total_num_authors: 0,
            num_commits: 2,
            is_shallow: true,
            time_of_most_recent_commit: timestamp,
            time_of_first_commit: timestamp,
        };

        let info = CommitsInfo::new(&commits, None);
        assert_eq!(info.value(), "2 (shallow)".to_string());
    }
}
