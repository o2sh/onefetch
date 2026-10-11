use crate::language::Language;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinesOfCode {
    pub lines_of_code: usize,
}

impl LinesOfCode {
    pub fn new(loc_by_language: &[(Language, usize)]) -> Self {
        let lines_of_code = loc_by_language.iter().map(|(_, loc)| loc).sum();
        Self { lines_of_code }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_lines_of_code() {
        let loc_by_language = [(Language::JavaScript, 100), (Language::Markdown, 300)];
        assert_eq!(LinesOfCode::new(&loc_by_language).lines_of_code, 400);
    }
}
