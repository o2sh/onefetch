use crate::info::info_field::{InfoField, InfoType};

pub struct LocInfo {
    pub lines_of_code: usize,
}

impl InfoField for LocInfo {
    const TYPE: InfoType = InfoType::LinesOfCode;

    fn value(&self) -> String {
        self.lines_of_code.to_string()
    }

    fn title(&self) -> String {
        String::from("Lines of code")
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

        assert_eq!(loc_info.value(), "1235".to_string());
    }
}
