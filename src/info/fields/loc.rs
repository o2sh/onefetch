use crate::info::format::Format;
use crate::info::langs::get_total_loc;
use crate::info::langs::language::Language;
use crate::info::{info_field::InfoField, text::Line};
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
    fn value(&self, format: &Format) -> Vec<Line> {
        vec![Line::from(format.number(&self.lines_of_code))]
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

        assert_eq!(loc_info.value(&Format::default()), vec![Line::from("1235")]);
    }
}
