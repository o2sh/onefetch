use self::metrics::GitMetrics;
use self::sig::Sig;
use crate::cli::MyRegex;
use anyhow::Result;
use gix::bstr::BString;
use gix::bstr::ByteSlice;
use gix::diff::Options;
use gix::diff::tree_with_rewrites::Change;
use gix::prelude::ObjectIdExt;
use gix::revision::walk::Sorting;
use gix::traverse::commit::simple::CommitTimeOrder;
use gix::{Commit, ObjectId};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread::JoinHandle;

pub mod metrics;
pub mod sig;

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    no_bots: Option<MyRegex>,
    no_merges: bool,
) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
    let mailmap = repo.open_mailmap();
    let is_traversal_complete = Arc::new(AtomicBool::default());

    let commit_graph = repo.commit_graph().ok();
    let can_use_commit_graph = commit_graph.is_some();

    let commit_iter = repo
        .head_commit()?
        .id()
        .ancestors()
        .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
        .use_commit_graph(can_use_commit_graph)
        .with_commit_graph(commit_graph)
        .all()?;

    // Best-effort churn computation: compute churn concurrently with the traversal.
    // Stop when traversal completes; churn results are therefore non-deterministic.
    let (churn_thread, churn_tx) =
        get_churn_channel(repo, &mailmap, no_bots.clone(), &is_traversal_complete);

    for commit in commit_iter {
        let commit = commit?;
        {
            if no_merges && commit.parent_ids.len() > 1 {
                continue;
            }

            update_signature_counts(
                &commit.object()?,
                &mailmap,
                no_bots.as_ref(),
                &mut number_of_commits_by_signature,
            )?;

            churn_tx.send(commit.id)?;

            let commit_time = gix::date::Time::new(
                commit
                    .commit_time
                    .expect("sorting by time yields this field as part of traversal"),
                0,
            );
            time_of_most_recent_commit.get_or_insert(commit_time);
            time_of_first_commit = commit_time.into();
        }
    }

    is_traversal_complete.store(true, Ordering::SeqCst);

    drop(churn_tx);

    let (number_of_commits_by_file_path, churn_pool_size) =
        churn_thread.join().expect("never panics")?;

    let git_metrics = GitMetrics::new(
        number_of_commits_by_signature,
        number_of_commits_by_file_path,
        churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
    );

    Ok(git_metrics)
}

type NumberOfCommitsByFilepath = HashMap<BString, usize>;
type ChurnPair = (NumberOfCommitsByFilepath, usize);

fn get_churn_channel(
    repo: &gix::Repository,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex_pattern: Option<MyRegex>,
    is_traversal_complete: &Arc<AtomicBool>,
) -> (JoinHandle<Result<ChurnPair>>, Sender<ObjectId>) {
    let (tx, rx) = channel::<gix::hash::ObjectId>();
    let thread = std::thread::spawn({
        let repo = repo.clone();
        let mailmap = mailmap.clone();
        let bot_regex_pattern = bot_regex_pattern.clone();
        let is_traversal_complete = is_traversal_complete.clone();
        move || -> Result<_> {
            let mut number_of_commits_by_file_path = NumberOfCommitsByFilepath::new();
            let mut diffs_computed = 0;
            while let Ok(commit_id) = rx.recv() {
                let commit = repo.find_object(commit_id)?.into_commit();
                if is_bot_commit(&commit, &mailmap, bot_regex_pattern.as_ref())? {
                    continue;
                }
                compute_diff_with_parent(&mut number_of_commits_by_file_path, &commit, &repo)?;
                diffs_computed += 1;
                if is_traversal_complete.load(Ordering::Relaxed) {
                    break;
                }
            }

            Ok((number_of_commits_by_file_path, diffs_computed))
        }
    });

    (thread, tx)
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

    if let (parent_tree_id, None) = parents {
        let old_tree = parent_tree_id.object()?.into_tree();
        let new_tree = commit.tree()?;
        let changes =
            repo.diff_tree_to_tree(&old_tree, &new_tree, Options::default().with_rewrites(None))?;
        for change in &changes {
            let is_file_change = match change {
                Change::Addition { entry_mode, .. } | Change::Modification { entry_mode, .. } => {
                    entry_mode.is_blob()
                }
                Change::Deletion { .. } | Change::Rewrite { .. } => false,
            };
            if is_file_change {
                let path = change.location();
                *change_map.entry(path.to_owned()).or_insert(0) += 1;
            }
        }
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
    bot_regex_pattern.is_some_and(|regex| regex.0.is_match(author_name.to_str_lossy().as_ref()))
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
}
