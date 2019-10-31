use colored::{Color, Colorize};

pub struct AsciiArt<'a> {
    content: Box<dyn 'a + Iterator<Item = &'a str>>,
    colors: Vec<Color>,
    bold: bool,
    start: usize,
    end: usize,
}
impl<'a> AsciiArt<'a> {
    pub fn new(input: &'a str, colors: Vec<Color>, bold: bool) -> AsciiArt<'a> {
        let mut lines: Vec<_> = input.lines().skip_while(|line| line.is_empty()).collect();
        while let Some(line) = lines.last() {
            if Tokens(line).is_empty() {
                lines.pop();
            } else {
                break;
            }
        }

        let (start, end) = lines
            .iter()
            .map(|line| {
                let line_start = Tokens(line).leading_spaces();
                let line_end = Tokens(line).true_length();
                (line_start, line_end)
            })
            .fold((std::usize::MAX, 0), |(acc_s, acc_e), (line_s, line_e)| {
                (acc_s.min(line_s), acc_e.max(line_e))
            });

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

/// Produces a series of lines which have been automatically truncated to the
/// correct width
impl<'a> Iterator for AsciiArt<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.content
            .next()
            .map(|line| Tokens(line).render(&self.colors, self.start, self.end, self.bold))
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
            Token::Color(c) => write!(f, "{{{}}}", c),
            Token::Char(c) => write!(f, "{}", c),
            Token::Space => write!(f, " "),
        }
    }
}
impl Token {
    fn is_solid(&self) -> bool {
        match *self {
            Token::Char(_) => true,
            _ => false,
        }
    }
    fn is_space(&self) -> bool {
        match *self {
            Token::Space => true,
            _ => false,
        }
    }
    fn has_zero_width(&self) -> bool {
        match *self {
            Token::Color(_) => true,
            _ => false,
        }
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
            .filter(|token| token.is_space())
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
    fn render(self, colors: &[Color], start: usize, end: usize, bold: bool) -> String {
        assert!(start <= end);
        let mut width = end - start;
        let mut colored_segment = String::new();
        let mut whole_string = String::new();
        let mut color = &Color::White;

        self.truncate(start, end).for_each(|token| {
            match token {
                Token::Char(chr) => {
                    width = width.saturating_sub(1);
                    colored_segment.push(chr);
                }
                Token::Color(col) => {
                    add_colored_segment(&mut whole_string, &colored_segment, *color, bold);
                    colored_segment = String::new();
                    color = colors.get(col as usize).unwrap_or(&Color::White);
                }
                Token::Space => {
                    width = width.saturating_sub(1);
                    colored_segment.push(' ')
                }
            };
        });

        add_colored_segment(&mut whole_string, &colored_segment, *color, bold);
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

fn add_colored_segment(base: &mut String, segment: &str, color: Color, bold: bool) {
    let mut colored_segment = segment.color(color);
    if bold {
        colored_segment = colored_segment.bold();
    }
    base.push_str(&format!("{}", colored_segment));
}

// Basic combinators

type ParseResult<'a, R> = Option<(&'a str, R)>;

fn token<R>(s: &str, predicate: impl FnOnce(char) -> Option<R>) -> ParseResult<R> {
    let token = s.chars().next()?;
    let result = predicate(token)?;
    Some((s.get(1..).unwrap(), result))
}

// Parsers

/// Parses a color indiator of the format `{n}` where `n` is a digit.
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
            "\u{1b}[1;37m\u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 0, true),
            "\u{1b}[1;37m\u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 5, true),
            "\u{1b}[1;37m     \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 1, 5, true),
            "\u{1b}[1;37m    \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 3, 5, true),
            "\u{1b}[1;37m  \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 4, true),
            "\u{1b}[1;37m    \u{1b}[0m"
        );

        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 3, true),
            "\u{1b}[1;37m   \u{1b}[0m"
        );

        assert_eq!(
            Tokens("  {1} {5}  {9} a").render(&colors_shim, 4, 10, true),
            "\u{1b}[1;37m\u{1b}[0m\u{1b}[1;37m\u{1b}[0m\u{1b}[1;37m \u{1b}[0m\u{1b}[1;37m a\u{1b}[0m   "
        );

        // Tests for bold disabled
        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 0, false),
            "\u{1b}[37m\u{1b}[0m"
        );
        assert_eq!(
            Tokens("     ").render(&colors_shim, 0, 5, false),
            "\u{1b}[37m     \u{1b}[0m"
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
