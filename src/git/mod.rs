use anyhow::{Context, Result};
use gix::Repository;
use gix::bstr::ByteSlice;
use std::path::PathBuf;

pub mod bots;
mod churn;
pub(crate) mod identity;
pub(crate) mod metrics;
mod traversal;

pub(crate) use traversal::traverse_commit_graph;

pub(crate) fn uses_reftables(repo: &Repository) -> bool {
    repo.config_snapshot()
        .string("extensions.refstorage")
        .is_some_and(|kind| kind.as_bstr() == "reftable")
}

pub fn get_work_dir(repo: &Repository) -> Result<PathBuf> {
    Ok(repo
        .workdir()
        .context("please run onefetch inside of a non-bare git repository")?
        .to_owned())
}

pub(crate) fn get_git_username(repo: &Repository) -> String {
    repo.committer()
        .and_then(Result::ok)
        .map(|c| c.name.to_string())
        .unwrap_or_default()
}

pub(crate) fn get_git_version() -> String {
    let version = std::process::Command::new("git").arg("--version").output();

    match version {
        Ok(v) => String::from_utf8_lossy(&v.stdout).replace('\n', ""),
        Err(_) => String::new(),
    }
}
