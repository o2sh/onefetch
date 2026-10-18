//! The ascii art.
//!
//! Turns an ascii template into lines: the text after a `{n}` marker is styled
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
use regex::regex;

/// The lines of an ascii template, without the common indentation and all
/// padded to the same width.
pub fn ascii_lines(input: &str) -> Vec<Line> {
    let lines = parse_lines(input);
    let Some(start) = lines.iter().map(leading_spaces).min() else {
        return Vec::new();
    };
    let end = lines.iter().map(visible_width).max().unwrap_or(start);
    lines.iter().map(|line| crop(line, start, end)).collect()
}

/// Parses the template, without the empty lines at the top and the blank
/// lines at the bottom.
fn parse_lines(input: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = input
        .lines()
        .skip_while(|line| line.is_empty())
        .map(parse_line)
        .collect();
    while lines.last().is_some_and(is_blank) {
        lines.pop();
    }
    lines
}

/// Splits a template line at its `{n}` markers. The text before the first
/// marker has no color.
fn parse_line(line: &str) -> Line {
    let mut segments = Vec::new();
    let mut color = None;
    let mut text_start = 0;
    for marker in regex!(r"\{([0-9])\}").captures_iter(line) {
        let range = marker.get(0).expect("group 0 is the whole match").range();
        push_text(&mut segments, &line[text_start..range.start], color);
        color = marker[1].parse().ok();
        text_start = range.end;
    }
    push_text(&mut segments, &line[text_start..], color);
    Line::from(segments)
}

fn push_text(segments: &mut Vec<Segment>, text: &str, color: Option<usize>) {
    if !text.is_empty() {
        segments.push(Segment::new(text, Style::AsciiArt(color)));
    }
}

/// The characters of a line, across its segments.
fn chars(line: &Line) -> impl Iterator<Item = char> + '_ {
    line.0.iter().flat_map(|segment| segment.text.chars())
}

/// Whether the line has no visible character.
fn is_blank(line: &Line) -> bool {
    chars(line).all(|c| c == ' ')
}

/// The number of spaces before the first visible character.
fn leading_spaces(line: &Line) -> usize {
    chars(line).take_while(|&c| c == ' ').count()
}

/// The width up to the last visible character.
fn visible_width(line: &Line) -> usize {
    chars(line)
        .enumerate()
        .filter(|&(_, c)| c != ' ')
        .last()
        .map_or(0, |(i, _)| i + 1)
}

/// Keeps the columns from `start` to `end`, padded with spaces to that width.
fn crop(line: &Line, start: usize, end: usize) -> Line {
    let mut skip = start;
    let mut width = end - start;
    let mut segments = Vec::new();
    for segment in &line.0 {
        let text: String = segment.text.chars().skip(skip).take(width).collect();
        skip = skip.saturating_sub(segment.text.chars().count());
        width -= text.chars().count();
        if !text.is_empty() {
            segments.push(Segment::new(text, segment.style));
        }
    }
    if width > 0 {
        segments.push(Segment::plain(" ".repeat(width)));
    }
    Line::from(segments)
}

#[cfg(test)]
mod test {
    use super::*;

    fn ascii(text: &str, color: Option<usize>) -> Segment {
        Segment::new(text, Style::AsciiArt(color))
    }

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
    fn parse_lines_trims_top_and_bottom() {
        let lines = parse_lines("\n\n{0}a\n\n{0}b\n  \n{1}\n");
        assert_eq!(
            lines,
            vec![
                Line::from(vec![ascii("a", Some(0))]),
                Line::default(),
                Line::from(vec![ascii("b", Some(0))]),
            ]
        );
    }

    #[test]
    fn parse_line_splits_at_markers() {
        assert_eq!(
            parse_line("{0}ab{1}cd"),
            Line::from(vec![ascii("ab", Some(0)), ascii("cd", Some(1))])
        );
        assert_eq!(
            parse_line("ab{1}{2}cd"),
            Line::from(vec![ascii("ab", None), ascii("cd", Some(2))])
        );
    }

    #[test]
    fn parse_line_keeps_other_braces_as_text() {
        assert_eq!(
            parse_line("{x}{12}{}"),
            Line::from(vec![ascii("{x}{12}{}", None)])
        );
    }

    #[test]
    fn is_blank_ignores_markers_and_spaces() {
        assert!(is_blank(&parse_line("")));
        assert!(is_blank(&parse_line("  {1} ")));
        assert!(!is_blank(&parse_line("  {1}a")));
    }

    #[test]
    fn leading_spaces_counts_correctly() {
        assert_eq!(leading_spaces(&parse_line("")), 0);
        assert_eq!(leading_spaces(&parse_line("     ")), 5);
        assert_eq!(leading_spaces(&parse_line("     a;lksjf;a")), 5);
        assert_eq!(leading_spaces(&parse_line("  {1} {5}  {9} a")), 6);
    }

    #[test]
    fn visible_width_ignores_trailing_spaces() {
        assert_eq!(visible_width(&parse_line("")), 0);
        assert_eq!(visible_width(&parse_line("     ")), 0);
        assert_eq!(visible_width(&parse_line("  ab  ")), 4);
        assert_eq!(visible_width(&parse_line("{0}a{1} b{2}  ")), 3);
    }

    #[test]
    fn crop_keeps_the_columns_and_pads() {
        let cropped = |text, start, end| crop(&parse_line(text), start, end);

        assert_eq!(cropped("", 0, 0), Line::default());
        assert_eq!(cropped("     ", 0, 0), Line::default());
        assert_eq!(
            cropped("     ", 0, 5),
            Line::from(vec![ascii("     ", None)])
        );
        assert_eq!(
            cropped("     ", 1, 5),
            Line::from(vec![ascii("    ", None)])
        );
        assert_eq!(cropped("     ", 3, 5), Line::from(vec![ascii("  ", None)]));
        assert_eq!(
            cropped("     ", 0, 4),
            Line::from(vec![ascii("    ", None)])
        );
        assert_eq!(cropped("     ", 0, 3), Line::from(vec![ascii("   ", None)]));

        // https://github.com/o2sh/onefetch/issues/935
        assert_eq!(cropped("███", 0, 3), Line::from(vec![ascii("███", None)]));

        assert_eq!(
            cropped("  {1} {5}  {9} a", 4, 10),
            Line::from(vec![
                ascii(" ", Some(5)),
                ascii(" a", Some(9)),
                Segment::plain("   "),
            ])
        );
    }

    #[test]
    fn crop_keeps_colors() {
        assert_eq!(
            crop(&parse_line("{0}ab{1}cd"), 0, 4),
            Line::from(vec![ascii("ab", Some(0)), ascii("cd", Some(1))])
        );
    }
}
