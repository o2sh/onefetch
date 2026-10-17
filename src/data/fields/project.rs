use crate::data::manifest::Manifest;
use anyhow::Result;
use gix::{Repository, bstr::ByteSlice};
use serde::Serialize;
use std::ffi::OsStr;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub repo_name: String,
    pub number_of_branches: usize,
    pub number_of_tags: usize,
}

impl Project {
    pub fn new(repo: &Repository, repo_url: &str, manifest: Option<&Manifest>) -> Result<Self> {
        let repo_name = get_repo_name(repo_url, manifest)?;
        let number_of_branches = get_number_of_branches(repo)?;
        let number_of_tags = get_number_of_tags(repo)?;
        Ok(Self {
            repo_name,
            number_of_branches,
            number_of_tags,
        })
    }
}

fn get_repo_name(repo_url: &str, manifest: Option<&Manifest>) -> Result<String> {
    if repo_url.is_empty() {
        return Ok(String::default());
    }
    let url = gix::url::parse(repo_url)?;
    let path = gix::path::from_bstr(url.path.as_bstr())?;
    let repo_name = path
        .with_extension("")
        .file_name()
        .map(OsStr::to_string_lossy)
        .map(std::borrow::Cow::into_owned)
        .unwrap_or_default();

    if repo_name.is_empty() {
        let repo_name_from_manifest = manifest.and_then(|m| m.name.clone()).unwrap_or_default();
        Ok(repo_name_from_manifest)
    } else {
        Ok(repo_name)
    }
}

fn get_number_of_tags(repo: &Repository) -> Result<usize> {
    Ok(repo.references()?.tags()?.count())
}

fn get_number_of_branches(repo: &Repository) -> Result<usize> {
    let mut number_of_branches = repo.references()?.remote_branches()?.count();
    number_of_branches = number_of_branches.saturating_sub(1); //Exclude origin/HEAD -> origin/main
    Ok(number_of_branches)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_repo_name_when_no_remote() -> Result<()> {
        let repo_name = get_repo_name("", None)?;
        assert!(repo_name.is_empty());

        Ok(())
    }
}
