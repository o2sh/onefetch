use anyhow::{Context, Result};
use gix::Repository;
use gix::bstr::ByteSlice;
use std::path::PathBuf;

pub mod bots;
mod churn;
pub(crate) mod identity;
pub(crate) mod metrics;
mod remote;
mod tags;
mod traversal;

pub(crate) use remote::get_repo_url;
pub(crate) use tags::{LatestTag, get_latest_tag};
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
