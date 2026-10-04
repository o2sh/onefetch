use crate::info::format::Format;
use crate::info::{info_field::InfoField, text::Line};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsInfo {
    pub total_number_of_authors: usize,
    #[serde(skip_serializing)]
    pub number_of_authors_to_display: usize,
}

impl ContributorsInfo {
    pub fn new(total_number_of_authors: usize, number_of_authors_to_display: usize) -> Self {
        Self {
            total_number_of_authors,
            number_of_authors_to_display,
        }
    }
}

#[typetag::serialize]
impl InfoField for ContributorsInfo {
    fn value(&self, format: &Format) -> Vec<Line> {
        if self.total_number_of_authors > self.number_of_authors_to_display {
            vec![Line::from(format.number(&self.total_number_of_authors))]
        } else {
            Vec::new()
        }
    }

    fn key(&self) -> String {
        "Contributors".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_contributors_info() {
        let contributors_info = ContributorsInfo::new(12, 2);
        assert_eq!(
            contributors_info.value(&Format::default()),
            vec![Line::from("12")]
        );
        assert_eq!(contributors_info.key(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo {
            total_number_of_authors: 1,
            number_of_authors_to_display: 3,
        };

        assert!(contributors_info.value(&Format::default()).is_empty());
    }
}
