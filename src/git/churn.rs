use super::bots::{BotRegex, is_bot};
use anyhow::Result;
use gix::bstr::BString;
use gix::diff::Options;
use gix::diff::tree_with_rewrites::Change;
use gix::prelude::ObjectIdExt;
use gix::{Commit, ObjectId};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Sender, channel};
use std::thread::JoinHandle;

type NumberOfCommitsByFilepath = HashMap<BString, usize>;
type ChurnPair = (NumberOfCommitsByFilepath, usize);

pub(super) fn get_churn_channel(
    repo: &gix::Repository,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex: Option<BotRegex>,
    is_traversal_complete: &Arc<AtomicBool>,
    total_number_of_commits: &Arc<AtomicUsize>,
    churn_pool_size: Option<usize>,
) -> (JoinHandle<Result<ChurnPair>>, Sender<ObjectId>) {
    let (tx, rx) = channel::<gix::hash::ObjectId>();
    let thread = std::thread::spawn({
        let repo = repo.clone();
        let mailmap = mailmap.clone();
        let is_traversal_complete = is_traversal_complete.clone();
        let total_number_of_commits = total_number_of_commits.clone();
        move || -> Result<_> {
            let mut number_of_commits_by_file_path = NumberOfCommitsByFilepath::new();
            let mut diffs_computed = 0;
            while let Ok(commit_id) = rx.recv() {
                let commit = repo.find_object(commit_id)?.into_commit();
                if is_bot_commit(&commit, &mailmap, bot_regex.as_ref())? {
                    continue;
                }
                if compute_diff_with_parent(&mut number_of_commits_by_file_path, &commit, &repo)? {
                    diffs_computed += 1;
                }
                if should_break(
                    is_traversal_complete.load(Ordering::Relaxed),
                    total_number_of_commits.load(Ordering::Relaxed),
                    churn_pool_size,
                    diffs_computed,
                ) {
                    break;
                }
            }

            Ok((number_of_commits_by_file_path, diffs_computed))
        }
    });

    (thread, tx)
}

fn should_break(
    is_traversal_complete: bool,
    total_number_of_commits: usize,
    churn_pool_size_opt: Option<usize>,
    diffs_computed: usize,
) -> bool {
    if !is_traversal_complete {
        return false;
    }

    churn_pool_size_opt.is_none_or(|churn_pool_size| {
        diffs_computed >= churn_pool_size.min(total_number_of_commits)
    })
}

fn compute_diff_with_parent(
    change_map: &mut HashMap<BString, usize>,
    commit: &Commit,
    repo: &gix::Repository,
) -> Result<bool> {
    let mut parents = commit.parent_ids();
    let parents = (
        parents
            .next()
            .and_then(|parent_id| parent_id.object().ok()?.into_commit().tree_id().ok())
            .unwrap_or_else(|| gix::hash::ObjectId::empty_tree(repo.object_hash()).attach(repo)),
        parents.next(),
    );

    if let (parent_tree_id, None) = parents {
        let Some(old_tree) = parent_tree_id.try_object()?.map(|tree| tree.into_tree()) else {
            return Ok(false);
        };
        let Some(new_tree) = commit.tree_id()?.try_object()?.map(|tree| tree.into_tree()) else {
            return Ok(false);
        };
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

    Ok(true)
}

fn is_bot_commit(
    commit: &Commit,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex: Option<&BotRegex>,
) -> Result<bool> {
    if bot_regex.is_some() {
        let sig = mailmap.resolve(commit.author()?);
        Ok(is_bot(&sig.name, bot_regex))
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(false, 10, Some(5), 5, false)]
    #[case(true, 10, Some(5), 5, true)]
    #[case(true, 10, Some(8), 5, false)]
    #[case(true, 10, Some(20), 10, true)]
    #[case(true, 10, None, 5, true)]
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
