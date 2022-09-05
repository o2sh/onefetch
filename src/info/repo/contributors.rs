use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoType},
};
pub struct ContributorsInfo {
    pub number_of_contributors: usize,
    pub number_of_authors_to_display: usize,
}

impl ContributorsInfo {
    pub fn new(commits: &Commits, number_of_authors_to_display: usize) -> Self {
        let contributors = number_of_contributors(commits);
        Self {
            number_of_contributors: contributors,
            number_of_authors_to_display,
        }
    }
}

pub fn number_of_contributors(commits: &Commits) -> usize {
    commits.total_num_authors
}

impl InfoField for ContributorsInfo {
    const TYPE: InfoType = InfoType::Contributors;

    fn value(&self) -> String {
        if self.number_of_contributors > self.number_of_authors_to_display {
            self.number_of_contributors.to_string()
        } else {
            "".to_string()
        }
    }

    fn title(&self) -> String {
        String::from("Contributors")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_contributors_info() {
        let contributors_info = ContributorsInfo {
            number_of_contributors: 12,
            number_of_authors_to_display: 2,
        };

        assert_eq!(contributors_info.value(), "12".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo {
            number_of_contributors: 1,
            number_of_authors_to_display: 3,
        };

        assert!(contributors_info.value().is_empty());
    }
}
