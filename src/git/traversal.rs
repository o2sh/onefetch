use super::bots::{BotRegex, is_bot};
use super::churn::get_churn_channel;
use super::identity::Identity;
use super::metrics::GitMetrics;
use anyhow::Result;
use gix::revision::walk::Sorting;
use gix::traverse::commit::simple::CommitTimeOrder;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    no_bots: Option<BotRegex>,
    churn_pool_size: Option<usize>,
    no_merges: bool,
) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_identity: HashMap<Identity, usize> = HashMap::new();
    let mailmap = repo.open_mailmap();
    let is_traversal_complete = Arc::new(AtomicBool::default());
    let total_number_of_commits = Arc::new(AtomicUsize::default());

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

    // Best-effort strategy for Churn computation: keep computing churn while traversal runs;
    // it stops once traversal is done and churn_pool_size is reached (if provided).
    let (churn_thread, churn_tx) = get_churn_channel(
        repo,
        &mailmap,
        no_bots.clone(),
        &is_traversal_complete,
        &total_number_of_commits,
        churn_pool_size,
    );

    let mut count = 0;
    for commit in commit_iter {
        let commit = commit?;
        {
            if no_merges && commit.parent_ids.len() > 1 {
                continue;
            }

            update_identity_counts(
                &commit.object()?,
                &mailmap,
                no_bots.as_ref(),
                &mut number_of_commits_by_identity,
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

            count += 1;
        }
    }

    total_number_of_commits.store(count, Ordering::SeqCst);
    is_traversal_complete.store(true, Ordering::SeqCst);

    drop(churn_tx);

    let (number_of_commits_by_file_path, churn_pool_size) =
        churn_thread.join().expect("never panics")?;

    let git_metrics = GitMetrics::new(
        number_of_commits_by_identity,
        number_of_commits_by_file_path,
        churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
    );

    Ok(git_metrics)
}

fn update_identity_counts(
    commit: &gix::Commit,
    mailmap: &gix::mailmap::Snapshot,
    bot_regex: Option<&BotRegex>,
    number_of_commits_by_identity: &mut HashMap<Identity, usize>,
) -> Result<()> {
    let sig = mailmap.resolve(commit.author()?);
    if !is_bot(&sig.name, bot_regex) {
        *number_of_commits_by_identity.entry(sig.into()).or_insert(0) += 1;
    }
    Ok(())
}
