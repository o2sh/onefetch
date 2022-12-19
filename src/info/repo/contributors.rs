use crate::{
    cli::NumberSeparator,
    info::{
        format_number,
        git::Commits,
        info_field::{InfoField, InfoType},
    },
};
pub struct ContributorsInfo {
    pub number_of_contributors: usize,
    pub number_of_authors_to_display: usize,
    number_separator: NumberSeparator,
}

impl ContributorsInfo {
    pub fn new(
        commits: &Commits,
        number_of_authors_to_display: usize,
        number_separator: NumberSeparator,
    ) -> Self {
        Self {
            number_of_contributors: commits.total_num_authors,
            number_of_authors_to_display,
            number_separator,
        }
    }
}

impl InfoField for ContributorsInfo {
    const TYPE: InfoType = InfoType::Contributors;

    fn value(&self) -> String {
        if self.number_of_contributors > self.number_of_authors_to_display {
            format_number(self.number_of_contributors, self.number_separator)
        } else {
            "".to_string()
        }
    }

    fn title(&self) -> String {
        "Contributors".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_contributors_info() {
        use crate::info::git::Commits;
        use git_repository::actor::Time;

        let timestamp = Time::now_utc();
        let commits = Commits {
            authors: vec![],
            total_num_authors: 12,
            num_commits: 2,
            is_shallow: true,
            time_of_most_recent_commit: timestamp,
            time_of_first_commit: timestamp,
        };

        let contributors_info = ContributorsInfo::new(&commits, 2, NumberSeparator::Plain);
        assert_eq!(contributors_info.value(), "12".to_string());
        assert_eq!(contributors_info.title(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo {
            number_of_contributors: 1,
            number_of_authors_to_display: 3,
            number_separator: NumberSeparator::Plain,
        };

        assert!(contributors_info.value().is_empty());
    }
}
