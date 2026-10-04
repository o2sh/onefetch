use crate::info::langs::get_total_loc;
use crate::info::langs::language::Language;
use crate::info::utils::format_number;
use crate::{
    cli::NumberSeparator,
    info::{info_field::InfoField, text::Line},
};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocInfo {
    pub lines_of_code: usize,
}

impl LocInfo {
    pub fn new(loc_by_language: &[(Language, usize)]) -> Self {
        let lines_of_code = get_total_loc(loc_by_language);
        Self { lines_of_code }
    }
}

#[typetag::serialize]
impl InfoField for LocInfo {
    fn value(&self, separator: NumberSeparator) -> Vec<Line> {
        vec![Line::from(format_number(&self.lines_of_code, separator))]
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
        };

        assert_eq!(
            loc_info.value(NumberSeparator::Plain),
            vec![Line::from("1235")]
        );
    }
}
