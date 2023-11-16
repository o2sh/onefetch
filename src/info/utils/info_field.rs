use super::super::get_style;
use crate::ui::text_colors::TextColors;
use owo_colors::OwoColorize;
use std::fmt;

#[typetag::serialize]
pub trait InfoField {
    fn value(&self) -> String;
    fn title(&self) -> String;

    /// Writes the styled info field. If the info doesn't have a value, nothing is
    /// written.
    fn write_styled(
        &self,
        w: &mut dyn fmt::Write,
        no_bold: bool,
        text_colors: &TextColors,
    ) -> fmt::Result {
        if let Some(styled_value) = self.style_value(text_colors) {
            writeln!(
                w,
                "{} {}",
                self.style_title(text_colors, no_bold),
                styled_value
            )
        } else {
            Ok(())
        }
    }

    /// Returns a styled version of the info field's title.
    fn style_title(&self, text_colors: &TextColors, no_bold: bool) -> String {
        let subtitle_style = get_style(!no_bold, text_colors.subtitle);
        let colon_style = get_style(!no_bold, text_colors.colon);
        format!(
            "{}{}",
            self.title().style(subtitle_style),
            ":".style(colon_style)
        )
    }

    /// Returns a styled version of the info field's value. This can be `None` if the
    /// value is empty.
    fn style_value(&self, text_colors: &TextColors) -> Option<String> {
        let value = self.value();
        if value.is_empty() {
            return None;
        }
        let style = get_style(false, text_colors.info);
        let styled_lines: Vec<String> = self
            .value()
            .lines()
            .map(|line| format!("{}", line.style(style)))
            .collect();
        Some(styled_lines.join("\n"))
    }
}

#[derive(Clone, clap::ValueEnum, Debug, Eq, PartialEq)]
pub enum InfoType {
    Project,
    Description,
    Head,
    Pending,
    Version,
    Created,
    Languages,
    Dependencies,
    Authors,
    LastChange,
    Contributors,
    URL,
    Commits,
    Churn,
    LinesOfCode,
    Size,
    License,
}

#[cfg(test)]
mod test {
    use owo_colors::DynColors;
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct InfoFieldImpl(&'static str);

    #[typetag::serialize]
    impl InfoField for InfoFieldImpl {
        fn value(&self) -> String {
            self.0.into()
        }

        fn title(&self) -> String {
            "title".into()
        }
    }

    #[test]
    fn test_info_field_with_value() {
        let info = InfoFieldImpl("test");
        assert_eq!(info.title(), "title".to_string());
        assert_eq!(info.value(), "test".to_string());
    }

    #[test]
    fn test_write_styled() {
        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let info = InfoFieldImpl("test");
        let mut buffer = String::new();
        info.write_styled(&mut buffer, false, &colors).unwrap();
        insta::assert_snapshot!(buffer);
    }

    #[test]
    fn test_write_styled_no_value() {
        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let info = InfoFieldImpl("");
        let mut buffer = String::new();
        info.write_styled(&mut buffer, false, &colors).unwrap();
        assert_eq!(buffer, "", "It should not write anything");
    }
}
