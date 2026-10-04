//! Text as lines of styled spans, kept apart from how it is displayed.
//!
//! A span's style is either a role from the text colors (title, info, ...) or an
//! explicit color. Turning styles into terminal escape sequences is the renderer's
//! job.

use owo_colors::DynColors;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    /// No styling, for spacing.
    Plain,
    /// A module's value, in the info color.
    Value,
    /// A module's key, in the subtitle color.
    Key,
    /// The separator between a key and its value, in the colon color.
    Separator,
    /// The title, in the title color.
    Title,
    /// Text in the tilde color.
    Tilde,
    /// Text in the underline color.
    Underline,
    /// Text in a specific color.
    Color(DynColors),
    /// Text on a specific background color.
    Background(DynColors),
}

/// A piece of text with a single style.
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

/// A line of text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line(pub Vec<Span>);

impl Line {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|span| span.text.is_empty())
    }

    /// The number of characters in the line.
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
