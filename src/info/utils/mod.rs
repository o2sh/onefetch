use crate::info::text::Span;
use gix::date::Time;
use std::time::SystemTime;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use time_humanize::HumanTime;

/// `1 tag`, `12 tags`, or nothing when there are none.
pub fn quantity(count: u64, singular: &str, plural: &str) -> Option<[Span; 2]> {
    let noun = match count {
        0 => return None,
        1 => singular,
        _ => plural,
    };
    Some([Span::number(count), Span::value(format!(" {noun}"))])
}

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

#[cfg(test)]
mod tests {
    use super::*;
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
}
