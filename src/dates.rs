//! Date helpers shared by `data` and `render`.

use gix::date::{SecondsSinceUnixEpoch, Time};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

/// The current time in seconds since the Unix epoch, or 0 if the clock is set
/// before it.
pub fn now() -> SecondsSinceUnixEpoch {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs() as SecondsSinceUnixEpoch)
}

/// `None` for dates outside the years 0 to 9999. Commit dates come from the
/// repository, so they can be anything.
pub fn to_rfc3339(time: Time) -> Option<String> {
    OffsetDateTime::from_unix_timestamp(time.seconds)
        .ok()?
        .format(&Rfc3339)
        .ok()
}
