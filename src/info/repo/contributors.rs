use crate::info::{
    git::Commits,
    info_field::{InfoField, InfoFieldValue, InfoType},
};
pub struct ContributorsInfo {
    pub number_of_contributors: usize,
}

impl ContributorsInfo {
    pub fn new(commits: &Commits) -> Self {
        let contributors = commits.number_of_contributors();
        Self {
            number_of_contributors: contributors,
        }
    }
}
impl InfoField for ContributorsInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::Contributors,
            value: self.number_of_contributors.to_string(),
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
        };

        assert_eq!(contributors_info.value().value, "12".to_string());
    }
}
