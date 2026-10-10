use crate::info::dates::{to_human_time, to_rfc3339};
use gix::date::Time;
use num_format::{CustomFormat, ToFormattedString};
use serde::Serialize;

pub const DEFAULT_NUMBER_OF_LANGUAGES: usize = 6;
pub const DEFAULT_NUMBER_OF_AUTHORS: usize = 3;

/// How fields are displayed in the terminal output: number and date formats,
/// language chips, the color palette, and how many entries to list. Fields
/// keep raw values and use this to display them.
#[derive(Clone, Copy, Debug)]
pub struct DisplayOptions {
    pub number_separator: NumberSeparator,
    pub iso_time: bool,
    pub true_color: bool,
    pub nerd_fonts: bool,
    pub no_color_palette: bool,
    pub number_of_languages: usize,
    pub number_of_authors: usize,
}

/// The CLI defaults, except that true colors are off since `--true-color auto`
/// depends on the terminal.
impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            number_separator: NumberSeparator::default(),
            iso_time: false,
            true_color: false,
            nerd_fonts: false,
            no_color_palette: false,
            number_of_languages: DEFAULT_NUMBER_OF_LANGUAGES,
            number_of_authors: DEFAULT_NUMBER_OF_AUTHORS,
        }
    }
}

#[derive(clap::ValueEnum, Clone, PartialEq, Eq, Debug, Serialize, Copy, Default)]
pub enum NumberSeparator {
    #[default]
    Plain,
    Comma,
    Space,
    Underscore,
}

impl NumberSeparator {
    fn separator(self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Comma => ",",
            Self::Space => "\u{202f}",
            Self::Underscore => "_",
        }
    }

    pub fn get_format(&self) -> CustomFormat {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator(self.separator())
            .build()
            .unwrap()
    }
}

impl DisplayOptions {
    pub fn number<T: ToFormattedString>(&self, number: &T) -> String {
        number.to_formatted_string(&self.number_separator.get_format())
    }

    /// Dates that RFC 3339 can't represent are shown as relative time.
    pub fn time(&self, time: Time) -> String {
        if self.iso_time
            && let Some(rfc3339) = to_rfc3339(time)
        {
            return rfc3339;
        }
        to_human_time(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::time::{Duration, SystemTime};

    const HUMAN_TIME: DisplayOptions = DisplayOptions {
        number_separator: NumberSeparator::Plain,
        iso_time: false,
        true_color: false,
        nerd_fonts: false,
        no_color_palette: false,
        number_of_languages: 0,
        number_of_authors: 0,
    };

    const ISO_TIME: DisplayOptions = DisplayOptions {
        iso_time: true,
        ..HUMAN_TIME
    };

    #[test]
    fn display_time_as_human_time_current_time_now() {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let time = Time::new(
            current_time.as_secs() as gix::date::SecondsSinceUnixEpoch,
            0,
        );
        assert_eq!(HUMAN_TIME.time(time), "now");
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
        assert_eq!(HUMAN_TIME.time(time), "a year ago");
    }

    #[test]
    fn display_time_as_iso_time_current_epoch() {
        let time = Time::new(0, 0);
        assert_eq!(ISO_TIME.time(time), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn handle_display_human_time_and_commit_date_in_the_future() {
        let day = Duration::from_secs(60 * 60 * 24);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let tomorrow = current_time + day;
        let time = Time::new(tomorrow.as_secs() as gix::date::SecondsSinceUnixEpoch, 0);
        assert_eq!(HUMAN_TIME.time(time), "in a day");
    }

    #[test]
    fn display_time_before_epoch() {
        let time = Time::new(gix::date::SecondsSinceUnixEpoch::MIN, 0);
        assert!(HUMAN_TIME.time(time).ends_with(" years ago"));
    }

    #[rstest]
    #[case(gix::date::SecondsSinceUnixEpoch::MIN)]
    #[case(gix::date::SecondsSinceUnixEpoch::MAX)]
    #[case(253_402_300_800)] // 10000-01-01
    fn display_out_of_range_iso_time_as_human_time(
        #[case] seconds: gix::date::SecondsSinceUnixEpoch,
    ) {
        let time = Time::new(seconds, 0);
        assert_eq!(ISO_TIME.time(time), HUMAN_TIME.time(time));
    }

    #[rstest]
    #[case(1_000_000, NumberSeparator::Comma, "1,000,000")]
    #[case(1_000_000, NumberSeparator::Space, "1\u{202f}000\u{202f}000")]
    #[case(1_000_000, NumberSeparator::Underscore, "1_000_000")]
    #[case(1_000_000, NumberSeparator::Plain, "1000000")]
    fn test_number(
        #[case] number: usize,
        #[case] number_separator: NumberSeparator,
        #[case] expected: &str,
    ) {
        let options = DisplayOptions {
            number_separator,
            ..DisplayOptions::default()
        };
        assert_eq!(options.number(&number), expected);
    }
}
