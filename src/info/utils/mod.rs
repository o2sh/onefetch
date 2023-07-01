use std::time::{Duration, SystemTime};

use gix::date::Time;
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
        .unwrap();

    let ts = Duration::from_secs(match time.seconds.try_into() {
        Ok(s) => s,
        Err(_) => return "<before UNIX epoch>".into(),
    });
    let duration = since_epoch_duration.checked_sub(ts).expect(
        "Achievement unlocked: time travel! \
        Check your system clock and commit dates.",
    );
    let ht = HumanTime::from(-(duration.as_secs() as i64));
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
    #[should_panic(
        expected = "Achievement unlocked: time travel! Check your system clock and commit dates."
    )]
    fn should_panic_when_display_human_time_and_commit_date_in_the_future() {
        let day = Duration::from_secs(60 * 60 * 24);
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let tomorrow = current_time + day;
        let time = Time::new(tomorrow.as_secs() as gix::date::SecondsSinceUnixEpoch, 0);
        format_time(time, false);
    }

    #[test]
    fn display_time_before_epoch() {
        let time = Time::new(gix::date::SecondsSinceUnixEpoch::MIN, 0);
        let result = to_human_time(time);
        assert_eq!(result, "<before UNIX epoch>");
    }
}
