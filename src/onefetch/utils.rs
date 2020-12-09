use byte_unit::Byte;
use colored::Color;

pub fn num_to_color(num: &str) -> Option<Color> {
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

pub fn bytes_to_human_readable(bytes: u128) -> String {
    let byte = Byte::from_bytes(bytes);
    byte.get_appropriate_unit(true).to_string()
}
