//! The lines onefetch prints, made of spans that each say which part of the
//! output they are (`Style`). `colors.rs` decides what each `Style` looks like.

use owo_colors::DynColors;

/// Everything that can be colored.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    // The parts `--text-colors` colors, in its order.
    Title,
    Tilde,
    Underline,
    Key,
    Separator,
    Value,
    /// The ascii art after a `{n}` token, which `--ascii-colors` colors.
    /// `None` before the first token of a line.
    Ascii(Option<usize>),
    /// A fixed color: the language chips.
    Color(DynColors),
    /// A fixed background color: the language bar and the palette row.
    Background(DynColors),
    Plain,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub text: String,
    pub style: Style,
}

impl Span {
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self::new(text, Style::Plain)
    }

    pub fn value(text: impl Into<String>) -> Self {
        Self::new(text, Style::Value)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line(pub Vec<Span>);

impl Line {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|span| span.text.is_empty())
    }

    pub fn width(&self) -> usize {
        self.0.iter().map(|span| span.text.chars().count()).sum()
    }
}

impl From<Vec<Span>> for Line {
    fn from(spans: Vec<Span>) -> Self {
        Self(spans)
    }
}

impl From<String> for Line {
    fn from(text: String) -> Self {
        Self(vec![Span::value(text)])
    }
}

impl From<&str> for Line {
    fn from(text: &str) -> Self {
        Self(vec![Span::value(text)])
    }
}
