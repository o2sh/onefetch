use crate::cli::NumberSeparator;
use crate::info::text::{Content, Line, Style};
use crate::ui::text_colors::TextColors;
use num_format::ToFormattedString;
use owo_colors::{DynColors, OwoColorize, Style as AnsiStyle};

pub fn render(
    lines: &[Line],
    text_colors: &TextColors,
    no_bold: bool,
    number_separator: NumberSeparator,
) -> String {
    let mut output = String::new();
    for line in lines {
        for span in &line.0 {
            let text = match &span.content {
                Content::Text(text) => sanitize(text),
                Content::Number(number) => format_number(*number, number_separator),
            };
            let style = ansi_style(span.style, text_colors, !no_bold);
            output.push_str(&text.style(style).to_string());
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

fn get_style(is_bold: bool, color: DynColors) -> AnsiStyle {
    let mut style = AnsiStyle::new().color(color);
    if is_bold {
        style = style.bold();
    }
    style
}

fn format_number(number: u64, number_separator: NumberSeparator) -> String {
    number.to_formatted_string(&number_separator.get_format())
}

/// Replaces control characters so untrusted repository data can't inject
/// terminal escape sequences.
fn sanitize(text: &str) -> String {
    text.replace(char::is_control, "\u{FFFD}")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::info::text::Span;
    use owo_colors::AnsiColors;
    use rstest::rstest;

    fn render_with_white(lines: &[Line], no_bold: bool) -> String {
        let text_colors = TextColors::new(&[], DynColors::Rgb(0xFF, 0xFF, 0xFF));
        render(lines, &text_colors, no_bold, NumberSeparator::Plain)
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
        assert_eq!(
            render_with_white(&[line], false),
            "\u{1b}[41m  \u{1b}[0m\u{1b}[31m●\u{1b}[0m\n"
        );
    }

    #[test]
    fn test_render_number() {
        let text_colors = TextColors::default();
        let line = Line::from(vec![Span::number(1_234_567)]);
        assert_eq!(
            render(&[line], &text_colors, false, NumberSeparator::Comma),
            "\u{1b}[39m1,234,567\u{1b}[0m\n"
        );
    }

    #[rstest]
    #[case(1_000_000, NumberSeparator::Comma, "1,000,000")]
    #[case(1_000_000, NumberSeparator::Space, "1\u{202f}000\u{202f}000")]
    #[case(1_000_000, NumberSeparator::Underscore, "1_000_000")]
    #[case(1_000_000, NumberSeparator::Plain, "1000000")]
    fn test_format_number(
        #[case] number: u64,
        #[case] number_separator: NumberSeparator,
        #[case] expected: &str,
    ) {
        assert_eq!(format_number(number, number_separator), expected);
    }

    #[test]
    fn test_render_empty_line() {
        assert_eq!(render_with_white(&[Line::default()], false), "\n");
    }

    #[test]
    fn test_get_style() {
        let style = get_style(true, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(
            style,
            AnsiStyle::new()
                .color(DynColors::Ansi(AnsiColors::Cyan))
                .bold()
        );
    }

    #[test]
    fn test_get_style_no_bold() {
        let style = get_style(false, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(
            style,
            AnsiStyle::new().color(DynColors::Ansi(AnsiColors::Cyan))
        );
    }

    #[test]
    fn test_render_sanitizes_text() {
        // OSC set-title sequence
        let rendered = render_with_white(&[Line::from("1.0.0\u{1b}]0;PWNED\u{07}")], false);
        assert!(rendered.contains("1.0.0\u{FFFD}]0;PWNED\u{FFFD}"));
        assert!(!rendered.contains('\u{07}'));
    }
}
