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
use gix::Commit;
use std::cmp;
use std::collections::HashMap;

pub mod metrics;
pub mod sig;

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    no_bots: Option<MyRegex>,
    min_churn_pool_size: Option<usize>,
    no_merges: bool,
) -> Result<GitMetrics> {
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mailmap = repo.open_mailmap();
    let commit_iter = commit_iter(repo)?;
    let mut number_of_commits_by_signature: HashMap<Sig, usize> = HashMap::new();
    let mut number_of_commits_by_file_path: HashMap<BString, usize> = HashMap::new();
    let mut diffs_computed = 0;

    for commit in commit_iter {
        let commit = commit?;
        if no_merges && commit.parent_ids.len() > 1 {
            continue;
        }

        let commit_obj = commit.object()?;
        update_signature_counts(
            &commit_obj,
            &mailmap,
            no_bots.as_ref(),
            &mut number_of_commits_by_signature,
        )?;

        if should_compute_churn(min_churn_pool_size, diffs_computed)
            && !is_bot_commit(&commit_obj, &mailmap, no_bots.as_ref())?
        {
            compute_diff_with_parent(&mut number_of_commits_by_file_path, &commit_obj, repo)?;
            diffs_computed += 1;
        }

        update_commit_times(
            &mut time_of_most_recent_commit,
            &mut time_of_first_commit,
            commit.commit_time,
        );
    }

    let churn_pool_size = compute_churn_pool_size(min_churn_pool_size, diffs_computed);

    let git_metrics = GitMetrics::new(
        number_of_commits_by_signature,
        number_of_commits_by_file_path,
        churn_pool_size,
        time_of_first_commit,
        time_of_most_recent_commit,
    );

    Ok(git_metrics)
}

fn commit_iter(
    repo: &gix::Repository,
) -> Result<
    impl Iterator<Item = Result<gix::revision::walk::Info<'_>, gix::revision::walk::iter::Error>>,
>
{
    Ok(repo
        .head_commit()?
        .id()
        .ancestors()
        .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
        .use_commit_graph(false)
        .all()?)
}

fn should_compute_churn(min_churn_pool_size: Option<usize>, diffs_computed: usize) -> bool {
    min_churn_pool_size.is_none_or(|limit| diffs_computed < limit)
}

fn compute_churn_pool_size(min_churn_pool_size: Option<usize>, diffs_computed: usize) -> usize {
    min_churn_pool_size.map_or(diffs_computed, |limit| cmp::min(limit, diffs_computed))
}

fn update_commit_times(
    time_of_most_recent_commit: &mut Option<gix::date::Time>,
    time_of_first_commit: &mut Option<gix::date::Time>,
    commit_time_opt: Option<gix::date::SecondsSinceUnixEpoch>,
) {
    let commit_time = gix::date::Time::new(
        commit_time_opt.expect("sorting by time yields this field as part of traversal"),
        0,
    );
    time_of_most_recent_commit.get_or_insert(commit_time);
    *time_of_first_commit = commit_time.into();
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

    #[rstest]
    #[case(None, 0, true)]
    #[case(None, 5, true)]
    #[case(Some(3), 0, true)]
    #[case(Some(3), 2, true)]
    #[case(Some(3), 3, false)]
    fn test_should_compute_churn(
        #[case] min_churn_pool_size: Option<usize>,
        #[case] diffs_computed: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(
            should_compute_churn(min_churn_pool_size, diffs_computed),
            expected
        );
    }

    #[rstest]
    #[case(None, 0, 0)]
    #[case(None, 5, 5)]
    #[case(Some(3), 0, 0)]
    #[case(Some(3), 2, 2)]
    #[case(Some(3), 3, 3)]
    #[case(Some(3), 10, 3)]
    fn test_compute_churn_pool_size(
        #[case] min_churn_pool_size: Option<usize>,
        #[case] diffs_computed: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(
            compute_churn_pool_size(min_churn_pool_size, diffs_computed),
            expected
        );
    }

}
