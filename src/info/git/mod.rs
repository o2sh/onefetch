use self::metrics::GitMetrics;
use self::sig::Sig;
use crate::cli::{CliOptions, MyRegex};
use anyhow::Result;
use gix::bstr::ByteSlice;
use gix::bstr::{BString, Utf8Error};
use gix::object::tree::diff::change::Event;
use gix::object::tree::diff::Action;
use gix::objs::tree::EntryMode;
use gix::prelude::ObjectIdExt;
use gix::traverse::commit::Sorting;
use gix::{Commit, ObjectId};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

pub mod metrics;
mod sig;

pub fn traverse_commit_graph(repo: &gix::Repository, options: &CliOptions) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
    let mailmap = repo.open_mailmap();
    let bot_regex_pattern = get_no_bots_regex(&options.info.no_bots)?;
    let has_commit_graph_traversal_ended = Arc::new(AtomicBool::default());
    let total_number_of_commits = Arc::new(AtomicUsize::default());

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

    let (churn_thread, churn_tx) = get_churn_channel(
        repo,
        &has_commit_graph_traversal_ended,
        &total_number_of_commits,
        options.info.churn_pool_size,
    )?;

    let author_threads = can_use_author_threads
        .then(|| get_author_channel(repo, num_threads, &bot_regex_pattern, &mailmap));

    let mut count = 0;
    for commit in commit_iter {
        let commit = commit?;
        {
            if options.info.no_merges && commit.parent_ids.len() > 1 {
                continue;
            }

            if let Some((_threads, author_tx)) = author_threads.as_ref() {
                author_tx.send(commit.id)?;
            } else {
                update_signature_counts(
                    &commit.object()?,
                    &mailmap,
                    &bot_regex_pattern,
                    &mut number_of_commits_by_signature,
                )?;
            }

            churn_tx.send(commit.id)?;

            let commit_time = gix::date::Time::new(
                commit
                    .commit_time
                    .expect("sorting by time yields this field as part of traversal"),
                0,
            );
            time_of_most_recent_commit.get_or_insert(commit_time);
            time_of_first_commit = commit_time.into();

            count += 1;
        }
    }

    if let Some((threads, sender)) = author_threads {
        drop(sender);
        for thread in threads {
            let mapping = thread.join().expect("never panics")?;
            for (sig, num_commits) in mapping {
                *number_of_commits_by_signature.entry(sig).or_insert(0) += num_commits;
            }
        }
    }

    total_number_of_commits.store(count, Ordering::SeqCst);
    has_commit_graph_traversal_ended.store(true, Ordering::SeqCst);

    let (number_of_commits_by_file_path, churn_pool_size) =
        churn_thread.join().expect("never panics")?;

    let git_metrics = GitMetrics::new(
        number_of_commits_by_signature,
        number_of_commits_by_file_path,
        churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
        options,
    )?;

    Ok(git_metrics)
}

fn get_author_channel(
    repo: &gix::Repository,
    num_threads: usize,
    bot_regex_pattern: &Option<MyRegex>,
    mailmap: &gix::mailmap::Snapshot,
) -> (
    Vec<JoinHandle<Result<HashMap<Sig, usize>>>>,
    crossbeam_channel::Sender<ObjectId>,
) {
    // we intentionally over-allocate threads a little as the main thread won't be very busy anyway
    // traversing commits with the graph available.
    // The channel is generously bounded to assure all threads are fed, without consuming excessive memory.
    // We have to wait for the threads to finish anyway so some synchronization here will be fine, while tests
    // show that this is about as fast as if it was unbounded.
    let (tx, rx) = crossbeam_channel::bounded::<ObjectId>(num_threads * 100);
    let threads: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn({
                let mut repo = repo.clone();
                let mailmap = mailmap.clone();
                let bot_regex_pattern = bot_regex_pattern.clone();
                let rx = rx.clone();
                move || -> anyhow::Result<_> {
                    let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
                    // We are sure to see each object only once.
                    repo.object_cache_size(0);
                    while let Ok(commit_id) = rx.recv() {
                        let commit = repo.find_object(commit_id)?.into_commit();
                        update_signature_counts(
                            &commit,
                            &mailmap,
                            &bot_regex_pattern,
                            &mut number_of_commits_by_signature,
                        )?;
                    }
                    Ok(number_of_commits_by_signature)
                }
            })
        })
        .collect();
    (threads, tx)
}

