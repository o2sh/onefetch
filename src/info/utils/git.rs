use crate::cli::{CliOptions, MyRegex, NumberSeparator};
use crate::info::author::Author;
use crate::info::churn::FileChurn;
use anyhow::Result;
use gix::bstr::ByteSlice;
use gix::bstr::{BString, Utf8Error};
use gix::object::tree::diff::change::Event;
use gix::object::tree::diff::Action;
use gix::objs::tree::EntryMode;
use gix::prelude::ObjectIdExt;
use gix::traverse::commit::Sorting;
use gix::Commit;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

pub struct CommitMetrics {
    pub authors_to_display: Vec<Author>,
    pub file_churns_to_display: Vec<FileChurn>,
    pub total_number_of_authors: usize,
    pub total_number_of_commits: usize,
    pub churn_pool_size: usize,
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
    pub fn new(repo: &gix::Repository, options: &CliOptions) -> Result<Self> {
        let bot_regex_pattern = get_no_bots_regex(&options.info.no_bots)?;
        let mut time_of_most_recent_commit = None;
        let mut time_of_first_commit = None;

        let num_threads = std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1);
        let commit_graph = repo.commit_graph().ok();
        let can_use_author_threads = num_threads > 1 && commit_graph.is_some();
        let commit_iter = repo
            .head_commit()?
            .id()
            .ancestors()
            .sorting(Sorting::ByCommitTimeNewestFirst)
            .use_commit_graph(can_use_author_threads)
            .with_commit_graph(commit_graph)
            .all()?;

        let mailmap = repo.open_mailmap();
        let author_threads = can_use_author_threads.then(|| {
            // we intentionally over-allocate threads a little as the main thread won't be very busy anyway
            // traversing commits with the graph available.
            // The channel is generously bounded to assure all threads are fed, without consuming excessive memory.
            // We have to wait for the threads to finish anyway so some synchronization here will be fine, while tests
            // show that this is about as fast as if it was unbounded.
            let (tx, rx) = crossbeam_channel::bounded(num_threads * 100);
            let threads: Vec<_> = (0..num_threads)
                .map(|_| {
                    std::thread::spawn({
                        let mut repo = repo.clone();
                        let mailmap = mailmap.clone();
                        let bot_regex_pattern = bot_regex_pattern.clone();
                        let rx = rx.clone();
                        move || -> anyhow::Result<_> {
                            let mut number_of_commits_by_signature: HashMap<Sig, usize> =
                                HashMap::new();
                            // We are sure to see each object only once.
                            repo.object_cache_size(0);
                            for commit_id in rx {
                                if let Some(commit) =
                                    repo.try_find_object(commit_id)?.map(|c| c.into_commit())
                                {
                                    let sig = mailmap.resolve(commit.author()?);
                                    if !is_bot(&sig.name, &bot_regex_pattern) {
                                        *number_of_commits_by_signature
                                            .entry(sig.into())
                                            .or_insert(0) += 1;
                                    }
                                }
                            }
                            Ok(number_of_commits_by_signature)
                        }
                    })
                })
                .collect();
            (threads, tx)
        });
        let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
        let (sender, receiver) = std::sync::mpsc::channel::<gix::hash::ObjectId>();
        let has_graph_commit_traversal_ended = Arc::new(AtomicBool::default());
        let total_number_of_commits = Arc::new(AtomicUsize::default());

        let churn_results = std::thread::spawn({
            let repo = repo.clone();
            let has_commit_graph_traversal_ended = has_graph_commit_traversal_ended.clone();
            let total_number_of_commits = total_number_of_commits.clone();
            let churn_pool_size_opt = options.info.churn_pool_size;
            move || -> Result<_> {
                let mut number_of_commits_by_file_path: HashMap<BString, usize> = HashMap::new();
                let mut number_of_diffs_computed = 0;
                while let Ok(commit_id) = receiver.recv() {
                    let commit = repo.find_object(commit_id)?.into_commit();
                    compute_diff_with_parent(&mut number_of_commits_by_file_path, &commit, &repo)?;
                    number_of_diffs_computed += 1;
                    if should_break(
                        has_commit_graph_traversal_ended.load(Ordering::Relaxed),
                        total_number_of_commits.load(Ordering::Relaxed),
                        churn_pool_size_opt,
                        number_of_diffs_computed,
                    ) {
                        break;
                    }
                }

                Ok((number_of_commits_by_file_path, number_of_diffs_computed))
            }
        });

        let mut count = 0;
        // From newest to oldest
        for commit in commit_iter {
            let commit = commit?;
            {
                if options.info.no_merges && commit.parent_ids.len() > 1 {
                    continue;
                }

                if let Some((_threads, send_commit)) = author_threads.as_ref() {
                    send_commit.send(commit.id)?;
                } else {
                    let sig = mailmap.resolve(commit.object()?.author()?);
                    if !is_bot(&sig.name, &bot_regex_pattern) {
                        *number_of_commits_by_signature
                            .entry(sig.into())
                            .or_insert(0) += 1;
                    }
                }
                let commit_time = gix::actor::Time::new(
                    commit
                        .commit_time
                        .expect("sorting by time yields this field as part of traversal")
                        as u32, // TODO: remove this cast once `gix` supports 64 bit dates.
                    0,
                );
                time_of_most_recent_commit.get_or_insert(commit_time);
                time_of_first_commit = commit_time.into();

                count += 1;
            }

            sender.send(commit.id).ok();
        }

        total_number_of_commits.store(count, Ordering::SeqCst);

