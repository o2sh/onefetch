use crate::{
    cli::NumberFormat,
    info::{
        format_number,
        git::Commits,
        info_field::{InfoField, InfoType},
    },
};
pub struct ContributorsInfo {
    pub number_of_contributors: String,
}

impl ContributorsInfo {
    pub fn new(
        commits: &Commits,
        number_of_authors_to_display: usize,
        number_format: Option<&NumberFormat>,
    ) -> Self {
        let number_of_contributors = number_of_contributors(
            commits.total_num_authors,
            number_of_authors_to_display,
            number_format,
        );
        Self {
            number_of_contributors,
        }
    }
}

fn number_of_contributors(
    total_num_authors: usize,
    number_of_authors_to_display: usize,
    number_format: Option<&NumberFormat>,
) -> String {
    if total_num_authors > number_of_authors_to_display {
        format_number(total_num_authors, number_format)
    } else {
        "".to_string()
    }
}

impl InfoField for ContributorsInfo {
    const TYPE: InfoType = InfoType::Contributors;

    fn value(&self) -> String {
        self.number_of_contributors.clone()
    }

    fn title(&self) -> String {
        "Contributors".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::git::Commits;
    use git_repository::actor::Time;

    #[test]
    fn test_display_contributors_info() {
        let timestamp = Time::now_utc();
        let commits = Commits {
            authors: vec![],
            total_num_authors: 12,
            num_commits: 2,
            is_shallow: true,
            time_of_most_recent_commit: timestamp,
            time_of_first_commit: timestamp,
        };

        let contributors_info = ContributorsInfo::new(&commits, 2, None);
        assert_eq!(contributors_info.value(), "12".to_string());
        assert_eq!(contributors_info.title(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let timestamp = Time::now_utc();
        let commits = Commits {
            authors: vec![],
            total_num_authors: 1,
            num_commits: 2,
            is_shallow: true,
            time_of_most_recent_commit: timestamp,
            time_of_first_commit: timestamp,
        };

        let contributors_info = ContributorsInfo::new(&commits, 2, None);

        assert!(contributors_info.value().is_empty());
    }
}
