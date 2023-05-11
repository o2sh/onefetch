use crate::ui::num_to_color;
use owo_colors::{AnsiColors, DynColors};

#[derive(Clone)]
pub struct TextColors {
    pub title: DynColors,
    pub tilde: DynColors,
    pub underline: DynColors,
    pub subtitle: DynColors,
    pub colon: DynColors,
    pub info: DynColors,
}

impl TextColors {
    pub fn new(colors: &[u8], logo_primary_color: DynColors) -> Self {
        let mut text_colors = Self {
            title: logo_primary_color,
            tilde: DynColors::Ansi(AnsiColors::Default),
            underline: DynColors::Ansi(AnsiColors::Default),
            subtitle: logo_primary_color,
            colon: DynColors::Ansi(AnsiColors::Default),
            info: DynColors::Ansi(AnsiColors::Default),
        };

        if !colors.is_empty() {
            let custom_color = colors.iter().map(num_to_color).collect::<Vec<DynColors>>();

            text_colors.title = *custom_color.get(0).unwrap_or(&logo_primary_color);
            text_colors.tilde = *custom_color
                .get(1)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.underline = *custom_color
                .get(2)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.subtitle = *custom_color.get(3).unwrap_or(&logo_primary_color);
            text_colors.colon = *custom_color
                .get(4)
                .unwrap_or(&DynColors::Ansi(AnsiColors::Default));
            text_colors.info = *custom_color
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
    fn no_custom_colors() {
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let text_colors = TextColors::new(&[], primary_color);
        assert_eq!(text_colors.title, primary_color);
        assert_eq!(text_colors.tilde, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.underline, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.subtitle, primary_color);
        assert_eq!(text_colors.colon, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.info, DynColors::Ansi(AnsiColors::Default));
    }

    #[test]
    fn with_custom_colors() {
        let custom_colors = vec![0, 1, 2, 3, 4, 5];
        let text_colors = TextColors::new(&custom_colors, DynColors::Ansi(AnsiColors::Blue));
        assert_eq!(text_colors.title, num_to_color(&custom_colors[0]));
        assert_eq!(text_colors.tilde, num_to_color(&custom_colors[1]));
        assert_eq!(text_colors.underline, num_to_color(&custom_colors[2]));
        assert_eq!(text_colors.subtitle, num_to_color(&custom_colors[3]));
        assert_eq!(text_colors.colon, num_to_color(&custom_colors[4]));
        assert_eq!(text_colors.info, num_to_color(&custom_colors[5]));
    }

    #[test]
    fn with_some_custom_colors() {
        let custom_colors = vec![0, 1, 2];
        let primary_color = DynColors::Ansi(AnsiColors::Blue);
        let text_colors = TextColors::new(&custom_colors, primary_color);
        assert_eq!(text_colors.title, num_to_color(&custom_colors[0]));
        assert_eq!(text_colors.tilde, num_to_color(&custom_colors[1]));
        assert_eq!(text_colors.underline, num_to_color(&custom_colors[2]));
        assert_eq!(text_colors.subtitle, primary_color);
        assert_eq!(text_colors.colon, DynColors::Ansi(AnsiColors::Default));
        assert_eq!(text_colors.info, DynColors::Ansi(AnsiColors::Default));
    }
}
