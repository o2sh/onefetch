use crate::{info::utils::get_style, ui::text_colors::TextColors};
use owo_colors::OwoColorize;
use std::fmt;

/// Replaces control characters so untrusted repository data can't inject
/// terminal escape sequences.
fn sanitize(s: &str) -> String {
    s.replace(char::is_control, "\u{FFFD}")
}

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
        let styled_lines: Vec<String> = value
            .lines()
            .map(|line| format!("{}", sanitize(line).style(style)))
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
    use super::*;
    use owo_colors::DynColors;
    use serde::Serialize;

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

    #[test]
    fn test_sanitize_strips_control_chars() {
        // OSC set-title sequence
        assert_eq!(
            sanitize("1.0.0\u{1b}]0;PWNED\u{07}"),
            "1.0.0\u{FFFD}]0;PWNED\u{FFFD}"
        );
    }

    #[test]
    fn test_sanitize_leaves_normal_text_untouched() {
        let input = "some normal description, with punctuation! 42";
        assert_eq!(sanitize(input), input);
    }

    #[test]
    fn test_style_value_strips_control_chars_from_field() {
        // Output still contains ESC from our own color codes.
        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let info = InfoFieldImpl("1.0.0\u{1b}]0;PWNED\u{07}");
        let styled = info.style_value(&colors).unwrap();
        assert!(!styled.contains('\u{07}'));
        assert!(styled.contains("1.0.0"));
        assert_eq!(styled.matches('\u{FFFD}').count(), 2);
    }

    #[test]
    fn test_style_value_crlf_lines() {
        let colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        let styled = InfoFieldImpl("line one\r\nline two")
            .style_value(&colors)
            .unwrap();
        assert_eq!(styled.lines().count(), 2);
        assert!(!styled.contains('\u{FFFD}'), "{styled:?}");
    }

    #[test]
    fn test_sanitize_strips_c1_and_cr() {
        assert_eq!(sanitize("1.0\u{9b}2J"), "1.0\u{FFFD}2J");
        assert_eq!(sanitize("evil\rOK"), "evil\u{FFFD}OK");
    }
}
