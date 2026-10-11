use crate::info::loc::get_total_loc;
use crate::language::Language;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinesOfCode {
    pub lines_of_code: usize,
}

impl LinesOfCode {
    pub fn new(loc_by_language: &[(Language, usize)]) -> Self {
        let lines_of_code = get_total_loc(loc_by_language);
        Self { lines_of_code }
    }
}
