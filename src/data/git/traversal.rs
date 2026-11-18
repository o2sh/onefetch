use super::churn::get_churn_channel;
use super::identity::Identity;
use super::metrics::GitMetrics;
use anyhow::Result;
use gix::revision::walk::Sorting;
use gix::traverse::commit::simple::CommitTimeOrder;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub fn traverse_commit_graph(
    repo: &gix::Repository,
    ignore_author: Option<&Regex>,
    churn_pool_size: Option<usize>,
    no_merges: bool,
) -> Result<GitMetrics> {
    let mut number_of_counted_commits = 0;
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_identity: HashMap<Identity, usize> = HashMap::new();
    let mut commit_times = Vec::new();
    let mailmap = repo.open_mailmap();
    let is_traversal_complete = Arc::new(AtomicBool::default());
    let total_number_of_counted_commits = Arc::new(AtomicUsize::default());

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
    let (churn_thread, churn_sender) = get_churn_channel(
        repo,
        &is_traversal_complete,
        &total_number_of_counted_commits,
        churn_pool_size,
    );

    for commit in commit_iter {
        let commit = commit?;
        if no_merges && commit.parent_ids.len() > 1 {
            continue;
        }

        // Commits that credit nobody (e.g. authors left out by `--ignore-author`)
        // count neither as commits, nor in the activity, nor in the churn,
        // nor in the creation and last change dates.
        let identities = credited_identities(&commit.object()?, &mailmap, ignore_author)?;
        if identities.is_empty() {
            continue;
        }
        for identity in identities {
            *number_of_commits_by_identity.entry(identity).or_default() += 1;
        }

        let commit_time = gix::date::Time::new(
            commit
                .commit_time
                .expect("sorting by time yields this field as part of traversal"),
            0,
        );
        time_of_most_recent_commit.get_or_insert(commit_time);
        time_of_first_commit = Some(commit_time);
        commit_times.push(commit_time.seconds);
        churn_sender.send(commit.id)?;
        number_of_counted_commits += 1;
    }

    total_number_of_counted_commits.store(number_of_counted_commits, Ordering::SeqCst);
    is_traversal_complete.store(true, Ordering::SeqCst);

    drop(churn_sender);

    let (number_of_commits_by_file_path, churn_pool_size) =
        churn_thread.join().expect("never panics")?;

    let git_metrics = GitMetrics::new(
        number_of_commits_by_identity,
        number_of_counted_commits,
        number_of_commits_by_file_path,
        churn_pool_size,
        commit_times,
        time_of_first_commit,
        time_of_most_recent_commit,
    );

    Ok(git_metrics)
}

