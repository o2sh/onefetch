use crate::info::langs::language::Language;
use owo_colors::{AnsiColors, DynColors};

pub mod ascii_art;
pub mod image_backends;
pub mod printer;
pub mod text_colors;

pub fn get_ascii_colors(
    ascii_language: &Option<Language>,
    dominant_language: &Language,
    ascii_colors: &[String],
    true_color: bool,
) -> Vec<DynColors> {
    let language = if let Some(ascii_language) = ascii_language {
        ascii_language
    } else {
        dominant_language
    };

    let colors = language.get_colors(true_color);

    let colors: Vec<DynColors> = colors
        .iter()
        .enumerate()
        .map(|(index, default_color)| {
            if let Some(color_num) = ascii_colors.get(index) {
                return num_to_color(color_num);
            }
            *default_color
        })
        .collect();
    colors
}

fn num_to_color(num: &str) -> DynColors {
    match num {
        "0" => DynColors::Ansi(AnsiColors::Black),
        "1" => DynColors::Ansi(AnsiColors::Red),
        "2" => DynColors::Ansi(AnsiColors::Green),
        "3" => DynColors::Ansi(AnsiColors::Yellow),
        "4" => DynColors::Ansi(AnsiColors::Blue),
        "5" => DynColors::Ansi(AnsiColors::Magenta),
        "6" => DynColors::Ansi(AnsiColors::Cyan),
        "7" => DynColors::Ansi(AnsiColors::White),
        "8" => DynColors::Ansi(AnsiColors::BrightBlack),
        "9" => DynColors::Ansi(AnsiColors::BrightRed),
        "10" => DynColors::Ansi(AnsiColors::BrightGreen),
        "11" => DynColors::Ansi(AnsiColors::BrightYellow),
        "12" => DynColors::Ansi(AnsiColors::BrightBlue),
        "13" => DynColors::Ansi(AnsiColors::BrightMagenta),
        "14" => DynColors::Ansi(AnsiColors::BrightCyan),
        "15" => DynColors::Ansi(AnsiColors::BrightWhite),
        _ => DynColors::Ansi(AnsiColors::Default),
    }
}
