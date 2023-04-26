use crate::cli::{MyRegex, NumberSeparator};
use crate::info::author::Author;
use anyhow::Result;
use gix::bstr::BString;
use gix::bstr::ByteSlice;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Commits {
    pub authors_to_display: Vec<Author>,
    pub total_number_of_authors: usize,
    pub total_number_of_commits: usize,
    /// false if we have found the first commit that started it all, true if the repository is shallow.
    pub is_shallow: bool,
    pub time_of_most_recent_commit: gix::actor::Time,
    pub time_of_first_commit: gix::actor::Time,
}

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

        let mailmap_config = repo.open_mailmap();
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        let mut count = 0;

        // From newest to oldest
        while let Some(commit_id) = commit_iter_peekable.next() {
            let commit = commit_id?.object()?.into_commit();
            let commit = commit.decode()?;

            if no_merges && commit.parents().take(2).count() > 1 {
                continue;
            }

            let sig = Sig::from(mailmap_config.resolve(commit.author));

            if is_bot(&sig.name, &bot_regex_pattern) {
                continue;
            }

            let author_nbr_of_commits = number_of_commits_by_signature.entry(sig).or_insert(0);
            *author_nbr_of_commits += 1;

            time_of_most_recent_commit.get_or_insert_with(|| commit.time());
            if commit_iter_peekable.peek().is_none() {
                time_of_first_commit = commit.time().into();
            }

            count += 1;
        }

        let (authors_to_display, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            count,
            number_of_authors_to_display,
            show_email,
            number_separator,
        );

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        drop(commit_iter);
        Ok(Self {
            authors_to_display,
            total_number_of_authors,
            total_number_of_commits: count,
            is_shallow: repo.is_shallow(),
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }
}

fn compute_authors(
    number_of_commits_by_signature: HashMap<Sig, usize>,
    total_number_of_commits: usize,
    number_of_authors_to_display: usize,
    show_email: bool,
    number_separator: NumberSeparator,
) -> (Vec<Author>, usize) {
    let total_number_of_authors = number_of_commits_by_signature.len();
    let mut signature_with_number_of_commits_sorted: Vec<(Sig, usize)> =
        number_of_commits_by_signature.into_iter().collect();

    signature_with_number_of_commits_sorted.sort_by(|(sa, a_count), (sb, b_count)| {
        b_count.cmp(a_count).then_with(|| sa.name.cmp(&sb.name))
    });

    let authors: Vec<Author> = signature_with_number_of_commits_sorted
        .into_iter()
        .map(|(author, author_nbr_of_commits)| {
            let email = author.email;
            Author::new(
                author.name,
                email,
                author_nbr_of_commits,
                total_number_of_commits,
                show_email,
                number_separator,
            )
        })
        .take(number_of_authors_to_display)
        .collect();
    (authors, total_number_of_authors)
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

fn is_bot(author_name: &BString, bot_regex_pattern: &Option<MyRegex>) -> bool {
    bot_regex_pattern
        .as_ref()
        .map(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_no_bots_regex() {
        // Test case 1: no_bots is None
        let no_bots: Option<Option<MyRegex>> = None;
        let result = get_no_bots_regex(&no_bots).unwrap();
        assert_eq!(result, None);

        // Test case 2: no_bots is Some(None)
        let no_bots: Option<Option<MyRegex>> = Some(None);
        let result = get_no_bots_regex(&no_bots).unwrap();
        assert_eq!(result.unwrap().0.as_str(), "(b|B)ot");

        // Test case 3: no_bots is Some(Some(regex))
        let regex = MyRegex(Regex::new(r"foo").unwrap());
        let no_bots: Option<Option<MyRegex>> = Some(Some(regex));
        let result = get_no_bots_regex(&no_bots).unwrap();
        assert_eq!(result.unwrap().0.as_str(), "foo");
    }

    #[test]
    fn test_compute_authors() {
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        number_of_commits_by_signature.insert(
            Sig {
                name: "John Doe".into(),
                email: "johndoe@example.com".into(),
            },
            10,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Jane Doe".into(),
                email: "janedoe@example.com".into(),
            },
            5,
        );
        number_of_commits_by_signature.insert(
            Sig {
                name: "Ellen Smith".into(),
                email: "ellensmith@example.com".into(),
            },
            50,
        );
        let total_number_of_commits = 15;
        let number_of_authors_to_display = 2;
        let show_email = false;
        let number_separator = NumberSeparator::Comma;

        let (authors, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
            number_separator,
        );

        assert_eq!(total_number_of_authors, 3);
        assert_eq!(authors.len(), 2);
        assert_eq!(authors.get(0).unwrap().name, "Ellen Smith".to_string());
    }
}
