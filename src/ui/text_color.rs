use crate::ui::num_to_color;
use colored::{Color, ColoredString, Colorize};

pub struct TextColor {
    pub title: Color,
    pub tilde: Option<Color>,
    pub underline: Option<Color>,
    pub subtitle: Color,
    pub colon: Option<Color>,
    pub info: Option<Color>,
}

impl TextColor {
    fn new(color: Color) -> TextColor {
        TextColor {
            title: color,
            tilde: None,
            underline: None,
            subtitle: color,
            colon: None,
            info: None,
        }
    }

    pub fn get_text_colors(text_colors: &[String], default_colors: &[Color]) -> TextColor {
        let mut text_color_set = TextColor::new(default_colors[0]);
        if !text_colors.is_empty() {
            let custom_color = text_colors
                .iter()
                .map(|color_num| {
                    let custom = num_to_color(color_num);
                    match custom {
                        Some(custom) => custom,
                        None => Color::White,
                    }
                })
                .collect::<Vec<Color>>();

            text_color_set.title = *custom_color.get(0).unwrap_or(&default_colors[0]);
            text_color_set.tilde = custom_color.get(1).cloned();
            text_color_set.underline = custom_color.get(2).cloned();
            text_color_set.subtitle = *custom_color.get(3).unwrap_or(&default_colors[0]);
            text_color_set.colon = custom_color.get(4).cloned();
            text_color_set.info = custom_color.get(5).cloned();
        }

        text_color_set
    }
}

pub fn color_if_some(s: &str, color: Option<Color>) -> ColoredString {
    if let Some(c) = color {
        s.color(c)
    } else {
        s.into()
    }
}
