use crate::cli::NumberSeparator;
use gix::date::Time;
use num_format::ToFormattedString;
use serde::Serializer;
use std::time::SystemTime;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use time_humanize::HumanTime;

/// How numbers and dates are written in the terminal output. Fields keep raw
/// values and use this to display them.
#[derive(Clone, Copy, Debug, Default)]
pub struct Format {
    pub number_separator: NumberSeparator,
    pub iso_time: bool,
}

impl Format {
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

/// Serializes a time as RFC 3339, whatever the display format, or as seconds
/// since the epoch when RFC 3339 can't represent it.
pub fn serialize_time<S: Serializer>(time: &Time, serializer: S) -> Result<S::Ok, S::Error> {
    match to_rfc3339(*time) {
        Some(rfc3339) => serializer.serialize_str(&rfc3339),
        None => serializer.serialize_i64(time.seconds),
    }
}

/// `None` for dates outside the years 0 to 9999. Commit dates come from the
/// repository, so they can be anything.
fn to_rfc3339(time: Time) -> Option<String> {
    OffsetDateTime::from_unix_timestamp(time.seconds)
        .ok()?
        .format(&Rfc3339)
        .ok()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::time::{Duration, SystemTime};

    const HUMAN_TIME: Format = Format {
        number_separator: NumberSeparator::Plain,
        iso_time: false,
    };

    const ISO_TIME: Format = Format {
        number_separator: NumberSeparator::Plain,
        iso_time: true,
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
    fn display_time_as_iso_time_some_time() {
        // Set "current" time to 11/18/2021 11:02:22
        let time = Time::new(1_637_233_282, 0);
        assert_eq!(ISO_TIME.time(time), "2021-11-18T11:01:22Z");
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
    #[case(gix::date::SecondsSinceUnixEpoch::MIN)]
    #[case(gix::date::SecondsSinceUnixEpoch::MAX)]
    #[case(253_402_300_800)] // 10000-01-01
    fn serialize_out_of_range_time_as_seconds(#[case] seconds: gix::date::SecondsSinceUnixEpoch) {
        #[derive(serde::Serialize)]
        struct Date(#[serde(serialize_with = "serialize_time")] Time);

        let json = serde_json::to_value(Date(Time::new(seconds, 0))).unwrap();
        assert_eq!(json, seconds);
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
        let format = Format {
            number_separator,
            iso_time: false,
        };
        assert_eq!(format.number(&number), expected);
    }
}