fn get_churn_channel(
    repo: &gix::Repository,
    has_commit_graph_traversal_ended: &Arc<AtomicBool>,
    total_number_of_commits: &Arc<AtomicUsize>,
    churn_pool_size_opt: Option<usize>,
) -> Result<(
    JoinHandle<Result<(HashMap<BString, usize>, usize)>>,
    Sender<ObjectId>,
)> {
    let (tx, rx) = channel::<gix::hash::ObjectId>();
    let thread = std::thread::spawn({
        let repo = repo.clone();
        let has_commit_graph_traversal_ended = has_commit_graph_traversal_ended.clone();
        let total_number_of_commits = total_number_of_commits.clone();
        move || -> Result<_> {
            let mut number_of_commits_by_file_path: HashMap<BString, usize> = HashMap::new();
            let mut number_of_diffs_computed = 0;
            while let Ok(commit_id) = rx.recv() {
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

    Ok((thread, tx))
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
        number_of_diffs_computed >= churn_pool_size.min(total_number_of_commits)
    })
}

fn update_signature_counts(
    commit: &gix::Commit,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex_pattern: &Option<MyRegex>,
    number_of_commits_by_signature: &mut HashMap<Sig, usize>,
) -> Result<()> {
    let sig = mailmap.resolve(commit.author()?);
    if !is_bot(&sig.name, bot_regex_pattern) {
        *number_of_commits_by_signature
            .entry(sig.into())
            .or_insert(0) += 1;
    }
    Ok(())
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
            .track_rewrites(None)
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
            None => Some(MyRegex(Regex::from_str(r"(?:-|\s)[Bb]ot$|\[[Bb]ot\]")?)),
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
    fn test_get_no_bots_regex() -> Result<()> {
        // Test case 1: no_bots is None
        let no_bots: Option<Option<MyRegex>> = None;
        let result = get_no_bots_regex(&no_bots)?;
        assert_eq!(result, None);

        // Test case 2: no_bots is Some(None)
        let no_bots: Option<Option<MyRegex>> = Some(None);
        let result = get_no_bots_regex(&no_bots)?;
        assert_eq!(result.unwrap().0.as_str(), r"(?:-|\s)[Bb]ot$|\[[Bb]ot\]");

        // Test case 3: no_bots is Some(Some(regex))
        let regex = MyRegex(Regex::new(r"foo")?);
        let no_bots: Option<Option<MyRegex>> = Some(Some(regex));
        let result = get_no_bots_regex(&no_bots)?;
        assert_eq!(result.unwrap().0.as_str(), "foo");

        Ok(())
    }

    #[rstest]
    #[case("John Doe", false)]
    #[case("dependabot[bot]", true)]
    #[case("foo bot", true)]
    #[case("foo-bot", true)]
    #[case("bot", false)]
    fn test_is_bot(#[case] author_name: &str, #[case] expected: bool) -> Result<()> {
        let no_bots: Option<Option<MyRegex>> = Some(None);
        let regex = get_no_bots_regex(&no_bots)?;
        assert_eq!(is_bot(&author_name.into(), &regex), expected);
        Ok(())
    }

    #[rstest]
    #[case(true, 10, Some(8), 4, false)]
    #[case(false, 10, Some(10), 10, false)]
    #[case(true, 10, Some(5), 5, true)]
    #[case(true, 5, Some(10), 5, true)]
    #[case(true, 5, Some(10), 3, false)]
    #[case(true, 10, Some(5), 3, false)]
    #[case(true, 100, Some(30), 90, true)]
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
