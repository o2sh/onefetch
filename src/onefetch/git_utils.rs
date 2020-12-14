use crate::onefetch::{error::*, utils};
use git2::{Commit, Repository};

pub struct GitClient<'a> {
    repo: &'a Repository,
    commit_history: Vec<Commit<'a>>,
}

impl<'a> GitClient<'a> {
    pub fn new(repo: &Repository, no_merges: bool) -> Result<GitClient> {
        let commit_history = GitClient::get_git_history(repo, no_merges)?;
        Ok(GitClient { repo: repo, commit_history: commit_history })
    }

    pub fn get_creation_date(&self) -> Result<String> {
        let first_commit = self.commit_history.last();
        let output = match first_commit {
            Some(commit) => {
                let time = commit.time();
                utils::git_time_to_human_time(&time)
            }
            None => "".into(),
        };

        Ok(output)
    }

    pub fn get_number_of_commits(&self) -> String {
        let number_of_commits = self.commit_history.len();
        number_of_commits.to_string()
    }

    pub fn get_authors(&self, n: usize) -> Vec<(String, usize, usize)> {
        let mut authors = std::collections::HashMap::new();
        let mut author_name_by_email = std::collections::HashMap::new();
        let mut total_commits = 0;
        for commit in &self.commit_history {
            let author = commit.author();
            let author_name = String::from_utf8_lossy(author.name_bytes()).into_owned();
            let author_email = String::from_utf8_lossy(author.email_bytes()).into_owned();

            let commit_count = authors.entry(author_email.to_string()).or_insert(0);
            author_name_by_email.entry(author_email.to_string()).or_insert(author_name);
            *commit_count += 1;
            total_commits += 1;
        }

        let mut authors: Vec<(String, usize)> = authors.into_iter().collect();
        authors.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

        authors.truncate(n);

        let authors: Vec<(String, usize, usize)> = authors
            .into_iter()
            .map(|(author, count)| {
                (
                    author_name_by_email.get(&author).unwrap().trim_matches('\'').to_string(),
                    count,
                    count * 100 / total_commits,
                )
            })
            .collect();

        authors
    }

    pub fn get_date_of_last_commit(&self) -> String {
        let last_commit = self.commit_history.first();

        let output = match last_commit {
            Some(commit) => utils::git_time_to_human_time(&commit.time()),
            None => "".into(),
        };
        output
    }

    // This collects the repo size excluding .git
    pub fn get_repo_size(&self) -> (String, u64) {
        let (repo_size, file_count) = match self.repo.index() {
            Ok(index) => index.iter().fold(
                (0, 0),
                |(repo_size, file_count): (u128, u64), index_entry| -> (u128, u64) {
                    (repo_size + index_entry.file_size as u128, file_count + 1)
                },
            ),
            Err(_) => (0, 0),
        };

        (utils::bytes_to_human_readable(repo_size), file_count)
    }

    fn get_git_history(repo: &'a Repository, no_merges: bool) -> Result<Vec<Commit<'a>>> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        let commits: Vec<Commit<'a>> = revwalk
            .filter_map(|r| match r {
                Err(_) => None,
                Ok(r) => {
                    let commit = repo.find_commit(r).expect("Could not find commit");
                    if no_merges {
                        let parents = commit.parents().len();
                        if parents > 1 {
                            return None;
                        }
                    }
                    Some(commit.to_owned())
                }
            })
            .collect();

        Ok(commits)
    }
}

// Should be moved to fmt::Display of Info
pub fn get_packed_size(repo_size: String, files_count: u64) -> Result<String> {
    match files_count {
        0 => Ok(repo_size),
        _ => {
            let res = format!("{} ({} files)", repo_size, files_count.to_string());
            Ok(res)
        }
    }
}
