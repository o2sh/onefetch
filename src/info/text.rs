use owo_colors::DynColors;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    Plain,
    Value,
    Key,
    Separator,
    Title,
    Tilde,
    Underline,
    Color(DynColors),
    Background(DynColors),
}

/// Numbers are kept raw and only formatted, with the thousands separator, when
/// printed to the terminal.
#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Text(String),
    Number(u64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub content: Content,
    pub style: Style,
}

impl Span {
    pub fn new(text: impl Into<String>, style: Style) -> Self {
        Self {
            content: Content::Text(text.into()),
            style,
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self::new(text, Style::Plain)
    }

    pub fn value(text: impl Into<String>) -> Self {
        Self::new(text, Style::Value)
    }

    pub fn number(number: u64) -> Self {
        Self {
            content: Content::Number(number),
            style: Style::Value,
        }
    }

    fn width(&self) -> usize {
        match &self.content {
            Content::Text(text) => text.chars().count(),
            Content::Number(number) => number.to_string().len(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Line(pub Vec<Span>);

impl Line {
    pub fn is_empty(&self) -> bool {
        self.width() == 0
    }

    pub fn width(&self) -> usize {
        self.0.iter().map(Span::width).sum()
    }
}

impl From<Vec<Span>> for Line {
    fn from(spans: Vec<Span>) -> Self {
        Self(spans)
    }
}

impl From<Span> for Line {
    fn from(span: Span) -> Self {
        Self(vec![span])
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
