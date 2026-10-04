use super::utils::format_number;
use crate::info::langs::get_total_loc;
use crate::info::langs::language::Language;
use crate::{
    cli::NumberSeparator,
    info::utils::{module::Module, text::Line},
};
use serde::Serialize;

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
impl Module for LocInfo {
    fn value(&self) -> Vec<Line> {
        vec![Line::from(format_number(
            &self.lines_of_code,
            self.number_separator,
        ))]
    }

    fn key(&self) -> String {
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

        assert_eq!(loc_info.value(), vec![Line::from("1235")]);
    }
}
