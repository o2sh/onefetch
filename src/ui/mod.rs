use crate::info::language::Language;
use colored::Color;

pub mod ascii_art;
pub mod image_backends;
pub mod printer;
pub mod text_color;

pub fn get_ascii_colors(
    ascii_language: &Option<Language>,
    dominant_language: &Language,
    ascii_colors: &[String],
    true_color: bool,
) -> Vec<Color> {
    let language =
        if let Some(ascii_language) = ascii_language { ascii_language } else { &dominant_language };

    let colors = language.get_colors(true_color);

    let colors: Vec<Color> = colors
        .iter()
        .enumerate()
        .map(|(index, default_color)| {
            if let Some(color_num) = ascii_colors.get(index) {
                if let Some(color) = num_to_color(color_num) {
                    return color;
                }
            }
            *default_color
        })
        .collect();
    colors
}

fn num_to_color(num: &str) -> Option<Color> {
    let color = match num {
        "0" => Color::Black,
        "1" => Color::Red,
        "2" => Color::Green,
        "3" => Color::Yellow,
        "4" => Color::Blue,
        "5" => Color::Magenta,
        "6" => Color::Cyan,
        "7" => Color::White,
        "8" => Color::BrightBlack,
        "9" => Color::BrightRed,
        "10" => Color::BrightGreen,
        "11" => Color::BrightYellow,
        "12" => Color::BrightBlue,
        "13" => Color::BrightMagenta,
        "14" => Color::BrightCyan,
        "15" => Color::BrightWhite,
        _ => return None,
    };
    Some(color)
}
