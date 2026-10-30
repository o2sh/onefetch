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
    co_authors: bool,
) -> Result<GitMetrics> {
    let mut number_of_counted_commits = 0;
    let mut time_of_most_recent_commit = None;
    let mut time_of_first_commit = None;
    let mut number_of_commits_by_identity: HashMap<Identity, usize> = HashMap::new();
    let mut commit_times = Vec::new();
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

            let commit_time = gix::date::Time::new(
                commit
                    .commit_time
                    .expect("sorting by time yields this field as part of traversal"),
                0,
            );
            // Commits that credit nobody (e.g. bots left out by `--no-bots`)
            // count neither as commits, nor in the activity, nor in the churn.
            let identities =
                credited_identities(&commit.object()?, &mailmap, no_bots.as_ref(), co_authors)?;
            if !identities.is_empty() {
                update_identity_counts(identities, &mut number_of_commits_by_identity);
                update_commit_times(commit_time.seconds, &mut commit_times);
                churn_tx.send(commit.id)?;
                number_of_counted_commits += 1;
            }
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
        number_of_counted_commits,
        number_of_commits_by_file_path,
        churn_pool_size,
        commit_times,
        time_of_first_commit,
        time_of_most_recent_commit,
    );

    Ok(git_metrics)
}

fn update_identity_counts(
    identities: Vec<Identity>,
    number_of_commits_by_identity: &mut HashMap<Identity, usize>,
) {
    for identity in identities {
        *number_of_commits_by_identity.entry(identity).or_insert(0) += 1;
    }
}

fn update_commit_times(
    commit_time: gix::date::SecondsSinceUnixEpoch,
    commit_times: &mut Vec<gix::date::SecondsSinceUnixEpoch>,
) {
    commit_times.push(commit_time);
}

