use crate::cli::MyRegex;
use anyhow::Result;
use git::bstr::BString;
use git_repository as git;
use git_repository::bstr::ByteSlice;
use std::collections::HashMap;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use time_humanize::HumanTime;

use super::repo::author::Author;

pub struct Commits {
    authors: Vec<Author>,
    pub total_num_authors: usize,
    pub num_commits: usize,
    /// false if we have found the first commit that started it all, true if the repository is shallow.
    pub is_shallow: bool,
    pub time_of_most_recent_commit: git::actor::Time,
    pub time_of_first_commit: git::actor::Time,
}

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Sig {
    name: git::bstr::BString,
    email: git::bstr::BString,
}

impl From<git::actor::Signature> for Sig {
    fn from(git::actor::Signature { name, email, .. }: git::actor::Signature) -> Self {
        Self { name, email }
    }
}

impl Commits {
    pub fn new(
        mut repo: git::Repository,
        no_merges: bool,
        bot_regex_pattern: &Option<MyRegex>,
        number_of_authors_to_display: usize,
        show_email: bool,
    ) -> Result<Self> {
        // assure that objects we just traversed are coming from cache
        // when we read the commit right after.
        repo.object_cache_size(32 * 1024);

        let mut time_of_most_recent_commit = None;
        let mut time_of_first_commit = None;
        let mut commit_iter = repo.head_commit()?.ancestors().all()?;
        let mut commit_iter_peekable = commit_iter.by_ref().peekable();

        let mailmap = repo.open_mailmap();
        let mut author_to_number_of_commits: HashMap<Sig, usize> = HashMap::new();
        let mut total_nbr_of_commits = 0;

        let mut num_commits = 0;
        while let Some(commit_id) = commit_iter_peekable.next() {
            let commit = commit_id?.object()?.into_commit();
            let commit = commit.decode()?;

            if no_merges && commit.parents().take(2).count() > 1 {
                continue;
            }

            let sig = Sig::from(mailmap.resolve(commit.author));

            if is_bot(&sig.name, bot_regex_pattern) {
                continue;
            }
            num_commits += 1;

            let author_nbr_of_commits = author_to_number_of_commits.entry(sig).or_insert(0);
            *author_nbr_of_commits += 1;
            total_nbr_of_commits += 1;

            time_of_most_recent_commit.get_or_insert_with(|| commit.time());
            if commit_iter_peekable.peek().is_none() {
                time_of_first_commit = commit.time().into();
            }
        }

        let mut authors_by_number_of_commits: Vec<(Sig, usize)> =
            author_to_number_of_commits.into_iter().collect();

        let total_num_authors = authors_by_number_of_commits.len();
        authors_by_number_of_commits.sort_by(|(sa, a_count), (sb, b_count)| {
            b_count.cmp(a_count).then_with(|| sa.name.cmp(&sb.name))
        });

        let authors: Vec<Author> = authors_by_number_of_commits
            .into_iter()
            .map(|(author, author_nbr_of_commits)| {
                let email = author.email;
                Author::new(
                    author.name,
                    email,
                    author_nbr_of_commits,
                    total_nbr_of_commits,
                    show_email,
                )
            })
            .take(number_of_authors_to_display)
            .collect();

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        let is_shallow = commit_iter.is_shallow.expect(
            "BUG: we must deplete the iterator. If you are seeing this, please let us know at https://github.com/o2sh/onefetch/issues/new",
        );
        drop(commit_iter);
        Ok(Self {
            authors,
            total_num_authors,
            num_commits,
            is_shallow,
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }

    pub fn authors(&mut self) -> Vec<Author> {
        std::mem::take(&mut self.authors)
    }
}

fn is_bot(author_name: &BString, bot_regex_pattern: &Option<MyRegex>) -> bool {
    bot_regex_pattern
        .as_ref()
        .map(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
        .unwrap_or(false)
}

pub fn gitoxide_time_to_formatted_time(time: git::actor::Time, iso_time: bool) -> String {
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

        let time = git::actor::Time::new(current_time.as_secs() as u32, 0);
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
        let time = git::actor::Time::new(year_ago.as_secs() as u32, 0);
        let result = gitoxide_time_to_formatted_time(time, false);
        assert_eq!(result, "a year ago");
    }

    #[test]
    fn display_time_as_iso_time_some_time() {
        // Set "current" time to 11/18/2021 11:02:22
        let time_sample = 1637233282;
        let time = git::actor::Time::new(time_sample, 0);
        let result = gitoxide_time_to_formatted_time(time, true);
        assert_eq!(result, "2021-11-18T11:01:22Z");
    }
    #[test]
    fn display_time_as_iso_time_current_epoch() {
        let time_sample = 0;
        let time = git::actor::Time::new(time_sample, 0);
        let result = gitoxide_time_to_formatted_time(time, true);
        assert_eq!(result, "1970-01-01T00:00:00Z");
    }
}
