use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
pub struct ContributorsInfo {
    pub number_of_contributors: usize,
    pub number_of_authors_to_display: usize,
}

impl ContributorsInfo {
    pub fn new(commits: &Commits, number_of_authors_to_display: usize) -> Self {
        let contributors = commits.number_of_contributors();
        Self {
            number_of_contributors: contributors,
            number_of_authors_to_display,
        }
    }
}
impl InfoField for ContributorsInfo {
    fn value(&self) -> InfoFieldValue {
        let number_of_contributors =
            if self.number_of_contributors > self.number_of_authors_to_display {
                self.number_of_contributors.to_string()
            } else {
                "".to_string()
            };

        InfoFieldValue {
            r#type: InfoType::Contributors,
            value: number_of_contributors,
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

        assert_eq!(contributors_info.value().value, "12".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo {
            number_of_contributors: 1,
            number_of_authors_to_display: 3,
        };

        assert!(contributors_info.value().value.is_empty());
    }
}
