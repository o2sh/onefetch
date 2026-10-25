use serde::Serialize;

/// A twelfth of a mean Gregorian year (365.2425 days).
const SECONDS_PER_MONTH: i64 = 2_629_746;

/// Commits per month over the last few months, to tell at a glance whether the
/// repository is alive.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// Commits in each of the last few months, oldest first. A month is a
    /// twelfth of a year, counted back from now.
    pub commits_by_month: Vec<usize>,
}

impl Activity {
    /// `commit_times` and `now` are seconds since the Unix epoch. Commits older
    /// than `months` or in the future are left out.
    pub fn new(commit_times: &[i64], now: i64, months: usize) -> Self {
        let mut commits_by_month = vec![0; months];
        for &time in commit_times {
            let Some(age) = now.checked_sub(time).filter(|age| *age >= 0) else {
                continue;
            };
            let months_ago = (age / SECONDS_PER_MONTH) as usize;
            if months_ago < months {
                commits_by_month[months - 1 - months_ago] += 1;
            }
        }
        Self { commits_by_month }
    }

    pub fn total(&self) -> usize {
        self.commits_by_month.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const NOW: i64 = 1_700_000_000;
    const DAY: i64 = 86_400;

    #[test]
    fn buckets_commits_by_month_oldest_first() {
        let commit_times = [NOW, NOW - DAY, NOW - 40 * DAY, NOW - 360 * DAY];
        let activity = Activity::new(&commit_times, NOW, 12);

        assert_eq!(
            activity.commits_by_month,
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2]
        );
        assert_eq!(activity.total(), 4);
    }

    #[test]
    fn looks_back_the_given_number_of_months() {
        let commit_times = [NOW, NOW - 40 * DAY, NOW - 100 * DAY];
        let activity = Activity::new(&commit_times, NOW, 3);

        assert_eq!(activity.commits_by_month, vec![0, 1, 1]);
    }

    #[test]
    fn leaves_out_old_and_future_commits() {
        let commit_times = [NOW - 400 * DAY, NOW + DAY, i64::MIN, i64::MAX];
        let activity = Activity::new(&commit_times, NOW, 12);

        assert_eq!(activity.commits_by_month, vec![0; 12]);
        assert_eq!(activity.total(), 0);
    }

    #[test]
    fn serializes_commits_by_month() {
        let activity = Activity::new(&[NOW], NOW, 12);
        assert_eq!(
            serde_json::to_value(&activity).unwrap(),
            serde_json::json!({ "commitsByMonth": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] })
        );
    }
}
