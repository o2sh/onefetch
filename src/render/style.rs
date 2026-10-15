//! The lines onefetch prints, made of segments that each say which part of the
//! output they are (`Style`). `colors.rs` decides what each `Style` looks like.

use crate::language::Language;

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
    /// A language's chip in the languages field.
    Chip(Chip),
    /// A language's part of the languages bar.
    ChipBar(Chip),
    /// The palette row: the n-th of the 8 standard terminal colors.
    Palette(usize),
    Plain,
}

/// A language of the languages field, which its chip and its part of the bar
/// are colored after.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chip {
    /// `None` for "Other", the languages left out by `--number-of-languages`.
    pub language: Option<Language>,
    /// Its place in the languages field, from 0.
    pub rank: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub text: String,
    pub style: Style,
}

impl Segment {
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
pub struct Line(pub Vec<Segment>);

impl Line {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|segment| segment.text.is_empty())
    }

    pub fn width(&self) -> usize {
        self.0
            .iter()
            .map(|segment| segment.text.chars().count())
            .sum()
    }
}

impl From<Vec<Segment>> for Line {
    fn from(segments: Vec<Segment>) -> Self {
        Self(segments)
    }
}

impl From<String> for Line {
    fn from(text: String) -> Self {
        Self(vec![Segment::value(text)])
    }
}

impl From<&str> for Line {
    fn from(text: &str) -> Self {
        Self(vec![Segment::value(text)])
    }
}
