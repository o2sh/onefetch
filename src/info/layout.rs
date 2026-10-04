//! Lays out the info as lines: the title, the modules and the color palette.

use crate::info::title::Title;
use crate::info::utils::module::Module;
use crate::info::utils::text::{Line, Span, Style};
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

/// The title, underlined. Empty if the title is.
pub fn title_lines(title: &Title) -> Vec<Line> {
    let title = title.line();
    if title.is_empty() {
        return Vec::new();
    }
    let underline = Span::new("-".repeat(title.width()), Style::Underline);
    vec![title, Line::from(vec![underline])]
}

/// `key: value`, with the next lines of the value aligned under the first one.
/// Empty if the value is.
pub fn module_lines(module: &dyn Module) -> Vec<Line> {
    let value = module.value();
    if value.iter().all(Line::is_empty) {
        return Vec::new();
    }

    let key = module.key();
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

/// A blank line followed by the terminal's color palette.
pub fn palette_lines() -> Vec<Line> {
    let colors = PALETTE
        .into_iter()
        .map(|color| Span::new("   ", Style::Background(DynColors::Ansi(color))))
        .collect::<Vec<_>>();
    vec![Line::default(), Line::from(colors)]
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct ModuleImpl {
        #[serde(skip)]
        value: Vec<Line>,
    }

    #[typetag::serialize]
    impl Module for ModuleImpl {
        fn key(&self) -> String {
            "key".into()
        }

        fn value(&self) -> Vec<Line> {
            self.value.clone()
        }
    }

    #[test]
    fn test_module_lines() {
        let module = ModuleImpl {
            value: vec![Line::from("one"), Line::from("two")],
        };
        assert_eq!(
            module_lines(&module),
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
    fn test_module_lines_no_value() {
        let module = ModuleImpl { value: vec![] };
        assert!(module_lines(&module).is_empty());

        let module = ModuleImpl {
            value: vec![Line::from("")],
        };
        assert!(module_lines(&module).is_empty());
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

    #[test]
    fn test_palette_lines() {
        let lines = palette_lines();
        assert_eq!(lines[0], Line::default());
        assert_eq!(lines[1].0.len(), PALETTE.len());
    }
}
