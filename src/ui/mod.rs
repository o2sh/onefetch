use crate::info::langs::language::Language;
use owo_colors::{AnsiColors, DynColors};

pub mod printer;
pub mod text_colors;

pub fn get_ascii_colors(
    ascii_language: &Option<Language>,
    dominant_language: &Language,
    ascii_colors: &[u8],
    true_color: bool,
) -> Vec<DynColors> {
    let language = if let Some(ascii_language) = ascii_language {
        ascii_language
    } else {
        dominant_language
    };

    let mut language_colors: Vec<DynColors> = language.get_colors(true_color);

    if ascii_colors.is_empty() {
        language_colors
    } else {
        let mut colors: Vec<DynColors> = ascii_colors.iter().map(num_to_color).collect();

        if language_colors.len() > colors.len() {
            let mut missing = language_colors.drain(colors.len()..).collect();
            colors.append(&mut missing);
        }
        colors
    }
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
    fn get_ascii_colors_no_custom_language_no_custom_colors_no_true_color() {
        let colors = get_ascii_colors(&None, &Language::Rust, &[], false);
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
        let colors = get_ascii_colors(&None, &Language::Rust, &[], true);
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![DynColors::Rgb(228, 55, 23), DynColors::Rgb(255, 255, 255)]
        );
    }

    #[test]
    fn get_ascii_colors_custom_language_no_custom_colors_no_true_color() {
        let colors = get_ascii_colors(&Some(Language::Sh), &Language::Rust, &[], false);
        assert_eq!(colors.len(), 1);
        assert_eq!(colors, vec![DynColors::Ansi(AnsiColors::Green)]);
    }

    #[test]
    fn get_ascii_colors_no_custom_language_custom_colors_no_true_color() {
        let colors = get_ascii_colors(&None, &Language::Rust, &[2, 3], false);
        assert_eq!(colors.len(), 2);
        assert_eq!(colors, vec![num_to_color(&2), num_to_color(&3)]);
    }

    #[test]
    fn get_ascii_colors_fill_custom_colors_with_language_colors() {
        // When custom ascii colors are not enough for the given language,
        // language colors should be used as default
        let colors = get_ascii_colors(&None, &Language::Go, &[0], false);
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
