use crate::ui::num_to_color;
use owo_colors::{AnsiColors, DynColors};

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
