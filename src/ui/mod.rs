use crate::info::langs::language::Language;
use owo_colors::{AnsiColors, DynColors};

pub mod printer;
pub mod text_colors;

pub fn get_ascii_colors(
    language_opt: Option<&Language>,
    override_language_opt: Option<&Language>,
    ascii_colors: &[u8],
    true_color: bool,
) -> Vec<DynColors> {
    let language_colors = match language_opt {
        Some(lang) => override_language_opt.unwrap_or(lang).get_colors(true_color),
        None => vec![DynColors::Ansi(AnsiColors::White)],
    };
    if ascii_colors.is_empty() {
        return language_colors;
    }

    let mut colors: Vec<DynColors> = ascii_colors.iter().map(num_to_color).collect();

    if language_colors.len() > colors.len() {
        colors.extend(language_colors.into_iter().skip(colors.len()));
    }

    colors
}

pub fn num_to_color(num: &u8) -> DynColors {
    match num {
        0 => DynColors::Ansi(AnsiColors::Black),
        1 => DynColors::Ansi(AnsiColors::Red),
        2 => DynColors::Ansi(AnsiColors::Green),
        3 => DynColors::Ansi(AnsiColors::Yellow),
        4 => DynColors::Ansi(AnsiColors::Blue),
        5 => DynColors::Ansi(AnsiColors::Magenta),
        6 => DynColors::Ansi(AnsiColors::Cyan),
        7 => DynColors::Ansi(AnsiColors::White),
        8 => DynColors::Ansi(AnsiColors::BrightBlack),
        9 => DynColors::Ansi(AnsiColors::BrightRed),
        10 => DynColors::Ansi(AnsiColors::BrightGreen),
        11 => DynColors::Ansi(AnsiColors::BrightYellow),
        12 => DynColors::Ansi(AnsiColors::BrightBlue),
        13 => DynColors::Ansi(AnsiColors::BrightMagenta),
        14 => DynColors::Ansi(AnsiColors::BrightCyan),
        15 => DynColors::Ansi(AnsiColors::BrightWhite),
        _ => DynColors::Ansi(AnsiColors::Default),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_to_color() {
        assert_eq!(num_to_color(&2), DynColors::Ansi(AnsiColors::Green));
        assert_eq!(num_to_color(&u8::MAX), DynColors::Ansi(AnsiColors::Default));
    }

    #[test]
    fn get_ascii_colors_no_language_no_custom_language_custom_colors() {
        let colors = get_ascii_colors(None, None, &[3, 5, 8], false);
        assert_eq!(colors.len(), 3);
        assert_eq!(
            colors,
            vec![num_to_color(&3), num_to_color(&5), num_to_color(&8)]
        );
    }

    #[test]
    fn get_ascii_colors_no_language_no_custom_language() {
        let colors = get_ascii_colors(None, None, &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::White)]);
    }

    #[test]
    fn get_ascii_colors_no_language_with_custom_language() {
        let colors = get_ascii_colors(None, Some(&Language::Python), &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::White)]);
    }

    #[test]
    fn get_ascii_colors_no_custom_language_no_custom_colors_no_true_color() {
        let colors = get_ascii_colors(Some(&Language::Rust), None, &[], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![
                DynColors::Ansi(AnsiColors::Red),
                DynColors::Ansi(AnsiColors::Default)
            ]
        );
    }

    #[test]
    fn get_ascii_colors_no_custom_language_no_custom_colors_true_color() {
        let colors = get_ascii_colors(Some(&Language::Rust), None, &[], true);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![DynColors::Rgb(228, 55, 23), DynColors::Rgb(255, 255, 255)]
        );
    }

    #[test]
    fn get_ascii_colors_custom_language_no_custom_colors_no_true_color() {
        let colors = get_ascii_colors(Some(&Language::Rust), Some(&Language::Sh), &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::Green)]);
    }

    #[test]
    fn get_ascii_colors_no_custom_language_custom_colors_no_true_color() {
        let colors = get_ascii_colors(Some(&Language::Rust), None, &[2, 3], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(colors, vec![num_to_color(&2), num_to_color(&3)]);
    }

    #[test]
    fn get_ascii_colors_fill_custom_colors_with_language_colors() {
        // When custom ascii colors are not enough for the given language,
        // language colors should be used as default
        let colors = get_ascii_colors(Some(&Language::Go), None, &[0], false);
        assert_eq!(colors.len(), 3);
        assert_eq!(
            colors,
            vec![
                num_to_color(&0),
                DynColors::Ansi(AnsiColors::Default),
                DynColors::Ansi(AnsiColors::Yellow)
            ]
        );
    }
}