/// The identities a commit counts for: its author and everyone in its
/// `Co-authored-by` trailers, each resolved through the mailmap and credited
/// once. Authors left out by `--ignore-author` are dropped.
fn credited_identities(
    commit: &gix::Commit<'_>,
    mailmap: &gix::mailmap::Snapshot,
    ignore_author: Option<&Regex>,
) -> Result<Vec<Identity>> {
    let author = Identity::resolve(commit.author()?.actor(), mailmap);
    let message = gix::objs::commit::MessageRef::from_bytes(commit.message_raw_sloppy());
    let co_authors = message
        .body()
        .into_iter()
        .flat_map(|body| body.trailers().co_authored_by())
        .filter_map(|trailer| {
            gix::actor::IdentityRef::from_bytes(&trailer.value)
                .ok()
                .map(|co_author| Identity::resolve(co_author, mailmap))
        });

    let mut identities = Vec::new();
    for identity in std::iter::once(author).chain(co_authors) {
        if !identity.is_ignored(ignore_author) && !identities.contains(&identity) {
            identities.push(identity);
        }
    }
    Ok(identities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const AUTHOR: &str = "Author <author@example.org>";

    /// The identities credited for a commit by `author` with `message`, written
    /// to a throwaway repository, as `Name <email>`. The commit is signed, with a
    /// blank line inside its signature header, so the message must be told
    /// apart from the headers correctly.
    fn credited(
        author: &str,
        message: &str,
        mailmap: &str,
        ignore_author: Option<&str>,
    ) -> Vec<String> {
        let dir = gix_testtools::tempfile::tempdir().unwrap();
        let repo = gix::init_bare(dir.path()).unwrap();
        let author = gix::actor::IdentityRef::from_bytes(author.as_bytes()).unwrap();
        let signature = gix::actor::Signature {
            name: author.name.into(),
            email: author.email.into(),
            time: gix::date::Time::default(),
        };
        let id = repo
            .write_object(gix::objs::Commit {
                tree: repo.empty_tree().id,
                parents: Default::default(),
                author: signature.clone(),
                committer: signature,
                encoding: None,
                message: message.into(),
                extra_headers: vec![(
                    "gpgsig".into(),
                    "-----BEGIN PGP SIGNATURE-----\n\nsignature\n-----END PGP SIGNATURE-----\n"
                        .into(),
                )],
            })
            .unwrap();
        let ignore_author = ignore_author.map(|pattern| Regex::new(pattern).unwrap());
        credited_identities(
            &repo.find_commit(id).unwrap(),
            &gix::mailmap::Snapshot::from_bytes(mailmap.as_bytes()),
            ignore_author.as_ref(),
        )
        .unwrap()
        .into_iter()
        .map(|identity| format!("{} <{}>", identity.name, identity.email))
        .collect()
    }

    #[rstest]
    #[case::without_trailers("subject\n\nbody\n", "", None, &[AUTHOR])]
    #[case::other_trailers(
        "subject\n\nSigned-off-by: One <one@example.org>\nReviewed-by: Two <two@example.org>\n",
        "",
        None,
        &[AUTHOR],
    )]
    #[case::each_identity_once(
        "subject\n\nCo-authored-by: Author <author@example.org>\nCo-authored-by: One <one@example.org>\nCo-authored-by: One <one@example.org>\n",
        "",
        None,
        &[AUTHOR, "One <one@example.org>"],
    )]
    #[case::through_mailmap(
        "subject\n\nCo-authored-by: Old Name <old@example.org>\nCo-authored-by: Alias <author-alias@example.org>\n",
        "One <one@example.org> <old@example.org>\nAuthor <author@example.org> <author-alias@example.org>\n",
        None,
        &[AUTHOR, "One <one@example.org>"],
    )]
    #[case::malformed_co_author(
        "subject\n\nCo-authored-by: just a name\nCo-authored-by: One <one@example.org>\n",
        "",
        None,
        &[AUTHOR, "One <one@example.org>"],
    )]
    #[case::ignored_co_author(
        "subject\n\nCo-authored-by: dependabot[bot] <bot@example.org>\nCo-authored-by: One <one@example.org>\n",
        "",
        Some(r"\[bot\]"),
        &[AUTHOR, "One <one@example.org>"],
    )]
    fn test_credited_identities(
        #[case] message: &str,
        #[case] mailmap: &str,
        #[case] ignore_author: Option<&str>,
        #[case] expected: &[&str],
    ) {
        assert_eq!(credited(AUTHOR, message, mailmap, ignore_author), expected);
    }

    #[rstest]
    #[case("Co-authored-by")]
    #[case("co-authored-by")]
    #[case("CO-AUTHORED-BY")]
    #[case("Co-Authored-By")]
    fn test_credited_identities_trailer_token_is_case_insensitive(#[case] token: &str) {
        let message =
            format!("subject\n\n{token}: One <one@example.org>\n{token}: Two <two@example.org>\n");
        assert_eq!(
            credited(AUTHOR, &message, "", None),
            [AUTHOR, "One <one@example.org>", "Two <two@example.org>"]
        );
    }

    #[test]
    fn test_credited_identities_keeps_co_authors_of_ignored_author() {
        assert_eq!(
            credited(
                "Copilot[bot] <copilot@example.org>",
                "subject\n\nCo-authored-by: One <one@example.org>\n",
                "",
                Some(r"\[bot\]"),
            ),
            ["One <one@example.org>"]
        );
    }
}
