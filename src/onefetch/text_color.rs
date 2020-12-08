use {
    crate::onefetch::serializer::ColorDef, crate::onefetch::utils::num_to_color, colored::Color,
    serde::Serialize,
};

#[derive(Serialize)]
pub struct TextColor {
    #[serde(with = "ColorDef")]
    pub title: Color,
    #[serde(with = "ColorDef")]
    pub tilde: Color,
    #[serde(with = "ColorDef")]
    pub underline: Color,
    #[serde(with = "ColorDef")]
    pub subtitle: Color,
    #[serde(with = "ColorDef")]
    pub colon: Color,
    #[serde(with = "ColorDef")]
    pub info: Color,
}

impl TextColor {
    fn new(color: Color) -> TextColor {
        TextColor {
            title: color,
            tilde: Color::White,
            underline: Color::White,
            subtitle: color,
            colon: Color::White,
            info: Color::White,
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
            text_color_set.tilde = *custom_color.get(1).unwrap_or(&Color::White);
            text_color_set.underline = *custom_color.get(2).unwrap_or(&Color::White);
            text_color_set.subtitle = *custom_color.get(3).unwrap_or(&default_colors[0]);
            text_color_set.colon = *custom_color.get(4).unwrap_or(&Color::White);
            text_color_set.info = *custom_color.get(5).unwrap_or(&Color::White);
        }

        text_color_set
    }
}
