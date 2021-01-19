use byte_unit::Byte;
use chrono::{FixedOffset, TimeZone};
use chrono_humanize::HumanTime;
use colored::Color;
use git2::Time;

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

pub fn git_time_to_formatted_time(time: &Time, iso_time: bool) -> String {
    let (offset, _) = match time.offset_minutes() {
        n if n < 0 => (-n, '-'),
        n => (n, '+'),
    };

    let offset = FixedOffset::west(offset);
    let dt_with_tz = offset.timestamp(time.seconds(), 0);
    if iso_time {
        dt_with_tz.with_timezone(&chrono::Utc).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    } else {
        let ht = HumanTime::from(dt_with_tz);
        format!("{}", ht)
    }
}
