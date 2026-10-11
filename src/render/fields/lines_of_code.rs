use super::FieldView;
use crate::info::fields::lines_of_code::LinesOfCode;
use crate::render::format::FormatOptions;
use crate::render::text::Line;

impl FieldView for LinesOfCode {
    fn value(&self, options: &FormatOptions) -> Vec<Line> {
        vec![Line::from(options.number(&self.lines_of_code))]
    }

    fn key(&self) -> String {
        "Lines of code".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_lines_of_code() {
        let lines_of_code = LinesOfCode {
            lines_of_code: 1235,
        };

        assert_eq!(
            lines_of_code.value(&FormatOptions::default()),
            vec![Line::from("1235")]
        );
    }
}
