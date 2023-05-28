//! # onefetch-ascii
//!
//! Provides the ascii template interface for [onefetch](https://github.com/o2sh/onefetch).
//!
//! ```rust,no_run
//! use onefetch_ascii::AsciiArt;
//! use owo_colors::{DynColors, AnsiColors};
//!
//! const ASCII: &str = r#"
//! {2}            .:--::////::--.`
//! {1}        `/yNMMNho{2}////////////:.
//! {1}      `+NMMMMMMMMmy{2}/////////////:`
//! {0}    `-:::{1}ohNMMMMMMMNy{2}/////////////:`
//! {0}   .::::::::{1}odMMMMMMMNy{2}/////////////-
//! {0}  -:::::::::::{1}/hMMMMMMMmo{2}////////////-
//! {0} .::::::::::::::{1}oMMMMMMMMh{2}////////////-
//! {0}`:::::::::::::{1}/dMMMMMMMMMMNo{2}///////////`
//! {0}-::::::::::::{1}sMMMMMMmMMMMMMMy{2}//////////-
//! {0}-::::::::::{1}/dMMMMMMs{0}:{1}+NMMMMMMd{2}/////////:
//! {0}-:::::::::{1}+NMMMMMm/{0}:::{1}/dMMMMMMm+{2}///////:
//! {0}-::::::::{1}sMMMMMMh{0}:::::::{1}dMMMMMMm+{2}//////-
//! {0}`:::::::{1}sMMMMMMy{0}:::::::::{1}dMMMMMMm+{2}/////`
//! {0} .:::::{1}sMMMMMMs{0}:::::::::::{1}mMMMMMMd{2}////-
//! {0}  -:::{1}sMMMMMMy{0}::::::::::::{1}/NMMMMMMh{2}//-
//! {0}   .:{1}+MMMMMMd{0}::::::::::::::{1}oMMMMMMMo{2}-
//! {1}    `yMMMMMN/{0}:::::::::::::::{1}hMMMMMh.
//! {1}      -yMMMo{0}::::::::::::::::{1}/MMMy-
//! {1}        `/s{0}::::::::::::::::::{1}o/`
//! {0}            ``.---::::---..`
//! "#;
//!
//! let colors = vec![
//!     DynColors::Ansi(AnsiColors::Blue),
//!     DynColors::Ansi(AnsiColors::Default),
//!     DynColors::Ansi(AnsiColors::BrightBlue)
//! ];
//!
//! let art = AsciiArt::new(ASCII, colors.as_slice(), true);
//!
//! for line in art {
//!     println!("{line}")
//! }
//! ```
//!

use owo_colors::{AnsiColors, DynColors, OwoColorize, Style};
use std::fmt::Write;

/// Renders an ascii template with the given colors truncated to the correct width.
pub struct AsciiArt<'a> {
    content: Box<dyn 'a + Iterator<Item = &'a str>>,
    colors: &'a [DynColors],
    bold: bool,
    start: usize,
    end: usize,
}
impl<'a> AsciiArt<'a> {
    pub fn new(input: &'a str, colors: &'a [DynColors], bold: bool) -> AsciiArt<'a> {
        let mut lines: Vec<_> = input.lines().skip_while(|line| line.is_empty()).collect();
        while let Some(line) = lines.last() {
            if Tokens(line).is_empty() {
                lines.pop();
            } else {
                break;
            }
        }

        let (start, end) = get_min_start_max_end(&lines);

        AsciiArt {
            content: Box::new(lines.into_iter()),
            colors,
            bold,
            start,
            end,
        }
    }

    pub fn width(&self) -> usize {
        assert!(self.end >= self.start);
        self.end - self.start
    }
}

fn get_min_start_max_end(lines: &[&str]) -> (usize, usize) {
    lines
        .iter()
        .map(|line| {
            let line_start = Tokens(line).leading_spaces();
            let line_end = Tokens(line).true_length();
            (line_start, line_end)
        })
        .fold((std::usize::MAX, 0), |(acc_s, acc_e), (line_s, line_e)| {
            (acc_s.min(line_s), acc_e.max(line_e))
        })
}

