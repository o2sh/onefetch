use crate::cli::{MyRegex, NumberSeparator};
use crate::info::author::Author;
use anyhow::Result;
use gix::bstr::BStr;
use gix::bstr::BString;
use gix::bstr::ByteSlice;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq)]
pub struct Sig {
    name: gix::bstr::BString,
    email: gix::bstr::BString,
}

impl From<gix::actor::Signature> for Sig {
    fn from(gix::actor::Signature { name, email, .. }: gix::actor::Signature) -> Self {
        Self { name, email }
    }
}

pub struct Commits {
    pub authors: Vec<Author>,
    pub total_num_authors: usize,
    pub num_commits: usize,
    /// false if we have found the first commit that started it all, true if the repository is shallow.
    pub is_shallow: bool,
    pub time_of_most_recent_commit: gix::actor::Time,
    pub time_of_first_commit: gix::actor::Time,
}

impl Commits {
    pub fn new(
        mut repo: gix::Repository,
        no_merges: bool,
        no_bots: &Option<Option<MyRegex>>,
        number_of_authors_to_display: usize,
        show_email: bool,
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        // assure that objects we just traversed are coming from cache
        // when we read the commit right after.
        repo.object_cache_size(32 * 1024);

        let bot_regex_pattern = get_no_bots_regex(no_bots)?;
        let mut time_of_most_recent_commit = None;
        let mut time_of_first_commit = None;
        let mut commit_iter = repo.head_commit()?.ancestors().all()?;
        let mut commit_iter_peekable = commit_iter.by_ref().peekable();

        let mut email_to_names: HashMap<BString, HashMap<BString, usize>> = HashMap::new();
        let mut total_nbr_of_commits = 0;

        let mut num_commits = 0;
        while let Some(commit_id) = commit_iter_peekable.next() {
            let commit = commit_id?.object()?.into_commit();
            let commit = commit.decode()?;

            if no_merges && commit.parents().take(2).count() > 1 {
                continue;
            }

            if is_bot(&commit.author.name, &bot_regex_pattern) {
                continue;
            }

            num_commits += 1;

            let email_names = email_to_names
                .entry(BString::from(commit.author.email))
                .or_insert(HashMap::new());

            let name_nbr_of_commits = (*email_names)
                .entry(BString::from(commit.author.name))
                .or_insert(0);
            *name_nbr_of_commits += 1;
            total_nbr_of_commits += 1;

            time_of_most_recent_commit.get_or_insert_with(|| commit.time());
            if commit_iter_peekable.peek().is_none() {
                time_of_first_commit = commit.time().into();
            }
        }

        let mut authors_by_number_of_commits: Vec<(Sig, usize)> = email_to_names
            .into_iter()
            .map(|(email, name_to_nbr_of_commits)| {
                let mut author_nbr_of_commits = 0;

                let most_frequent_name = name_to_nbr_of_commits
                    .into_iter()
                    .fold(None, |acc, x| {
                        author_nbr_of_commits += x.1;

                        match acc {
                            None => Some(x),
                            Some(e) => {
                                if x.1 > e.1 {
                                    Some(x)
                                } else {
                                    Some(e)
                                }
                            }
                        }
                    })
                    .unwrap();

                (
                    Sig {
                        email,
                        name: most_frequent_name.0.clone(),
                    },
                    author_nbr_of_commits,
                )
            })
            .collect();

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
                    number_separator,
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

fn get_no_bots_regex(no_bots: &Option<Option<MyRegex>>) -> Result<Option<MyRegex>> {
    let reg = if let Some(r) = no_bots.clone() {
        match r {
            Some(p) => Some(p),
            None => Some(MyRegex(Regex::from_str(r"(b|B)ot")?)),
        }
    } else {
        None
    };

    Ok(reg)
}

fn is_bot(author_name: &BStr, bot_regex_pattern: &Option<MyRegex>) -> bool {
    bot_regex_pattern
        .as_ref()
        .map(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
        .unwrap_or(false)
}
