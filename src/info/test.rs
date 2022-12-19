#[cfg(test)]
pub mod utils {
    use anyhow::Result;
    use git_repository::{open, Repository, ThreadSafeRepository};

    pub fn repo(name: &str) -> Result<Repository> {
        let name = name.to_string();
        let repo_path = git_testtools::scripted_fixture_repo_read_only(name).unwrap();
        let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
        Ok(safe_repo.to_thread_local())
    }
}
