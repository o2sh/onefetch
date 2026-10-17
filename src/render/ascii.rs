//! The ascii art.
//!
//! Turns an ascii template into lines: the text after a `{n}` token is styled
//! `Style::AsciiArt(Some(n))`, which `--ascii-colors` colors.
//!
//! ```rust,no_run
//! use onefetch::render::ascii::ascii_lines;
//! use onefetch::render::style::Line;
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
//! let lines: Vec<Line> = ascii_lines(ASCII);
//! ```
//!

use super::style::{Line, Segment, Style};

/// The lines of an ascii template, without the common indentation and all
/// padded to the same width.
pub fn ascii_lines(input: &str) -> Vec<Line> {
    let mut lines: Vec<_> = input.lines().skip_while(|line| line.is_empty()).collect();
    while lines.last().is_some_and(|line| Tokens(line).is_empty()) {
        lines.pop();
    }
    let Some(start) = lines.iter().map(|line| Tokens(line).leading_spaces()).min() else {
        return Vec::new();
    };
    let end = lines
        .iter()
        .map(|line| Tokens(line).true_length())
        .max()
        .unwrap_or(start);
    lines
        .iter()
        .map(|line| Tokens(line).render(start, end))
        .collect()
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
impl Iterator for Tokens<'_> {
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
    fn render(self, start: usize, end: usize) -> Line {
        assert!(start <= end);
        let mut width = end - start;
        let mut text = String::new();
        let mut segments = Vec::new();
        let mut color = None;

        let mut push_segment = |text: &mut String, color| {
            if !text.is_empty() {
                segments.push(Segment::new(std::mem::take(text), Style::AsciiArt(color)));
            }
        };

        for token in self.truncate(start, end) {
            match token {
                Token::Char(chr) => {
                    width = width.saturating_sub(1);
                    text.push(chr);
                }
                Token::Color(col) => {
                    push_segment(&mut text, color);
                    color = Some(col as usize);
                }
                Token::Space => {
                    width = width.saturating_sub(1);
                    text.push(' ');
                }
            }
        }

        push_segment(&mut text, color);
        if width > 0 {
            segments.push(Segment::plain(" ".repeat(width)));
        }
        Line::from(segments)
    }
}

// Utility functions

fn succeed_when<I>(predicate: impl FnOnce(I) -> bool) -> impl FnOnce(I) -> Option<()> {
    |input| {
        if predicate(input) { Some(()) } else { None }
    }
}

// Basic combinators

type ParseResult<'a, R> = Option<(&'a str, R)>;

fn token<'a, R>(s: &'a str, predicate: impl FnOnce(char) -> Option<R>) -> ParseResult<'a, R> {
    let mut chars = s.chars();
    let token = chars.next()?;
    let result = predicate(token)?;
    Some((chars.as_str(), result))
}

// Parsers

/// Parses a color indicator of the format `{n}` where `n` is a digit.
fn color_token<'a>(s: &'a str) -> ParseResult<'a, Token> {
    let (s, ()) = token(s, succeed_when(|c| c == '{'))?;
    let (s, color_index) = token(s, |c| c.to_digit(10))?;
    let (s, ()) = token(s, succeed_when(|c| c == '}'))?;
    Some((s, Token::Color(color_index)))
}

/// Parses a space.
fn space_token<'a>(s: &'a str) -> ParseResult<'a, Token> {
    token(s, succeed_when(|c| c == ' ')).map(|(s, ())| (s, Token::Space))
}

/// Parses any arbitrary character. This cannot fail.
fn char_token<'a>(s: &'a str) -> ParseResult<'a, Token> {
    token(s, |c| Some(Token::Char(c)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ascii_lines_removes_common_indentation() {
        let input = [
            "                     xxx",
            "   xxx",
            "         oo",
            "     o",
            "                           xx",
        ]
        .join("\n");
        let lines = ascii_lines(&input);
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|line| line.width() == 26));
    }

    #[test]
    fn ascii_lines_without_visible_characters_is_empty() {
        for input in ["", "   ", "\n\n", "{0}"] {
            assert_eq!(ascii_lines(input), Vec::new());
        }
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

    fn ascii(text: &str, color: Option<usize>) -> Segment {
        Segment::new(text, Style::AsciiArt(color))
    }

    #[test]
    fn render() {
        assert_eq!(Tokens("").render(0, 0), Line::default());
        assert_eq!(Tokens("     ").render(0, 0), Line::default());
        assert_eq!(
            Tokens("     ").render(0, 5),
            Line::from(vec![ascii("     ", None)])
        );
        assert_eq!(
            Tokens("     ").render(1, 5),
            Line::from(vec![ascii("    ", None)])
        );
        assert_eq!(
            Tokens("     ").render(3, 5),
            Line::from(vec![ascii("  ", None)])
        );
        assert_eq!(
            Tokens("     ").render(0, 4),
            Line::from(vec![ascii("    ", None)])
        );
        assert_eq!(
            Tokens("     ").render(0, 3),
            Line::from(vec![ascii("   ", None)])
        );

        // https://github.com/o2sh/onefetch/issues/935
        assert_eq!(
            Tokens("███").render(0, 3),
            Line::from(vec![ascii("███", None)])
        );

        assert_eq!(
            Tokens("  {1} {5}  {9} a").render(4, 10),
            Line::from(vec![
                ascii(" ", Some(5)),
                ascii(" a", Some(9)),
                Segment::plain("   "),
            ])
        );
    }

    #[test]
    fn render_colors() {
        assert_eq!(
            Tokens("{0}ab{1}cd").render(0, 4),
            Line::from(vec![ascii("ab", Some(0)), ascii("cd", Some(1))])
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
