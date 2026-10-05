use gix::date::Time;
use serde::Serializer;
use std::time::SystemTime;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use time_humanize::HumanTime;

/// Serializes a time as RFC 3339, whatever `--iso-time` is set to, or as
/// seconds since the epoch when RFC 3339 can't represent it.
pub fn serialize_time<S: Serializer>(time: &Time, serializer: S) -> Result<S::Ok, S::Error> {
    match to_rfc3339(*time) {
        Some(rfc3339) => serializer.serialize_str(&rfc3339),
        None => serializer.serialize_i64(time.seconds),
    }
}

/// `None` for dates outside the years 0 to 9999. Commit dates come from the
/// repository, so they can be anything.
pub fn to_rfc3339(time: Time) -> Option<String> {
    OffsetDateTime::from_unix_timestamp(time.seconds)
        .ok()?
        .format(&Rfc3339)
        .ok()
}

pub fn to_human_time(time: Time) -> String {
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
}
