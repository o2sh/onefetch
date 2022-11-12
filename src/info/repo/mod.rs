use git_repository::actor::Time;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use time_humanize::HumanTime;

pub mod author;
pub mod commits;
pub mod contributors;
pub mod created;
pub mod dependencies;
pub mod description;
pub mod head;
pub mod last_change;
pub mod license;
pub mod loc;
pub mod pending;
pub mod project;
pub mod size;
pub mod url;
pub mod version;

pub fn gitoxide_time_to_formatted_time(time: Time, iso_time: bool) -> String {
    if iso_time {
        to_rfc3339(HumanTime::from(time.seconds_since_unix_epoch as i64))
    } else {
        let ht = HumanTime::from_duration_since_timestamp(time.seconds_since_unix_epoch as u64);
        ht.to_string()
    }
}

fn to_rfc3339<T>(dt: T) -> String
where
    T: Into<OffsetDateTime>,
{
    dt.into().format(&Rfc3339).unwrap()
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

        let time = Time::new(current_time.as_secs() as u32, 0);
        let result = gitoxide_time_to_formatted_time(time, false);
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
        let time = Time::new(year_ago.as_secs() as u32, 0);
        let result = gitoxide_time_to_formatted_time(time, false);
        assert_eq!(result, "a year ago");
    }

    #[test]
    fn display_time_as_iso_time_some_time() {
        // Set "current" time to 11/18/2021 11:02:22
        let time_sample = 1637233282;
        let time = Time::new(time_sample, 0);
        let result = gitoxide_time_to_formatted_time(time, true);
        assert_eq!(result, "2021-11-18T11:01:22Z");
    }
    #[test]
    fn display_time_as_iso_time_current_epoch() {
        let time_sample = 0;
        let time = Time::new(time_sample, 0);
        let result = gitoxide_time_to_formatted_time(time, true);
        assert_eq!(result, "1970-01-01T00:00:00Z");
    }
}
