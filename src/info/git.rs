use super::repo::author::Author;
use crate::cli::{MyRegex, ThousandsSeparator};
use anyhow::Result;
use git::bstr::BString;
use git_repository as git;
use git_repository::bstr::ByteSlice;
use std::collections::HashMap;

pub struct Commits {
    pub authors: Vec<Author>,
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
        thousands_separator: Option<&ThousandsSeparator>,
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
                    thousands_separator,
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
}

fn is_bot(author_name: &BString, bot_regex_pattern: &Option<MyRegex>) -> bool {
    bot_regex_pattern
        .as_ref()
        .map(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
        .unwrap_or(false)
}
