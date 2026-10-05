use crate::info::display_options::DisplayOptions;
use crate::info::info_field::InfoField;
use crate::info::text::{Line, Span, Style};
use crate::info::title::Title;
use owo_colors::{AnsiColors, DynColors};

const PALETTE: [AnsiColors; 8] = [
    AnsiColors::Black,
    AnsiColors::Red,
    AnsiColors::Green,
    AnsiColors::Yellow,
    AnsiColors::Blue,
    AnsiColors::Magenta,
    AnsiColors::Cyan,
    AnsiColors::White,
];

pub fn title_lines(title: &Title) -> Vec<Line> {
    let title = title.line();
    if title.is_empty() {
        return Vec::new();
    }
    let underline = Span::new("-".repeat(title.width()), Style::Underline);
    vec![title, Line::from(vec![underline])]
}

pub fn field_lines(field: &dyn InfoField, options: &DisplayOptions) -> Vec<Line> {
    let value = field.value(options);
    if value.iter().all(Line::is_empty) {
        return Vec::new();
    }

    let key = field.key();
    let indent = " ".repeat(key.chars().count() + 2);

    value
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let mut spans = if i == 0 {
                vec![
                    Span::new(key.clone(), Style::Key),
                    Span::new(":", Style::Separator),
                    Span::plain(" "),
                ]
            } else {
                vec![Span::plain(indent.clone())]
            };
            spans.extend(line.0);
            Line::from(spans)
        })
        .collect()
}

pub fn palette_line() -> Line {
    PALETTE
        .into_iter()
        .map(|color| Span::new("   ", Style::Background(DynColors::Ansi(color))))
        .collect::<Vec<_>>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct InfoFieldImpl {
        #[serde(skip)]
        value: Vec<Line>,
    }

    #[typetag::serialize]
    impl InfoField for InfoFieldImpl {
        fn key(&self) -> String {
            "key".into()
        }

        fn value(&self, _options: &DisplayOptions) -> Vec<Line> {
            self.value.clone()
        }
    }

    #[test]
    fn test_field_lines() {
        let field = InfoFieldImpl {
            value: vec![Line::from("one"), Line::from("two")],
        };
        assert_eq!(
            field_lines(&field, &DisplayOptions::default()),
            vec![
                Line::from(vec![
                    Span::new("key", Style::Key),
                    Span::new(":", Style::Separator),
                    Span::plain(" "),
                    Span::value("one"),
                ]),
                Line::from(vec![Span::plain("     "), Span::value("two")]),
            ]
        );
    }

    #[test]
    fn test_field_lines_no_value() {
        let field = InfoFieldImpl { value: vec![] };
        assert!(field_lines(&field, &DisplayOptions::default()).is_empty());

        let field = InfoFieldImpl {
            value: vec![Line::from("")],
        };
        assert!(field_lines(&field, &DisplayOptions::default()).is_empty());
    }

    #[test]
    fn test_title_lines() {
        let title = Title {
            git_username: "user".into(),
            git_version: String::new(),
        };
        assert_eq!(
            title_lines(&title),
            vec![
                Line::from(vec![Span::new("user", Style::Title)]),
                Line::from(vec![Span::new("----", Style::Underline)]),
            ]
        );
    }

    #[test]
    fn test_title_lines_unknown() {
        let title = Title {
            git_username: String::new(),
            git_version: String::new(),
        };
        assert!(title_lines(&title).is_empty());
    }
}
