use crate::info::langs::get_total_loc;
use crate::info::langs::language::Language;
use serde::Serialize;

use crate::{
    cli::NumberSeparator,
    info::{format_number, utils::info_field::InfoField},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocInfo {
    pub lines_of_code: usize,
    #[serde(skip_serializing)]
    number_separator: NumberSeparator,
}

impl LocInfo {
    pub fn new(loc_by_language: &[(Language, usize)], number_separator: NumberSeparator) -> Self {
        let lines_of_code = get_total_loc(loc_by_language);
        Self {
            lines_of_code,
            number_separator,
        }
    }
}

#[typetag::serialize]
impl InfoField for LocInfo {
    fn value(&self) -> String {
        format_number(&self.lines_of_code, self.number_separator)
    }

    fn title(&self) -> String {
        "Lines of code".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_loc_info() {
        let loc_info = LocInfo {
            lines_of_code: 1235,
            number_separator: NumberSeparator::Plain,
        };

        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
