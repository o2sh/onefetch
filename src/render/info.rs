use crate::data::title::Title;
use crate::data::{FieldKind, Info};
use crate::render::RenderOptions;
use crate::render::fields::{FieldView, view};
use crate::render::style::{Line, Segment, Style};

pub fn info_lines(info: &Info, options: &RenderOptions) -> Vec<Line> {
    let mut lines = Vec::new();
    if let Some(title) = &info.title {
        lines.extend(title_lines(title));
    }
    for field in &info.info_fields {
        lines.extend(field_lines(field.kind(), view(field), options));
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

fn field_lines(kind: FieldKind, field: &dyn FieldView, options: &RenderOptions) -> Vec<Line> {
    let value = field.value(options);
    if value.iter().all(Line::is_empty) {
        return Vec::new();
    }

    let layout = &options.key_layout;
    let field_style = layout.field_style(kind);
    let key = field_style
        .and_then(|style| style.key.clone())
        .unwrap_or_else(|| field.key());
    let key_style = field_style
        .and_then(|style| style.key_color)
        .map_or(Style::Key, Style::KeyColor);
    let value_style = field_style
        .and_then(|style| style.output_color)
        .map_or(Style::Value, Style::ValueColor);

    let mut prefix = Vec::new();
    if layout.padding_left > 0 {
        prefix.push(Segment::plain(" ".repeat(layout.padding_left)));
    }
    if !key.is_empty() {
        prefix.push(Segment::new(key, key_style));
    }
    // Only the visible part of the separator takes its color.
    let separator = layout.separator.trim_end();
    let spacing = &layout.separator[separator.len()..];
    if !separator.is_empty() {
        prefix.push(Segment::new(separator, Style::Separator));
    }
    if !spacing.is_empty() {
        prefix.push(Segment::plain(spacing));
    }
    let width = Line::from(prefix.clone()).width();
    if layout.width > width {
        prefix.push(Segment::plain(" ".repeat(layout.width - width)));
    }
    let indent = " ".repeat(width.max(layout.width));

    value
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let mut segments = if i == 0 {
                prefix.clone()
            } else {
                vec![Segment::plain(indent.clone())]
            };
            segments.extend(line.0.into_iter().map(|mut segment| {
                if segment.style == Style::Value {
                    segment.style = value_style;
                }
                segment
            }));
            Line::from(segments)
        })
        .collect()
}

/// The 8 standard terminal colors.
fn palette_line() -> Line {
    (0..8)
        .map(|index| Segment::new("   ", Style::Palette(index)))
        .collect::<Vec<_>>()
        .into()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::render::{FieldStyle, KeyLayout};

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
            field_lines(FieldKind::Head, &field, &RenderOptions::default()),
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
        assert!(field_lines(FieldKind::Head, &field, &RenderOptions::default()).is_empty());

        let field = TestField {
            value: vec![Line::from("")],
        };
        assert!(field_lines(FieldKind::Head, &field, &RenderOptions::default()).is_empty());
    }

    #[test]
    fn test_field_lines_key_layout() {
        let field = TestField {
            value: vec![Line::from("one"), Line::from("two")],
        };
        let options = RenderOptions {
            key_layout: KeyLayout {
                separator: " -> ".into(),
                width: 12,
                padding_left: 2,
                fields: vec![(
                    FieldKind::Head,
                    FieldStyle {
                        key: Some("Tip".into()),
                        key_color: Some(9),
                        output_color: Some(3),
                    },
                )],
            },
            ..RenderOptions::default()
        };
        assert_eq!(
            field_lines(FieldKind::Head, &field, &options),
            vec![
                Line::from(vec![
                    Segment::plain("  "),
                    Segment::new("Tip", Style::KeyColor(9)),
                    Segment::new(" ->", Style::Separator),
                    Segment::plain(" "),
                    Segment::plain("   "),
                    Segment::new("one", Style::ValueColor(3)),
                ]),
                Line::from(vec![
                    Segment::plain(" ".repeat(12)),
                    Segment::new("two", Style::ValueColor(3)),
                ]),
            ]
        );
        // Other fields keep their key and colors.
        assert_eq!(
            field_lines(FieldKind::Url, &field, &options)[0].0[1],
            Segment::new("key", Style::Key)
        );
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
