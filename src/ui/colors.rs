use crate::language::Language;
use owo_colors::{AnsiColors, DynColors};

pub fn get_ascii_colors(
    language_opt: Option<&Language>,
    override_language_opt: Option<&Language>,
    ascii_colors: &[u8],
    true_color: bool,
) -> Vec<DynColors> {
    let language_colors = match override_language_opt.or(language_opt) {
        Some(lang) => lang.get_colors(true_color),
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

pub struct TextColors {
    pub title: DynColors,
    pub tilde: DynColors,
    pub underline: DynColors,
    pub key: DynColors,
    pub separator: DynColors,
    pub value: DynColors,
}

impl TextColors {
    pub fn new(colors: &[u8], primary_color: DynColors) -> Self {
        let mut text_colors = Self {
            title: primary_color,
            tilde: DynColors::Ansi(AnsiColors::Default),
            underline: DynColors::Ansi(AnsiColors::Default),
            key: primary_color,
            separator: DynColors::Ansi(AnsiColors::Default),
            value: DynColors::Ansi(AnsiColors::Default),
        };

        if !colors.is_empty() {
            let custom_color = colors.iter().map(num_to_color).collect::<Vec<DynColors>>();

            text_colors.title = *custom_color.first().unwrap_or(&primary_color);
            text_colors.tilde = *custom_color
                .get(1)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.underline = *custom_color
                .get(2)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.key = *custom_color.get(3).unwrap_or(&primary_color);
            text_colors.separator = *custom_color
                .get(4)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.value = *custom_color
                .get(5)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
        }
        text_colors
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
        assert_eq!(colors.len(), 2);
        assert_eq!(
            colors,
            vec![
                DynColors::Ansi(AnsiColors::Blue),
                DynColors::Ansi(AnsiColors::Yellow)
            ]
        );
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

    #[test]
    fn no_custom_colors() {
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let text_colors = TextColors::new(&[], primary_color);
        assert_eq!(text_colors.title, primary_color);
        assert_eq!(text_colors.tilde, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.underline, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.key, primary_color);
        assert_eq!(text_colors.separator, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.value, DynColors::Ansi(AnsiColors::Default));
    }

    #[test]
    fn with_custom_colors() {
        let custom_colors = vec![0, 1, 2, 3, 4, 5];
        let text_colors = TextColors::new(&custom_colors, DynColors::Ansi(AnsiColors::Blue));
        assert_eq!(text_colors.title, num_to_color(&custom_colors[0]));
        assert_eq!(text_colors.tilde, num_to_color(&custom_colors[1]));
        assert_eq!(text_colors.underline, num_to_color(&custom_colors[2]));
        assert_eq!(text_colors.key, num_to_color(&custom_colors[3]));
        assert_eq!(text_colors.separator, num_to_color(&custom_colors[4]));
        assert_eq!(text_colors.value, num_to_color(&custom_colors[5]));
    }

    #[test]
    fn with_some_custom_colors() {
        let custom_colors = vec![0, 1, 2];
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let text_colors = TextColors::new(&custom_colors, primary_color);
        assert_eq!(text_colors.title, num_to_color(&custom_colors[0]));
        assert_eq!(text_colors.tilde, num_to_color(&custom_colors[1]));
        assert_eq!(text_colors.underline, num_to_color(&custom_colors[2]));
        assert_eq!(text_colors.key, primary_color);
        assert_eq!(text_colors.separator, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.value, DynColors::Ansi(AnsiColors::Default));
    }
}
