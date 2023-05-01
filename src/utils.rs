use anyhow::Result;
use gix::{open, Repository, ThreadSafeRepository};

pub fn repo(name: &str) -> Result<Repository> {
    let name = name.to_string();
    let repo_path = gix_testtools::scripted_fixture_read_only(name).unwrap();
    let safe_repo = ThreadSafeRepository::open_opts(repo_path, open::Options::isolated())?;
    Ok(safe_repo.to_thread_local())
}
