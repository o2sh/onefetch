use crate::info::info_field::{InfoField, InfoFieldValue, InfoType};
pub struct LocInfo {
    pub lines_of_code: usize,
}

impl InfoField for LocInfo {
    fn value(&self) -> InfoFieldValue {
        InfoFieldValue {
            r#type: InfoType::LinesOfCode,
            value: self.lines_of_code.to_string(),
        }
    }
    fn title(&self) -> String {
        String::from("Lines of code")
    }
}
