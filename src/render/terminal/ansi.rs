use super::sanitize::sanitize;
use crate::render::text::Line;
use crate::render::theme::Theme;
use anyhow::Result;
use owo_colors::OwoColorize;

/// Writes the lines with the escape codes of their styles.
pub(crate) fn paint(lines: &[Line], theme: &Theme) -> String {
    let mut output = String::new();
    for line in lines {
        for span in &line.0 {
            let style = theme.style(span.style);
            output.push_str(&sanitize(&span.text).style(style).to_string());
        }
        output.push('\n');
    }
    output
}

pub(crate) fn write_unwrapped(writer: &mut dyn std::io::Write, content: &str) -> Result<()> {
    // \x1B[?7l turns off line wrapping and \x1B[?7h turns it on
    write!(writer, "\x1B[?7l{content}\x1B[?7h")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::text::{Span, Style};
    use owo_colors::{AnsiColors, DynColors};

    fn paint_with_white(lines: &[Line], bold: bool) -> String {
        let theme = Theme::with_text_colors(vec![DynColors::Rgb(0xFF, 0xFF, 0xFF)], &[], bold);
        paint(lines, &theme)
    }

    #[test]
    fn test_paint() {
        let line = Line::from(vec![
            Span::new("title", Style::Key),
            Span::new(":", Style::Separator),
            Span::plain(" "),
            Span::value("test"),
        ]);
        assert_eq!(
            paint_with_white(&[line], true),
            "\u{1b}[38;2;255;255;255;1mtitle\u{1b}[0m\u{1b}[39;1m:\u{1b}[0m \u{1b}[39mtest\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_no_bold() {
        let line = Line::from(vec![Span::new("title", Style::Key)]);
        assert_eq!(
            paint_with_white(&[line], false),
            "\u{1b}[38;2;255;255;255mtitle\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_explicit_colors() {
        let red = DynColors::Ansi(AnsiColors::Red);
        let line = Line::from(vec![
            Span::new("  ", Style::Background(red)),
            Span::new("●", Style::Color(red)),
        ]);
        assert_eq!(
            paint_with_white(&[line], true),
            "\u{1b}[41m  \u{1b}[0m\u{1b}[31m●\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_logo() {
        let red = DynColors::Ansi(AnsiColors::Red);
        let line = Line::from(vec![
            Span::new("", Style::Logo(red)),
            Span::new("██", Style::Logo(red)),
        ]);
        assert_eq!(
            paint_with_white(std::slice::from_ref(&line), true),
            "\u{1b}[31;1m\u{1b}[0m\u{1b}[31;1m██\u{1b}[0m\n"
        );
        assert_eq!(
            paint_with_white(&[line], false),
            "\u{1b}[31m\u{1b}[0m\u{1b}[31m██\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_paint_empty_line() {
        assert_eq!(paint_with_white(&[Line::default()], true), "\n");
    }

    #[test]
    fn test_paint_sanitizes_text() {
        // OSC set-title sequence
        let painted = paint_with_white(&[Line::from("1.0.0\u{1b}]0;PWNED\u{07}")], true);
        assert!(painted.contains("1.0.0\u{FFFD}]0;PWNED\u{FFFD}"));
        assert!(!painted.contains('\u{07}'));
    }

    #[test]
    fn test_write_unwrapped() {
        let mut output = Vec::new();
        write_unwrapped(&mut output, "text").unwrap();
        assert_eq!(output, b"\x1B[?7ltext\x1B[?7h");
    }
}
