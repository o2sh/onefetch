use crate::info::info_field::InfoField;
use crate::info::langs::get_total_loc;
use crate::info::langs::language::Language;
use crate::info::text::{Line, Span};
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
    fn value(&self) -> Vec<Line> {
        vec![Span::number(self.lines_of_code as u64).into()]
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

        assert_eq!(loc_info.value(), vec![Line::from(vec![Span::number(1235)])]);
    }
}
