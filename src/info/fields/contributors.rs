use crate::info::display_options::DisplayOptions;
use crate::info::{info_field::InfoField, text::Line};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsInfo {
    pub total_number_of_authors: usize,
}

impl ContributorsInfo {
    pub fn new(total_number_of_authors: usize) -> Self {
        Self {
            total_number_of_authors,
        }
    }
}

#[typetag::serialize]
impl InfoField for ContributorsInfo {
    fn value(&self, options: &DisplayOptions) -> Vec<Line> {
        if self.total_number_of_authors > options.number_of_authors {
            vec![Line::from(options.number(&self.total_number_of_authors))]
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

    fn authors_options(number_of_authors: usize) -> DisplayOptions {
        DisplayOptions {
            number_of_authors,
            ..DisplayOptions::default()
        }
    }

    #[test]
    fn test_display_contributors_info() {
        let contributors_info = ContributorsInfo::new(12);
        assert_eq!(
            contributors_info.value(&authors_options(2)),
            vec![Line::from("12")]
        );
        assert_eq!(contributors_info.key(), "Contributors".to_string());
    }

    #[test]
    fn test_display_contributors_less_than_authors_to_display() {
        let contributors_info = ContributorsInfo::new(1);

        assert!(contributors_info.value(&authors_options(3)).is_empty());
    }
}
