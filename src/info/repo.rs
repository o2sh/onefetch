use crate::info::author::Author;
use crate::info::head_refs::HeadRefs;
use anyhow::{Context, Result};
use byte_unit::Byte;
use git2::{Repository, RepositoryOpenFlags, Status, StatusOptions, StatusShow};
use git_repository as git;
use git_repository::bstr::ByteSlice;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use time_humanize::HumanTime;

pub struct Repo<'a> {
    git2_repo: &'a Repository,
    repo: git::Repository,
    authors: Vec<Author>,
    total_num_authors: usize,
    num_commits: usize,
    time_of_most_recent_commit: git::actor::Time,
    time_of_first_commit: git::actor::Time,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Sig {
    name: git::bstr::BString,
    email: git::bstr::BString,
}

impl From<git::actor::Signature> for Sig {
    fn from(git::actor::Signature { name, email, .. }: git::actor::Signature) -> Self {
        Self { name, email }
    }
}

impl<'a> Repo<'a> {
    pub fn new(
        git2_repo: &'a Repository,
        no_merges: bool,
        bot_regex_pattern: &Option<Regex>,
        number_of_authors_to_display: usize,
    ) -> Result<Self> {
        let mut repo = git::open(git2_repo.path())?;

        // assure that objects we just traversed are coming from cache
        // when we read the commit right after.
        repo.object_cache_size(128 * 1024);

        let mut time_of_most_recent_commit = None;
        let mut time_of_first_commit = None;
        let mut commit_iter = repo.head_commit()?.ancestors().all().peekable();

        let mailmap = repo.load_mailmap();
        let mut author_to_number_of_commits: HashMap<Sig, usize> = HashMap::new();
        let mut total_nbr_of_commits = 0;

        let mut num_commits = 0;
        while let Some(commit_id) = commit_iter.next() {
            let commit: git::Commit = commit_id?
                .object()
                .expect("commit is still present/comes from cache")
                .into_commit();
            {
                let commit = commit.decode()?;
                if no_merges && commit.parents().take(2).count() > 1 {
                    continue;
                }

                if is_bot(commit.author, bot_regex_pattern) {
                    continue;
                }
                num_commits += 1;

                let author_nbr_of_commits = author_to_number_of_commits
                    .entry(Sig::from(mailmap.resolve(commit.author)))
                    .or_insert(0);
                *author_nbr_of_commits += 1;
                total_nbr_of_commits += 1;

                time_of_most_recent_commit.get_or_insert_with(|| commit.time());
                if commit_iter.peek().is_none() {
                    time_of_first_commit = commit.time().into();
                }
            }
        }

        let mut authors_by_number_of_commits: Vec<(Sig, usize)> =
            author_to_number_of_commits.into_iter().collect();

        let total_num_authors = authors_by_number_of_commits.len();
        authors_by_number_of_commits.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

        let authors: Vec<Author> = authors_by_number_of_commits
            .into_iter()
            .map(|(author, author_nbr_of_commits)| {
                let email = author.email;
                Author::new(
                    author.name,
                    email.into(),
                    author_nbr_of_commits,
                    total_nbr_of_commits,
                )
            })
            .take(number_of_authors_to_display)
            .collect();

        drop(commit_iter);
        Ok(Self {
            repo,
            git2_repo,
            authors,
            total_num_authors,
            num_commits,
            time_of_first_commit: time_of_first_commit.expect("at least one commit"),
            time_of_most_recent_commit: time_of_most_recent_commit.expect("at least one commit"),
        })
    }

    pub fn get_creation_date(&self, iso_time: bool) -> String {
        gitoxide_time_to_formatted_time(self.time_of_first_commit, iso_time)
    }

    pub fn get_number_of_commits(&self) -> String {
        self.num_commits.to_string()
    }

    pub fn take_authors(&mut self, show_email: bool) -> (Vec<Author>, usize) {
        if !show_email {
            for author in &mut self.authors {
                author.clear_email();
            }
        }
        (std::mem::take(&mut self.authors), self.total_num_authors)
    }

    pub fn get_date_of_last_commit(&self, iso_time: bool) -> String {
        gitoxide_time_to_formatted_time(self.time_of_most_recent_commit, iso_time)
    }

    // This collects the repo size excluding .git
    pub fn get_repo_size(&self) -> (String, u64) {
        let (repo_size, file_count) = match self.repo.load_index() {
            Some(Ok(index)) => {
                let repo_size = index.entries().iter().map(|e| e.stat.size as u128).sum();
                (repo_size, index.entries().len() as u64)
            }
            _ => (0, 0),
        };

        (bytes_to_human_readable(repo_size), file_count)
    }

    pub fn get_number_of_tags(&self) -> Result<usize> {
        Ok(self.repo.references()?.tags()?.count())
    }

    pub fn get_number_of_branches(&self) -> Result<usize> {
        let mut number_of_branches = self.repo.references()?.remote_branches()?.count();
        if number_of_branches > 0 {
            //Exclude origin/HEAD -> origin/main
            number_of_branches -= 1;
        }
        Ok(number_of_branches)
    }