/// The identities a commit counts for: its author and, if `co_authors`,
/// everyone in its `Co-authored-by` trailers, each resolved through the mailmap
/// and credited once. Bots left out by `--no-bots` are dropped.
fn credited_identities(
    commit: &gix::Commit<'_>,
    mailmap: &gix::mailmap::Snapshot,
    no_bots: Option<&BotRegex>,
    co_authors: bool,
) -> Result<Vec<Identity>> {
    let mut signatures = vec![mailmap.resolve(commit.author()?)];
    if co_authors
        && let Some(body) =
            gix::objs::commit::MessageRef::from_bytes(commit.message_raw_sloppy()).body()
    {
        signatures.extend(body.trailers().co_authored_by().filter_map(|trailer| {
            let co_author = gix::actor::IdentityRef::from_bytes(&trailer.value)
                .ok()?
                .trim();
            Some(mailmap.resolve(gix::actor::SignatureRef {
                name: co_author.name,
                email: co_author.email,
                time: "",
            }))
        }));
    }

    let mut identities: Vec<Identity> = Vec::new();
    for signature in signatures {
        let identity = Identity::from(signature);
        if !is_bot(&identity.name, no_bots) && !identities.contains(&identity) {
            identities.push(identity);
        }
    }
    Ok(identities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::git::bots::NO_BOTS_DEFAULT_REGEX_PATTERN;
    use rstest::rstest;
    use std::str::FromStr;

    fn identity(name: &str, email: &str) -> Identity {
        Identity {
            name: name.into(),
            email: email.into(),
        }
    }

    /// Credit a commit by `author` with `message`, written to a throwaway repository.
    /// The commit is signed, with a blank line inside its signature header,
    /// so the message must be told apart from the headers correctly.
    fn credit(
        author: (&str, &str),
        message: &str,
        mailmap: &str,
        no_bots: Option<&BotRegex>,
        co_authors: bool,
    ) -> Vec<Identity> {
        let dir = gix_testtools::tempfile::tempdir().unwrap();
        let repo = gix::init_bare(dir.path()).unwrap();
        let signature = gix::actor::Signature {
            name: author.0.into(),
            email: author.1.into(),
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
        credited_identities(
            &repo.find_commit(id).unwrap(),
            &gix::mailmap::Snapshot::from_bytes(mailmap.as_bytes()),
            no_bots,
            co_authors,
        )
        .unwrap()
    }

    fn credited(message: &str, mailmap: &str, no_bots: bool, co_authors: bool) -> Vec<Identity> {
        let no_bots = no_bots.then(|| BotRegex::from_str(NO_BOTS_DEFAULT_REGEX_PATTERN).unwrap());
        credit(
            ("Author", "author@example.org"),
            message,
            mailmap,
            no_bots.as_ref(),
            co_authors,
        )
    }

    #[test]
    fn test_credited_identities_without_trailers() {
        assert_eq!(
            credited("subject\n\nbody\n", "", false, true),
            vec![identity("Author", "author@example.org")]
        );
    }

    #[rstest]
    #[case("Co-authored-by")]
    #[case("co-authored-by")]
    #[case("CO-AUTHORED-BY")]
    #[case("Co-Authored-By")]
    fn test_credited_identities_with_co_authors(#[case] token: &str) {
        let message =
            format!("subject\n\n{token}: One <one@example.org>\n{token}: Two <two@example.org>\n");
        assert_eq!(
            credited(&message, "", false, true),
            vec![
                identity("Author", "author@example.org"),
                identity("One", "one@example.org"),
                identity("Two", "two@example.org"),
            ]
        );
    }

    #[test]
    fn test_credited_identities_ignores_co_authors_by_default() {
        assert_eq!(
            credited(
                "subject\n\nCo-authored-by: One <one@example.org>\n",
                "",
                false,
                false
            ),
            vec![identity("Author", "author@example.org")]
        );
    }

    #[test]
    fn test_credited_identities_ignores_other_trailers() {
        assert_eq!(
            credited(
                "subject\n\nSigned-off-by: One <one@example.org>\nReviewed-by: Two <two@example.org>\n",
                "",
                false,
                true
            ),
            vec![identity("Author", "author@example.org")]
        );
    }

    #[test]
    fn test_credited_identities_credits_each_identity_once() {
        assert_eq!(
            credited(
                "subject\n\nCo-authored-by: Author <author@example.org>\nCo-authored-by: One <one@example.org>\nCo-authored-by: One <one@example.org>\n",
                "",
                false,
                true
            ),
            vec![
                identity("Author", "author@example.org"),
                identity("One", "one@example.org"),
            ]
        );
    }

    #[test]
    fn test_credited_identities_resolves_co_authors_through_mailmap() {
        assert_eq!(
            credited(
                "subject\n\nCo-authored-by: Old Name <old@example.org>\nCo-authored-by: Alias <author-alias@example.org>\n",
                "One <one@example.org> <old@example.org>\nAuthor <author@example.org> <author-alias@example.org>\n",
                false,
                true
            ),
            vec![
                identity("Author", "author@example.org"),
                identity("One", "one@example.org"),
            ]
        );
    }

    #[test]
    fn test_credited_identities_skips_malformed_co_authors() {
        assert_eq!(
            credited(
                "subject\n\nCo-authored-by: just a name\nCo-authored-by: One <one@example.org>\n",
                "",
                false,
                true
            ),
            vec![
                identity("Author", "author@example.org"),
                identity("One", "one@example.org"),
            ]
        );
    }

    #[test]
    fn test_credited_identities_with_no_bots_drops_bot_co_authors() {
        assert_eq!(
            credited(
                "subject\n\nCo-authored-by: dependabot[bot] <bot@example.org>\nCo-authored-by: One <one@example.org>\n",
                "",
                true,
                true
            ),
            vec![
                identity("Author", "author@example.org"),
                identity("One", "one@example.org"),
            ]
        );
    }

    #[test]
    fn test_credited_identities_with_no_bots_keeps_human_co_authors_of_bot_commits() {
        let no_bots = BotRegex::from_str(NO_BOTS_DEFAULT_REGEX_PATTERN).unwrap();
        let credit = |co_authors| {
            credit(
                ("Copilot[bot]", "copilot@example.org"),
                "subject\n\nCo-authored-by: One <one@example.org>\n",
                "",
                Some(&no_bots),
                co_authors,
            )
        };
        assert_eq!(credit(true), vec![identity("One", "one@example.org")]);
        assert_eq!(credit(false), vec![]);
    }
}
