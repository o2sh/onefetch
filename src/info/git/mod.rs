use self::metrics::GitMetrics;
use self::sig::Sig;
use crate::cli::MyRegex;
use anyhow::Result;
use gix::bstr::ByteSlice;
use gix::bstr::{BString, Utf8Error};
use gix::object::tree::diff::change::Event;
use gix::object::tree::diff::Action;
use gix::prelude::ObjectIdExt;
use gix::traverse::commit::simple::Sorting;
use gix::{Commit, ObjectId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

pub mod metrics;
pub mod sig;

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    no_bots: Option<MyRegex>,
    max_churn_pool_size: Option<usize>,
    no_merges: bool,
) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
    let mailmap = repo.open_mailmap();
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
        &mailmap,
        no_bots.clone(),
        &has_commit_graph_traversal_ended,
        &total_number_of_commits,
        max_churn_pool_size,
    )?;

    let author_threads = can_use_author_threads
        .then(|| get_author_channel(repo, num_threads, no_bots.clone(), &mailmap));

    let mut count = 0;
    for commit in commit_iter {
        let commit = commit?;
        {
            if no_merges && commit.parent_ids.len() > 1 {
                continue;
            }

            if let Some((_threads, author_tx)) = author_threads.as_ref() {
                author_tx.send(commit.id)?;
            } else {
                update_signature_counts(
                    &commit.object()?,
                    &mailmap,
                    no_bots.as_ref(),
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

    drop(churn_tx);

    let (number_of_commits_by_file_path, churn_pool_size) =
        churn_thread.join().expect("never panics")?;

    let git_metrics = GitMetrics::new(
        number_of_commits_by_signature,
        number_of_commits_by_file_path,
        churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
    )?;

    Ok(git_metrics)
}

type NumberOfCommitsBySignature = HashMap<Sig, usize>;

fn get_author_channel(
    repo: &gix::Repository,
    num_threads: usize,
    bot_regex_pattern: Option<MyRegex>,
    mailmap: &gix::mailmap::Snapshot,
) -> (
    Vec<JoinHandle<Result<NumberOfCommitsBySignature>>>,
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
                    let mut number_of_commits_by_signature = NumberOfCommitsBySignature::new();
                    // We are sure to see each object only once.
                    repo.object_cache_size(0);
                    while let Ok(commit_id) = rx.recv() {
                        let commit = repo.find_object(commit_id)?.into_commit();
                        update_signature_counts(
                            &commit,
                            &mailmap,
                            bot_regex_pattern.as_ref(),
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

type NumberOfCommitsByFilepath = HashMap<BString, usize>;
type ChurnPair = (NumberOfCommitsByFilepath, usize);

fn get_churn_channel(
    repo: &gix::Repository,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex_pattern: Option<MyRegex>,
    has_commit_graph_traversal_ended: &Arc<AtomicBool>,
    total_number_of_commits: &Arc<AtomicUsize>,
    max_churn_pool_size: Option<usize>,
) -> Result<(JoinHandle<Result<ChurnPair>>, Sender<ObjectId>)> {
    let (tx, rx) = channel::<gix::hash::ObjectId>();
    let thread = std::thread::spawn({
        let repo = repo.clone();
        let mailmap = mailmap.clone();
        let bot_regex_pattern = bot_regex_pattern.clone();
        let has_commit_graph_traversal_ended = has_commit_graph_traversal_ended.clone();
        let total_number_of_commits = total_number_of_commits.clone();
        move || -> Result<_> {
            let mut number_of_commits_by_file_path = NumberOfCommitsByFilepath::new();
            let mut number_of_diffs_computed = 0;
            while let Ok(commit_id) = rx.recv() {
                let commit = repo.find_object(commit_id)?.into_commit();
                if is_bot_commit(&commit, &mailmap, bot_regex_pattern.as_ref())? {
                    continue;
                }
                compute_diff_with_parent(&mut number_of_commits_by_file_path, &commit, &repo)?;
                number_of_diffs_computed += 1;
                if should_break(
                    has_commit_graph_traversal_ended.load(Ordering::Relaxed),
                    total_number_of_commits.load(Ordering::Relaxed),
                    max_churn_pool_size,
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
    max_churn_pool_size_opt: Option<usize>,
    number_of_diffs_computed: usize,
) -> bool {
    if !has_commit_graph_traversal_ended {
        return false;
    }

    max_churn_pool_size_opt.map_or(true, |max_churn_pool_size| {
        number_of_diffs_computed >= max_churn_pool_size.min(total_number_of_commits)
    })
}

fn update_signature_counts(
    commit: &gix::Commit,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex_pattern: Option<&MyRegex>,
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
                        entry_mode.is_blob()
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

fn is_bot_commit(
    commit: &Commit,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex_pattern: Option<&MyRegex>,
) -> Result<bool> {
    if bot_regex_pattern.is_some() {
        let sig = mailmap.resolve(commit.author()?);
        Ok(is_bot(&sig.name, bot_regex_pattern))
    } else {
        Ok(false)
    }
}

fn is_bot(author_name: &BString, bot_regex_pattern: Option<&MyRegex>) -> bool {
    bot_regex_pattern.map_or(false, |regex| {
        regex.0.is_match(author_name.to_str_lossy().as_ref())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::NO_BOTS_DEFAULT_REGEX_PATTERN;
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case("John Doe", false)]
    #[case("dependabot[bot]", true)]
    #[case("foo bot", true)]
    #[case("foo-bot", true)]
    #[case("bot", false)]
    fn test_is_bot(#[case] author_name: &str, #[case] expected: bool) -> Result<()> {
        let from_str = MyRegex::from_str(NO_BOTS_DEFAULT_REGEX_PATTERN);
        let no_bots: Option<MyRegex> = Some(from_str?);
        assert_eq!(is_bot(&author_name.into(), no_bots.as_ref()), expected);
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