/// Produces a series of lines which have been automatically truncated to the
/// correct width
impl<'a> Iterator for AsciiArt<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.content
            .next()
            .map(|line| Tokens(line).render(self.colors, self.start, self.end, self.bold))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Color(u32),
    Char(char),
    Space,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Color(c) => write!(f, "{{{c}}}"),
            Token::Char(c) => write!(f, "{c}"),
            Token::Space => write!(f, " "),
        }
    }
}
impl Token {
    fn is_solid(&self) -> bool {
        matches!(*self, Token::Char(_))
    }
    fn is_space(&self) -> bool {
        matches!(*self, Token::Space)
    }
    fn has_zero_width(&self) -> bool {
        matches!(*self, Token::Color(_))
    }
}

/// An iterator over tokens found within the *.ascii format.
#[derive(Clone, Debug)]
struct Tokens<'a>(&'a str);
impl<'a> Iterator for Tokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let (s, tok) = color_token(self.0)
            .or_else(|| space_token(self.0))
            .or_else(|| char_token(self.0))?;

        self.0 = s;
        Some(tok)
    }
}

impl<'a> Tokens<'a> {
    fn is_empty(&mut self) -> bool {
        for token in self {
            if token.is_solid() {
                return false;
            }
        }
        true
    }
    fn true_length(&mut self) -> usize {
        let mut last_non_space = 0;
        let mut last = 0;
        for token in self {
            if token.has_zero_width() {
                continue;
            }
            last += 1;
            if !token.is_space() {
                last_non_space = last;
            }
        }
        last_non_space
    }
    fn leading_spaces(&mut self) -> usize {
        self.take_while(|token| !token.is_solid())
            .filter(Token::is_space)
            .count()
    }
    fn truncate(self, mut start: usize, end: usize) -> impl 'a + Iterator<Item = Token> {
        assert!(start <= end);
        let mut width = end - start;

        self.filter(move |token| {
            if start > 0 && !token.has_zero_width() {
                start -= 1;
                return false;
            }
            true
        })
        .take_while(move |token| {
            if width == 0 {
                return false;
            }
            if !token.has_zero_width() {
                width -= 1;
            }
            true
        })
    }
    /// render a truncated line of tokens.
    fn render(self, colors: &[DynColors], start: usize, end: usize, bold: bool) -> String {
        assert!(start <= end);
        let mut width = end - start;
        let mut colored_segment = String::new();
        let mut whole_string = String::new();
        let mut color = &DynColors::Ansi(AnsiColors::Default);

        self.truncate(start, end).for_each(|token| {
            match token {
                Token::Char(chr) => {
                    width = width.saturating_sub(1);
                    colored_segment.push(chr);
                }
                Token::Color(col) => {
                    add_styled_segment(&mut whole_string, &colored_segment, *color, bold);
                    colored_segment = String::new();
                    color = colors
                        .get(col as usize)
                        .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
                }
                Token::Space => {
                    width = width.saturating_sub(1);
                    colored_segment.push(' ')
                }
            };
        });

        add_styled_segment(&mut whole_string, &colored_segment, *color, bold);
        (0..width).for_each(|_| whole_string.push(' '));
        whole_string
    }
}

// Utility functions

fn succeed_when<I>(predicate: impl FnOnce(I) -> bool) -> impl FnOnce(I) -> Option<()> {
    |input| {
        if predicate(input) {
            Some(())
        } else {
            None
        }
    }
}

fn add_styled_segment(base: &mut String, segment: &str, color: DynColors, bold: bool) {
    let mut style = Style::new().color(color);
    if bold {
        style = style.bold();
    }
    let formatted_segment = segment.style(style);
    let _ = write!(base, "{formatted_segment}");
}

// Basic combinators

type ParseResult<'a, R> = Option<(&'a str, R)>;

fn token<R>(s: &str, predicate: impl FnOnce(char) -> Option<R>) -> ParseResult<R> {
    let mut chars = s.chars();
    let token = chars.next()?;
    let result = predicate(token)?;
    Some((chars.as_str(), result))
}

// Parsers

/// Parses a color indicator of the format `{n}` where `n` is a digit.
fn color_token(s: &str) -> ParseResult<Token> {
    let (s, _) = token(s, succeed_when(|c| c == '{'))?;
    let (s, color_index) = token(s, |c| c.to_digit(10))?;
    let (s, _) = token(s, succeed_when(|c| c == '}'))?;
    Some((s, Token::Color(color_index)))
}

