use crate::{
    cli::NumberSeparator,
    info::{
        format_number,
        info_field::{InfoField, InfoType},
    },
};

pub struct LocInfo {
    pub lines_of_code: usize,
    number_separator: NumberSeparator,
}

impl LocInfo {
    pub fn new(lines_of_code: usize, number_separator: NumberSeparator) -> Self {
        Self {
            lines_of_code,
            number_separator,
        }
    }
}

impl InfoField for LocInfo {
    const TYPE: InfoType = InfoType::LinesOfCode;

    fn value(&self) -> String {
        format_number(self.lines_of_code, self.number_separator)
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
