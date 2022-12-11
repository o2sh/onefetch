use crate::{
    cli::Format,
    info::{
        format_number,
        info_field::{InfoField, InfoType},
    },
};

pub struct LocInfo {
    pub lines_of_code: String,
}

impl LocInfo {
    pub fn new(lines_of_code: usize, format: Option<&Format>) -> Self {
        Self {
            lines_of_code: format_number(lines_of_code, format),
        }
    }
}

impl InfoField for LocInfo {
    const TYPE: InfoType = InfoType::LinesOfCode;

    fn value(&self) -> String {
        self.lines_of_code.clone()
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
        let loc_info = LocInfo::new(1235, None);
        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
