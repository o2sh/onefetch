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

/// One column of the art: a character and its `{n}` color.
type Cell = (char, Option<usize>);

/// The lines of an ascii template, without the common indentation and all
/// padded to the same width.
pub fn ascii_lines(input: &str) -> Vec<Line> {
    let rows = trim_blank_rows(input.lines().map(parse_row).collect());
    let left = rows
        .iter()
        .map(|row| leading_spaces(row))
        .min()
        .unwrap_or(0);
    let right = rows.iter().map(|row| visible_width(row)).max().unwrap_or(0);
    rows.into_iter()
        .map(|mut row| {
            row.resize(row.len().max(right), (' ', None));
            to_line(&row[left..right])
        })
        .collect()
}

/// The cells of a template line: each character with the color of the last
/// `{n}` marker before it.
fn parse_row(line: &str) -> Vec<Cell> {
    let mut cells = Vec::new();
    let mut color = None;
    let mut rest = line;
    while let Some(c) = rest.chars().next() {
        if let Some(n) = color_marker(rest) {
            color = Some(n);
            rest = &rest[3..];
        } else {
            cells.push((c, color));
            rest = &rest[c.len_utf8()..];
        }
    }
    cells
}

/// The `n` of a `{n}` marker at the start of `s`.
fn color_marker(s: &str) -> Option<usize> {
    let mut chars = s.chars();
    match (chars.next(), chars.next(), chars.next()) {
        (Some('{'), Some(digit), Some('}')) => digit.to_digit(10).map(|n| n as usize),
        _ => None,
    }
}

/// Drops the blank rows at the top and at the bottom.
fn trim_blank_rows(mut rows: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    while rows.last().is_some_and(|row| is_blank(row)) {
        rows.pop();
    }
    let top = rows.iter().take_while(|row| is_blank(row)).count();
    rows.split_off(top)
}

fn is_space(cell: &Cell) -> bool {
    cell.0 == ' '
}

fn is_blank(row: &[Cell]) -> bool {
    row.iter().all(is_space)
}

fn leading_spaces(row: &[Cell]) -> usize {
    row.iter().take_while(|cell| is_space(cell)).count()
}

/// The width up to the last visible character.
fn visible_width(row: &[Cell]) -> usize {
    row.iter()
        .rposition(|cell| !is_space(cell))
        .map_or(0, |i| i + 1)
}

/// Joins neighboring cells of the same color into segments.
fn to_line(cells: &[Cell]) -> Line {
    let mut segments: Vec<Segment> = Vec::new();
    for &(c, color) in cells {
        match segments.last_mut() {
            Some(last) if last.style == Style::AsciiArt(color) => last.text.push(c),
            _ => segments.push(Segment::new(c, Style::AsciiArt(color))),
        }
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
        assert_eq!(
            lines[1],
            Line::from(vec![ascii(&format!("xxx{}", " ".repeat(23)), None)])
        );
    }

    #[test]
    fn ascii_lines_keeps_colors() {
        assert_eq!(
            ascii_lines("{0}ab{1}cd"),
            vec![Line::from(vec![ascii("ab", Some(0)), ascii("cd", Some(1))])]
        );
    }

    #[test]
    fn ascii_lines_without_visible_characters_is_empty() {
        for input in ["", "   ", "\n\n", "{0}"] {
            assert_eq!(ascii_lines(input), Vec::new());
        }
    }

    // https://github.com/o2sh/onefetch/issues/935
    #[test]
    fn ascii_lines_counts_columns_not_bytes() {
        assert_eq!(
            ascii_lines("███"),
            vec![Line::from(vec![ascii("███", None)])]
        );
    }

    #[test]
    fn parse_row_gives_each_character_its_color() {
        assert_eq!(
            parse_row("a{1}b{2}{3}c"),
            vec![('a', None), ('b', Some(1)), ('c', Some(3))]
        );
    }

    #[test]
    fn parse_row_keeps_other_braces_as_text() {
        let cells: String = parse_row("{x}{12}{}").iter().map(|cell| cell.0).collect();
        assert_eq!(cells, "{x}{12}{}");
    }

    #[test]
    fn color_marker_reads_one_digit() {
        assert_eq!(color_marker("{3}abc"), Some(3));
        assert_eq!(color_marker("{x}"), None);
        assert_eq!(color_marker("{12}"), None);
        assert_eq!(color_marker("{3"), None);
    }

    #[test]
    fn trim_blank_rows_drops_top_and_bottom() {
        let rows: Vec<_> = ["{0}", "   ", "a", "", "b", "  ", ""]
            .into_iter()
            .map(parse_row)
            .collect();
        assert_eq!(
            trim_blank_rows(rows),
            vec![parse_row("a"), parse_row(""), parse_row("b")]
        );
    }

    #[test]
    fn measures_ignore_colors() {
        let row = parse_row("  {1} {5}  {9} a {2} ");
        assert!(!is_blank(&row));
        assert_eq!(leading_spaces(&row), 6);
        assert_eq!(visible_width(&row), 7);
        assert!(is_blank(&parse_row("  {1} ")));
        assert_eq!(visible_width(&parse_row("   ")), 0);
    }

    #[test]
    fn to_line_joins_cells_of_the_same_color() {
        let cells = [('a', Some(0)), ('b', Some(0)), (' ', None), ('c', Some(1))];
        assert_eq!(
            to_line(&cells),
            Line::from(vec![
                ascii("ab", Some(0)),
                ascii(" ", None),
                ascii("c", Some(1))
            ])
        );
    }
}
