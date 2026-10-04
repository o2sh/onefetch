//! Turns lines of styled text into text with ANSI escape sequences.
//!
//! This is the only place where text is styled, and all text goes through
//! `paint`, which sanitizes it first. Data read from a repository therefore can't
//! inject terminal escape sequences.

use crate::info::utils::get_style;
use crate::info::utils::text::{Line, Style};
use crate::ui::text_colors::TextColors;
use owo_colors::{OwoColorize, Style as AnsiStyle};

pub fn render(lines: &[Line], text_colors: &TextColors, no_bold: bool) -> String {
    let mut output = String::new();
    for line in lines {
        for span in &line.0 {
            let style = ansi_style(span.style, text_colors, !no_bold);
            output.push_str(&paint(&span.text, style));
        }
        output.push('\n');
    }
    output
}

fn ansi_style(style: Style, colors: &TextColors, bold: bool) -> AnsiStyle {
    match style {
        Style::Plain => AnsiStyle::new(),
        Style::Value => get_style(false, colors.value),
        Style::Key => get_style(bold, colors.key),
        Style::Separator => get_style(bold, colors.separator),
        Style::Title => get_style(bold, colors.title),
        Style::Tilde => get_style(bold, colors.tilde),
        Style::Underline => get_style(false, colors.underline),
        Style::Color(color) => AnsiStyle::new().color(color),
        Style::Background(color) => AnsiStyle::new().on_color(color),
    }
}

fn paint(text: &str, style: AnsiStyle) -> String {
    sanitize(text).style(style).to_string()
}

/// Replaces control characters so untrusted repository data can't inject
/// terminal escape sequences.
fn sanitize(text: &str) -> String {
    text.replace(char::is_control, "\u{FFFD}")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::utils::text::Span;
    use owo_colors::{AnsiColors, DynColors};

    fn render_with_white(lines: &[Line], no_bold: bool) -> String {
        let text_colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        render(lines, &text_colors, no_bold)
    }

    #[test]
    fn test_render() {
        let line = Line::from(vec![
            Span::new("title", Style::Key),
            Span::new(":", Style::Separator),
            Span::plain(" "),
            Span::value("test"),
        ]);
        assert_eq!(
            render_with_white(&[line], false),
            "\u{1b}[38;2;255;255;255;1mtitle\u{1b}[0m\u{1b}[39;1m:\u{1b}[0m \u{1b}[39mtest\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_render_no_bold() {
        let line = Line::from(vec![Span::new("title", Style::Key)]);
        assert_eq!(
            render_with_white(&[line], true),
            "\u{1b}[38;2;255;255;255mtitle\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_render_explicit_colors() {
        let red = DynColors::Ansi(AnsiColors::Red);
        let line = Line::from(vec![
            Span::new("  ", Style::Background(red)),
            Span::new("●", Style::Color(red)),
        ]);
        let rendered = render_with_white(&[line], false);
        assert!(rendered.contains(&paint("  ", AnsiStyle::new().on_color(red))));
        assert!(rendered.contains(&paint("●", AnsiStyle::new().color(red))));
    }

    #[test]
    fn test_render_empty_line() {
        assert_eq!(render_with_white(&[Line::default()], false), "\n");
    }

    #[test]
    fn test_render_sanitizes_text() {
        // OSC set-title sequence
        let rendered = render_with_white(&[Line::from("1.0.0\u{1b}]0;PWNED\u{07}")], false);
        assert!(rendered.contains("1.0.0\u{FFFD}]0;PWNED\u{FFFD}"));
        assert!(!rendered.contains('\u{07}'));
    }
}