        if let Some((threads, sender)) = author_threads {
            drop(sender);
            for thread in threads {
                let mapping = thread.join().expect("no panic")?;
                for (sig, num_commits) in mapping {
                    *number_of_commits_by_signature.entry(sig).or_insert(0) += num_commits;
                }
            }
        }

        has_graph_commit_traversal_ended.store(true, Ordering::SeqCst);
        let (authors_to_display, total_number_of_authors) = compute_authors(
            number_of_commits_by_signature,
            count,
            options.info.number_of_authors,
            options.info.email,
            options.text_formatting.number_separator,
        );

        let (number_of_commits_by_file_path, churn_pool_size) =
            churn_results.join().expect("never panics")?;

        let file_churns_to_display = compute_file_churns(
            number_of_commits_by_file_path,
            options.info.number_of_file_churns,
            options.text_formatting.number_separator,
        );

        // This could happen if a branch pointed to non-commit object, so no traversal actually happens.
        let (time_of_first_commit, time_of_most_recent_commit) = time_of_first_commit
            .and_then(|a| time_of_most_recent_commit.map(|b| (a, b)))
            .unwrap_or_default();

        Ok(Self {
            authors_to_display,
            file_churns_to_display,
            total_number_of_authors,
            total_number_of_commits: count,
            churn_pool_size,
            is_shallow: repo.is_shallow(),
            time_of_first_commit,
            time_of_most_recent_commit,
        })
    }
}

fn should_break(
    has_commit_graph_traversal_ended: bool,
    total_number_of_commits: usize,
    churn_pool_size_opt: Option<usize>,
    number_of_diffs_computed: usize,
) -> bool {
    if !has_commit_graph_traversal_ended {
        return false;
    }
    churn_pool_size_opt.map_or(true, |churn_pool_size| {
        number_of_diffs_computed == churn_pool_size.min(total_number_of_commits)
    })
}

fn compute_file_churns(
    number_of_commits_by_file_path: HashMap<BString, usize>,
    number_of_file_churns_to_display: usize,
    number_separator: NumberSeparator,
) -> Vec<FileChurn> {
    let mut number_of_commits_by_file_path_sorted = Vec::from_iter(number_of_commits_by_file_path);

    number_of_commits_by_file_path_sorted
        .sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

    number_of_commits_by_file_path_sorted
        .into_iter()
        .map(|(file_path, nbr_of_commits)| {
            FileChurn::new(file_path.to_string(), nbr_of_commits, number_separator)
        })
        .take(number_of_file_churns_to_display)
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

fn compute_diff_with_parent(
    change_map: &mut HashMap<BString, usize>,
    commit: &Commit,
    repo: &gix::Repository,
) -> Result<()> {
    let mut parents = commit.parent_ids();
    let parents = (
        parents
            .next()
            .and_then(|parent_id| parent_id.object().ok()?.into_commit().tree_id().ok())
            .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()).attach(repo)),
        parents.next(),
    );

    if let (tree_id, None) = parents {
        tree_id
            .object()?
            .into_tree()
            .changes()?
            .track_path()
            .for_each_to_obtain_tree(&commit.tree()?, |change| {
                let is_file_change = match change.event {
                    Event::Addition { entry_mode, .. } | Event::Modification { entry_mode, .. } => {
                        entry_mode == EntryMode::Blob || entry_mode == EntryMode::BlobExecutable
                    }
                    Event::Deletion { .. } | Event::Rewrite { .. } => false,
                };
                if is_file_change {
                    let path = change.location;
                    *change_map.entry(path.to_owned()).or_insert(0) += 1;
                }

                Ok::<Action, Utf8Error>(Action::Continue)
            })?;
    }

    Ok(())
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
    use rstest::rstest;

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

    #[test]
    fn test_compute_file_churns() {
        let mut number_of_commits_by_file_path = HashMap::new();
        number_of_commits_by_file_path.insert("path/to/file1.txt".into(), 2);
        number_of_commits_by_file_path.insert("path/to/file2.txt".into(), 5);
        number_of_commits_by_file_path.insert("path/to/file3.txt".into(), 3);
        number_of_commits_by_file_path.insert("path/to/file4.txt".into(), 7);

        let number_of_file_churns_to_display = 3;
        let number_separator = NumberSeparator::Comma;
        let file_churns = compute_file_churns(
            number_of_commits_by_file_path,
            number_of_file_churns_to_display,
            number_separator,
        );

        assert_eq!(file_churns.len(), 3);
        assert_eq!(
            file_churns.get(0).unwrap().file_path,
            "path/to/file4.txt".to_string()
        );
        assert_eq!(file_churns.get(0).unwrap().nbr_of_commits, 7);
    }

    #[rstest]
    #[case(true, 10, Some(8), 4, false)]
    #[case(false, 10, Some(10), 10, false)]
    #[case(true, 10, Some(5), 5, true)]
    #[case(true, 5, Some(10), 5, true)]
    #[case(true, 5, Some(10), 3, false)]
    #[case(true, 10, Some(5), 3, false)]
    fn test_should_break(
        #[case] has_commit_graph_traversal_ended: bool,
        #[case] total_number_of_commits: usize,
        #[case] churn_pool_size_opt: Option<usize>,
        #[case] number_of_diffs_computed: usize,
        #[case] expected: bool,
    ) {
        let result = should_break(
            has_commit_graph_traversal_ended,
            total_number_of_commits,
            churn_pool_size_opt,
            number_of_diffs_computed,
        );

        assert_eq!(result, expected);
    }
}
