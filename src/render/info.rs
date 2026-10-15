use crate::data::Info;
use crate::data::title::Title;
use crate::render::RenderOptions;
use crate::render::colors::COLOR_PALETTE;
use crate::render::fields::{FieldView, view};
use crate::render::style::{Line, Segment, Style};

pub fn info_lines(info: &Info, options: &RenderOptions) -> Vec<Line> {
    let mut lines = Vec::new();
    if let Some(title) = &info.title {
        lines.extend(title_lines(title));
    }
    for field in &info.info_fields {
        lines.extend(field_lines(view(field), options));
    }
    if !options.no_color_palette {
        lines.push(Line::default());
        lines.push(palette_line());
    }
    lines
}

fn title_lines(title: &Title) -> Vec<Line> {
    let title = title_line(title);
    if title.is_empty() {
        return Vec::new();
    }
    let underline = Segment::new("-".repeat(title.width()), Style::Underline);
    vec![title, Line::from(vec![underline])]
}

fn title_line(title: &Title) -> Line {
    let mut segments = Vec::new();
    for part in [&title.git_username, &title.git_version] {
        if part.is_empty() {
            continue;
        }
        if !segments.is_empty() {
            segments.push(Segment::plain(" "));
            segments.push(Segment::new("~", Style::Tilde));
            segments.push(Segment::plain(" "));
        }
        segments.push(Segment::new(part, Style::Title));
    }
    Line::from(segments)
}

fn field_lines(field: &dyn FieldView, options: &RenderOptions) -> Vec<Line> {
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
            let mut segments = if i == 0 {
                vec![
                    Segment::new(key.clone(), Style::Key),
                    Segment::new(":", Style::Separator),
                    Segment::plain(" "),
                ]
            } else {
                vec![Segment::plain(indent.clone())]
            };
            segments.extend(line.0);
            Line::from(segments)
        })
        .collect()
}

fn palette_line() -> Line {
    COLOR_PALETTE
        .into_iter()
        .map(|color| Segment::new("   ", Style::Background(color)))
        .collect::<Vec<_>>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestField {
        value: Vec<Line>,
    }

    impl FieldView for TestField {
        fn key(&self) -> String {
            "key".into()
        }

        fn value(&self, _options: &RenderOptions) -> Vec<Line> {
            self.value.clone()
        }
    }

    fn title(git_username: &str, git_version: &str) -> Title {
        Title {
            git_username: git_username.into(),
            git_version: git_version.into(),
        }
    }

    #[test]
    fn test_field_lines() {
        let field = TestField {
            value: vec![Line::from("one"), Line::from("two")],
        };
        assert_eq!(
            field_lines(&field, &RenderOptions::default()),
            vec![
                Line::from(vec![
                    Segment::new("key", Style::Key),
                    Segment::new(":", Style::Separator),
                    Segment::plain(" "),
                    Segment::value("one"),
                ]),
                Line::from(vec![Segment::plain("     "), Segment::value("two")]),
            ]
        );
    }

    #[test]
    fn test_field_lines_no_value() {
        let field = TestField { value: vec![] };
        assert!(field_lines(&field, &RenderOptions::default()).is_empty());

        let field = TestField {
            value: vec![Line::from("")],
        };
        assert!(field_lines(&field, &RenderOptions::default()).is_empty());
    }

    #[test]
    fn test_title_line() {
        let title = title("onefetch-committer-name", "git version 2.37.2");
        assert_eq!(
            title_line(&title),
            Line::from(vec![
                Segment::new("onefetch-committer-name", Style::Title),
                Segment::plain(" "),
                Segment::new("~", Style::Tilde),
                Segment::plain(" "),
                Segment::new("git version 2.37.2", Style::Title),
            ])
        );
    }

    #[test]
    fn test_title_line_without_git_version() {
        let title = title("onefetch-committer-name", "");
        assert_eq!(
            title_line(&title),
            Line::from(vec![Segment::new("onefetch-committer-name", Style::Title)])
        );
    }

    #[test]
    fn test_title_line_unknown() {
        assert!(title_line(&title("", "")).is_empty());
    }

    #[test]
    fn test_title_lines() {
        assert_eq!(
            title_lines(&title("user", "")),
            vec![
                Line::from(vec![Segment::new("user", Style::Title)]),
                Line::from(vec![Segment::new("----", Style::Underline)]),
            ]
        );
    }

    #[test]
    fn test_title_lines_unknown() {
        assert!(title_lines(&title("", "")).is_empty());
    }
}