/// Parses a space.
fn space_token(s: &str) -> ParseResult<Token> {
    token(s, succeed_when(|c| c == ' ')).map(|(s, _)| (s, Token::Space))
}

/// Parses any arbitrary character. This cannot fail.
fn char_token(s: &str) -> ParseResult<Token> {
    token(s, |c| Some(Token::Char(c)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_min_start_max_end() {
        let lines = [
            "                     xxx",
            "   xxx",
            "         oo",
            "     o",
            "                           xx",
        ];
        assert_eq!(get_min_start_max_end(&lines), (3, 29));
    }

    #[test]
    fn space_parses() {
        assert_eq!(space_token(" "), Some(("", Token::Space)));
        assert_eq!(space_token(" hello"), Some(("hello", Token::Space)));
        assert_eq!(space_token("      "), Some(("     ", Token::Space)));
        assert_eq!(space_token(" {1}{2}"), Some(("{1}{2}", Token::Space)));
    }

    #[test]
    fn color_indicator_parses() {
        assert_eq!(color_token("{1}"), Some(("", Token::Color(1))));
        assert_eq!(color_token("{9} "), Some((" ", Token::Color(9))));
    }

    #[test]
    fn leading_spaces_counts_correctly() {
        assert_eq!(Tokens("").leading_spaces(), 0);
        assert_eq!(Tokens("     ").leading_spaces(), 5);
        assert_eq!(Tokens("     a;lksjf;a").leading_spaces(), 5);
        assert_eq!(Tokens("  {1} {5}  {9} a").leading_spaces(), 6);
    }

    #[test]
    fn render() {
        let colors_shim = Vec::new();

        assert_eq!(
            Tokens("").render(&colors_shim, 0, 0, true),
            "\u{1b}[39;1m\u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 0, true),
            "\u{1b}[39;1m\u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 5, true),
            "\u{1b}[39;1m     \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 1, 5, true),
            "\u{1b}[39;1m    \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 3, 5, true),
            "\u{1b}[39;1m  \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 4, true),
            "\u{1b}[39;1m    \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 3, true),
            "\u{1b}[39;1m   \u{1b}[0m"
        );

        // https://github.com/o2sh/onefetch/issues/935
        assert_eq!(
            Tokens("███").render(Vec::new().as_slice(), 0, 3, true),
            "\u{1b}[39;1m███\u{1b}[0m"
        );

        assert_eq!(
            Tokens("  {1} {5}  {9} a").render(&colors_shim, 4, 10, true),
            "\u{1b}[39;1m\u{1b}[0m\u{1b}[39;1m\u{1b}[0m\u{1b}[39;1m \u{1b}[0m\u{1b}[39;1m a\u{1b}[0m   "
        );

        // Tests for bold disabled
        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 0, false),
            "\u{1b}[39m\u{1b}[0m"
        );
        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 5, false),
            "\u{1b}[39m     \u{1b}[0m"
        );
    }

    #[test]
    fn truncate() {
        assert_eq!(
            Tokens("").truncate(0, 0).collect::<Vec<_>>(),
            Tokens("").collect::<Vec<_>>()
        );

        assert_eq!(
            Tokens("     ").truncate(0, 0).collect::<Vec<_>>(),
            Tokens("").collect::<Vec<_>>()
        );
        assert_eq!(
            Tokens("     ").truncate(0, 5).collect::<Vec<_>>(),
            Tokens("     ").collect::<Vec<_>>()
        );
        assert_eq!(
            Tokens("     ").truncate(1, 5).collect::<Vec<_>>(),
            Tokens("    ").collect::<Vec<_>>()
        );
        assert_eq!(
            Tokens("     ").truncate(3, 5).collect::<Vec<_>>(),
            Tokens("  ").collect::<Vec<_>>()
        );
        assert_eq!(
            Tokens("     ").truncate(0, 4).collect::<Vec<_>>(),
            Tokens("    ").collect::<Vec<_>>()
        );
        assert_eq!(
            Tokens("     ").truncate(0, 3).collect::<Vec<_>>(),
            Tokens("   ").collect::<Vec<_>>()
        );

        assert_eq!(
            Tokens("  {1} {5}  {9} a")
                .truncate(4, 10)
                .collect::<Vec<_>>(),
            Tokens("{1}{5} {9} a").collect::<Vec<_>>()
        );
    }
}
