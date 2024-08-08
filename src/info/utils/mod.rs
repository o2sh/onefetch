use crate::cli::NumberSeparator;
use gix::date::Time;
use num_format::ToFormattedString;
use owo_colors::{DynColors, Style};
use std::time::SystemTime;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use time_humanize::HumanTime;

pub mod info_field;

pub fn format_time(time: Time, iso_time: bool) -> String {
    if iso_time {
        to_rfc3339(HumanTime::from(time.seconds))
    } else {
        to_human_time(time)
    }
}

fn to_rfc3339<T>(dt: T) -> String
where
    T: Into<OffsetDateTime>,
{
    dt.into().format(&Rfc3339).unwrap()
}

fn to_human_time(time: Time) -> String {
    let since_epoch_duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time is before the Unix epoch");
    // Calculate the distance from the current time. This handles
    // future dates gracefully and will simply return something like `in 5 minutes`
    let delta_in_seconds = time
        .seconds
        .saturating_sub(since_epoch_duration.as_secs() as i64);
    let ht = HumanTime::from(delta_in_seconds);
    ht.to_string()
}

pub fn format_number<T: ToFormattedString + std::fmt::Display>(
    number: &T,
    number_separator: NumberSeparator,
) -> String {
    number.to_formatted_string(&number_separator.get_format())
}

pub fn get_style(is_bold: bool, color: DynColors) -> Style {
    let mut style = Style::new().color(color);
    if is_bold {
        style = style.bold();
    }
    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors;
    use rstest::rstest;
    use std::time::{Duration, SystemTime};

    #[test]
    fn display_time_as_human_time_current_time_now() {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let time = Time::new(
            current_time.as_secs() as gix::date::SecondsSinceUnixEpoch,
            0,
        );
        let result = format_time(time, false);
        assert_eq!(result, "now");
    }

    #[test]
    fn display_time_as_human_time_current_time_arbitrary() {
        let day = Duration::from_secs(60 * 60 * 24);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        // NOTE 366 so that it's a year ago even with leap years.
        let year_ago = current_time - (day * 366);
        let time = Time::new(year_ago.as_secs() as gix::date::SecondsSinceUnixEpoch, 0);
        let result = format_time(time, false);
        assert_eq!(result, "a year ago");
    }

    #[test]
    fn display_time_as_iso_time_some_time() {
        // Set "current" time to 11/18/2021 11:02:22
        let time_sample = 1_637_233_282;
        let time = Time::new(time_sample, 0);
        let result = format_time(time, true);
        assert_eq!(result, "2021-11-18T11:01:22Z");
    }

    #[test]
    fn display_time_as_iso_time_current_epoch() {
        let time_sample = 0;
        let time = Time::new(time_sample, 0);
        let result = format_time(time, true);
        assert_eq!(result, "1970-01-01T00:00:00Z");
    }

    #[test]
    fn handle_display_human_time_and_commit_date_in_the_future() {
        let day = Duration::from_secs(60 * 60 * 24);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let tomorrow = current_time + day;
        let time = Time::new(tomorrow.as_secs() as gix::date::SecondsSinceUnixEpoch, 0);
        let result = format_time(time, false);
        assert_eq!(result, "in a day");
    }

    #[test]
    fn display_time_before_epoch() {
        let time = Time::new(gix::date::SecondsSinceUnixEpoch::MIN, 0);
        let result = to_human_time(time);
        assert!(result.ends_with(" years ago"));
    }

    #[rstest]
    #[case(1_000_000, NumberSeparator::Comma, "1,000,000")]
    #[case(1_000_000, NumberSeparator::Space, "1\u{202f}000\u{202f}000")]
    #[case(1_000_000, NumberSeparator::Underscore, "1_000_000")]
    #[case(1_000_000, NumberSeparator::Plain, "1000000")]
    fn test_format_number(
        #[case] number: usize,
        #[case] number_separator: NumberSeparator,
        #[case] expected: &str,
    ) {
        assert_eq!(&format_number(&number, number_separator), expected);
    }

    #[test]
    fn test_get_style() {
        let style = get_style(true, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(
            style,
            Style::new().color(DynColors::Ansi(AnsiColors::Cyan)).bold()
        );
    }

    #[test]
    fn test_get_style_no_bold() {
        let style = get_style(false, DynColors::Ansi(AnsiColors::Cyan));
        assert_eq!(style, Style::new().color(DynColors::Ansi(AnsiColors::Cyan)));
    }
}