    pub fn get_git_username(&self) -> Result<String> {
        let config = self.git2_repo.config()?;
        let username = match config.get_entry("user.name") {
            Ok(v) => v.value().unwrap_or("").into(),
            Err(_) => "".into(),
        };

        Ok(username)
    }

    pub fn get_version(&self) -> Result<String> {
        let mut version_name = String::new();
        let mut most_recent = 0;

        for tag in self
            .repo
            .references()?
            .tags()?
            .peeled()
            .filter_map(Result::ok)
        {
            if let Ok(commit) = tag.id().object()?.try_into_commit() {
                let current_time = commit.time()?.seconds();
                if current_time > most_recent {
                    most_recent = current_time;
                    version_name = tag.name().shorten().to_string();
                }
            }
        }
        Ok(version_name)
    }

    pub fn get_name_and_url(&self) -> Result<(String, String)> {
        let config = self.git2_repo.config()?;
        let mut remote_origin_url: Option<String> = None;
        let mut remote_url_fallback = String::new();
        let mut repository_name = String::new();
        let remote_regex = Regex::new(r"remote\.[a-zA-Z0-9]+\.url")?;

        for entry in &config.entries(None)? {
            let entry = entry?;
            let entry_name = entry.name().with_context(|| "Could not read entry name")?;
            if entry_name == "remote.origin.url" {
                remote_origin_url = Some(
                    entry
                        .value()
                        .with_context(|| "Could not read remote origin url")?
                        .to_string(),
                );
            } else if remote_regex.is_match(entry_name) {
                remote_url_fallback = entry
                    .value()
                    .with_context(|| "Could not read remote origin url fallback")?
                    .to_string()
            }
        }

        let remote_url = if let Some(url) = remote_origin_url {
            url
        } else {
            remote_url_fallback
        };

        let name_parts: Vec<&str> = remote_url.split('/').collect();

        if !name_parts.is_empty() {
            let mut i = 1;
            while repository_name.is_empty() && i <= name_parts.len() {
                repository_name = name_parts[name_parts.len() - i].to_string();
                i += 1;
            }
        }

        if repository_name.contains(".git") {
            let repo_name = repository_name.clone();
            let parts: Vec<&str> = repo_name.split(".git").collect();
            repository_name = parts[0].to_string();
        }

        Ok((repository_name, remote_url))
    }

    pub fn get_head_refs(&self) -> Result<HeadRefs> {
        let head_oid = self.repo.head_id().context("Could not read HEAD")?;
        let refs_info = self
            .repo
            .references()?
            .all()?
            .peeled()
            .filter_map(Result::ok)
            .filter_map(|reference: git::Reference<'_>| {
                (reference.id() == head_oid
                    && reference.name().category() != Some(git::reference::Category::Tag))
                .then(|| reference.name().shorten().to_string())
            })
            .collect();
        Ok(HeadRefs::new(head_oid.shorten()?.to_string(), refs_info))
    }
}

fn is_bot(author: git::actor::SignatureRef<'_>, bot_regex_pattern: &Option<Regex>) -> bool {
    bot_regex_pattern
        .as_ref()
        .map(|regex| regex.is_match(author.name.to_str_lossy().as_ref()))
        .unwrap_or(false)
}

fn bytes_to_human_readable(bytes: u128) -> String {
    let byte = Byte::from_bytes(bytes);
    byte.get_appropriate_unit(true).to_string()
}

fn gitoxide_time_to_formatted_time(time: git::actor::Time, iso_time: bool) -> String {
    if iso_time {
        to_rfc3339(HumanTime::from(time.seconds_since_unix_epoch as i64))
    } else {
        let ht = HumanTime::from_duration_since_timestamp(time.seconds_since_unix_epoch as u64);
        format!("{}", ht)
    }
}

fn to_rfc3339<T>(dt: T) -> String
where
    T: Into<OffsetDateTime>,
{
    dt.into().format(&Rfc3339).unwrap()
}

pub fn is_valid(repo_path: &str) -> Result<bool> {
    let repo = Repository::open_ext(repo_path, RepositoryOpenFlags::empty(), Vec::<&Path>::new());
    Ok(!repo?.is_bare())
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

pub fn get_pending_changes(repo: &git2::Repository) -> Result<String> {
    let statuses = repo.statuses(Some(
        StatusOptions::default()
            .show(StatusShow::Workdir)
            .update_index(true)
            .include_untracked(true)
            .renames_head_to_index(true)
            .recurse_untracked_dirs(true),
    ))?;

    let (added, deleted, modified) =
        statuses
            .iter()
            .fold((0, 0, 0), |(added, deleted, modified), e| {
                let s: Status = e.status();
                if s.is_index_new() || s.is_wt_new() {
                    (added + 1, deleted, modified)
                } else if s.is_index_deleted() || s.is_wt_deleted() {
                    (added, deleted + 1, modified)
                } else {
                    (added, deleted, modified + 1)
                }
            });

    let mut result = String::new();
    if modified > 0 {
        result = format!("{}+-", modified)
    }

    if added > 0 {
        result = format!("{} {}+", result, added);
    }

    if deleted > 0 {
        result = format!("{} {}-", result, deleted);
    }

    Ok(result.trim().into())
}
