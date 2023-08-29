use gengo::Language;
use serde::Serialize;

use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSizeInfo {
    pub size: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl CodeSizeInfo {
    pub fn new(size_by_language: &[(Language, usize)], number_separator: NumberSeparator) -> Self {
        let size = size_by_language.iter().map(|(_, size)| size).sum();
        Self {
            size,
            number_separator,
        }
    }
}

#[typetag::serialize]
impl InfoField for CodeSizeInfo {
    fn value(&self) -> String {
        format_number(&self.size, self.number_separator)
    }

    fn title(&self) -> String {
        "Total blob size".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_loc_info() {
        let loc_info = CodeSizeInfo {
            size: 1235,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
