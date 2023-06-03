use crate::cli::{MyRegex, NumberSeparator};
use crate::info::author::Author;
use crate::info::churn::Churn;
use anyhow::Result;
use gix::bstr::ByteSlice;
use gix::bstr::{BString, Utf8Error};
use gix::object::tree::diff::change::Event;
use gix::object::tree::diff::{Action, Change};
use gix::objs::tree::EntryMode;
use gix::Commit;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct CommitMetrics {
    pub authors_to_display: Vec<Author>,
    pub churns_to_display: Vec<Churn>,
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

impl CommitMetrics {
    pub fn new(
        repo: &gix::Repository,
        no_merges: bool,
        no_bots: &Option<Option<MyRegex>>,
        number_of_authors_to_display: usize,
        show_email: bool,
        number_separator: NumberSeparator,
    ) -> Result<Self> {
        let bot_regex_pattern = get_no_bots_regex(no_bots)?;
        let mut time_of_most_recent_commit = None;
        let mut time_of_first_commit = None;
        let mut commit_iter = repo.head_commit()?.ancestors().all()?;
        let mut commit_iter_peekable = commit_iter.by_ref().peekable();

        let mailmap_config = repo.open_mailmap();
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        let mut total_number_of_commits = 0;
        let mut diff_count = 0;
        let mut number_of_commits_by_file_path: HashMap<String, usize> = HashMap::new();

        // From newest to oldest
        while let Some(commit_id) = commit_iter_peekable.next() {
            let commit = commit_id?.object()?.into_commit();

            if no_merges && commit.parent_ids().count() > 1 {
                continue;
            }

            let sig = Sig::from(mailmap_config.resolve(commit.author()?));

            if is_bot(&sig.name, &bot_regex_pattern) {
                continue;
            }

            *number_of_commits_by_signature.entry(sig).or_insert(0) += 1;

            if diff_count <= 100 {
                compute_diff_with_parents(
                    &mut number_of_commits_by_file_path,
                    &commit,
                    repo,
                    &mut diff_count,
                )?;
            }

            let commit_time = commit
                .time()
                .expect("Could not read commit's creation time");

            time_of_most_recent_commit.get_or_insert(commit_time);

            if commit_iter_peekable.peek().is_none() {
                time_of_first_commit = commit_time.into();
            }

            total_number_of_commits += 1;
        }

        let (authors_to_display, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            total_number_of_commits,
            number_of_authors_to_display,
            show_email,
            number_separator,
        );

        let churns_to_display = compute_churns(number_of_commits_by_file_path, number_separator);

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        Ok(Self {
            authors_to_display,
            churns_to_display,
            total_number_of_authors,
            total_number_of_commits,
            is_shallow: repo.is_shallow(),
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }
}

fn compute_churns(
    number_of_commits_by_file_path: HashMap<String, usize>,
    number_separator: NumberSeparator,
) -> Vec<Churn> {
    let mut number_of_commits_by_file_path_sorted: Vec<(String, usize)> =
        number_of_commits_by_file_path.into_iter().collect();

    number_of_commits_by_file_path_sorted
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    number_of_commits_by_file_path_sorted
        .into_iter()
        .map(|(file_path, nbr_of_commits)| Churn::new(file_path, nbr_of_commits, number_separator))
        .take(3)
        .collect()
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
            Author::new(
                author.name.to_string(),
                if show_email {
                    Some(author.email.to_string())
                } else {
                    None
                },
                author_nbr_of_commits,
                total_number_of_commits,
                number_separator,
            )
        })
        .take(number_of_authors_to_display)
        .collect();
    (authors, total_number_of_authors)
}

fn compute_diff_with_parents(
    change_map: &mut HashMap<String, usize>,
    commit: &Commit,
    repo: &gix::Repository,
    diff_count: &mut usize,
) -> Result<()> {
    // Handles the very first commit
    if commit.parent_ids().count() == 0 {
        repo.empty_tree()
            .changes()?
            .track_path()
            .for_each_to_obtain_tree(&commit.tree()?, |change| {
                for_each_change(change, change_map)
            })?;
    }
    for parent_id in commit.parent_ids() {
        parent_id
            .object()?
            .into_commit()
            .tree()?
            .changes()?
            .track_path()
            .for_each_to_obtain_tree(&commit.tree()?, |change| {
                for_each_change(change, change_map)
            })?;
        *diff_count += 1;
    }

    Ok(())
}

fn for_each_change(
    change: Change,
    change_map: &mut HashMap<String, usize>,
) -> Result<Action, Utf8Error> {
    let is_file_change = match change.event {
        Event::Addition { entry_mode, .. } | Event::Modification { entry_mode, .. } => {
            entry_mode == EntryMode::Blob
        }
        Event::Deletion { .. } | Event::Rewrite { .. } => false,
    };
    if is_file_change {
        let path = change.location.to_os_str()?.to_string_lossy();
        *change_map.entry(path.into_owned()).or_insert(0) += 1;
    }

    Ok::<Action, Utf8Error>(Action::Continue)
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
    bot_regex_pattern.as_ref().map_or(false, |regex| {
        regex.0.is_match(author_name.to_str_lossy().as_ref())
    })
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
